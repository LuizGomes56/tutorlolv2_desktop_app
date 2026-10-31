use recorder_task::{LatestSnapshot, RecorderHandle};
use shortcut_commands::{get_overlay_shortcut, set_overlay_shortcut, set_shortcut_recording};
use std::time::Duration;
use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, RunEvent, Size, State,
    WebviewUrl, WebviewWindowBuilder, ipc::InvokeResponseBody,
};

/// Shared encoding for persisted records and server requests.
pub const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

/// How long closing the app may take to save the recording in progress.
const EXIT_FLUSH: Duration = Duration::from_secs(2);

pub struct AppState {
    latest: LatestSnapshot,
    recorder: RecorderHandle,
}

pub mod delivery;
pub mod history;
pub mod lcu;
pub mod live_client;
mod overlay;
pub mod recorder;
mod recorder_task;
pub mod sharing;
mod shortcut;
mod shortcut_commands;
pub mod spool;
pub mod store;
pub mod uploader;

#[cfg(windows)]
mod wkb;

#[tauri::command]
fn blur_overlay(app: AppHandle) -> Result<(), String> {
    overlay::set_focused(&app, false)
}

#[tauri::command]
fn get_overlay_focus() -> bool {
    overlay::is_focused()
}

#[tauri::command]
fn toggle_overlay(app: AppHandle) -> Result<bool, String> {
    overlay::toggle(&app)
}

/// The latest snapshot from port 2999, polled once per second by the recorder.
#[tauri::command]
async fn get_live_game(state: State<'_, AppState>) -> Result<InvokeResponseBody, String> {
    state
        .latest
        .get()
        .map(|bytes| InvokeResponseBody::Raw(bytes.to_vec()))
        .ok_or_else(|| "No live game".to_owned())
}

#[tauri::command]
fn get_recording_state(state: State<'_, AppState>) -> tutorlolv2_protocol::RecordingState {
    state.recorder.shared.recording_state()
}

#[tauri::command]
async fn set_share_mode(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    mode: tutorlolv2_protocol::ShareMode,
) -> Result<tutorlolv2_protocol::ShareState, String> {
    use tauri::Emitter;

    sharing::authorize_share_window(window.label())?;

    let result = state.recorder.shared.set_share_mode(mode).await;
    let current = state.recorder.shared.recording_state();

    let _ = window
        .app_handle()
        .emit("recording-state-changed", &current);

    let _ = window
        .app_handle()
        .emit("share-state-changed", &current.share);

    result
}

fn history_auth(window: &tauri::WebviewWindow) -> Result<(), tutorlolv2_protocol::HistoryError> {
    history::authorize(window.label())
}

fn queue_history(shared: std::sync::Arc<delivery::Delivery>, app: AppHandle, repair: bool) {
    if !shared.history.begin(repair) {
        return;
    }

    tauri::async_runtime::spawn(async move {
        use tauri::Emitter;
        let api = shared.api.read().await.clone();
        shared.history.refresh(api, repair).await;
        let _ = app.emit("history-changed", ());
    });
}

#[tauri::command]
fn list_history(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    offset: u32,
    limit: u32,
) -> Result<tutorlolv2_protocol::HistoryList, tutorlolv2_protocol::HistoryError> {
    history_auth(&window)?;
    if !(1..=100).contains(&limit) {
        return Err(history::error(
            tutorlolv2_protocol::HistoryErrorKind::InvalidData,
            "History page size must be between 1 and 100",
            false,
        ));
    }
    let shared = state.recorder.shared.clone();
    let cached = shared.history.list(offset, limit);

    queue_history(shared, window.app_handle().clone(), false);

    Ok(cached)
}

#[tauri::command]
async fn get_history_details(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    recording_id: String,
) -> Result<tutorlolv2_protocol::HistoryDetails, tutorlolv2_protocol::HistoryError> {
    history_auth(&window)?;

    let id = history::parse_id(&recording_id)?;
    let shared = &state.recorder.shared;
    let api = shared.api.read().await.clone();

    shared
        .history
        .details(api.device_id, api.history_details(id).await?)
}

#[tauri::command]
async fn get_history_frames(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    recording_id: String,
    seq: u32,
) -> Result<InvokeResponseBody, tutorlolv2_protocol::HistoryError> {
    history_auth(&window)?;

    let id = history::parse_id(&recording_id)?;
    let api = state.recorder.shared.api.read().await.clone();
    let bytes = api.history_frames(id, seq).await?;

    state.recorder.shared.history.check_device(api.device_id)?;
    Ok(InvokeResponseBody::Raw(bytes))
}

#[tauri::command]
async fn set_history_share_mode(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    recording_id: String,
    mode: tutorlolv2_protocol::HistoryShareMode,
) -> Result<tutorlolv2_protocol::ShareState, tutorlolv2_protocol::HistoryError> {
    use tauri::Emitter;

    history_auth(&window)?;

    let id = history::parse_id(&recording_id)?;
    let shared = &state.recorder.shared;
    let share = shared.set_history_share_mode(id, mode).await?;

    let _ = window.app_handle().emit("history-changed", ());

    Ok(share)
}

#[tauri::command]
fn hide_history_entry(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    recording_id: String,
) -> Result<(), tutorlolv2_protocol::HistoryError> {
    use tauri::Emitter;

    history_auth(&window)?;

    state
        .recorder
        .shared
        .history
        .hide(history::parse_id(&recording_id)?)?;

    let _ = window.app_handle().emit("history-changed", ());
    Ok(())
}

#[tauri::command]
fn repair_history(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
) -> Result<(), tutorlolv2_protocol::HistoryError> {
    history_auth(&window)?;
    queue_history(
        state.recorder.shared.clone(),
        window.app_handle().clone(),
        true,
    );

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(overlay::OverlayState::default())
        .invoke_handler(tauri::generate_handler![
            list_history,
            get_history_details,
            get_history_frames,
            set_history_share_mode,
            hide_history_entry,
            repair_history,
            get_live_game,
            get_recording_state,
            set_share_mode,
            blur_overlay,
            get_overlay_focus,
            toggle_overlay,
            get_overlay_shortcut,
            set_overlay_shortcut,
            set_shortcut_recording
        ])
        .on_window_event(|window, event| {
            #[cfg(windows)]
            if matches!(
                event,
                tauri::WindowEvent::Focused(false) | tauri::WindowEvent::Destroyed
            ) {
                wkb::set_recording(window.label(), false);
            }

            #[cfg(not(windows))]
            let _ = (window, event);
        })
        .setup(|app| {
            // No webview may invoke commands before its managed recorder state exists.
            // The configured main window has create=false and is built only after manage.
            let latest = LatestSnapshot::default();
            let recorder = recorder_task::spawn(app.handle(), latest.clone());
            app.manage(AppState { latest, recorder });

            // #[cfg(feature = "overlay")]
            {
                let overlay =
                    WebviewWindowBuilder::new(app, "overlay", WebviewUrl::App("/overlay".into()))
                        .shadow(false)
                        .decorations(false)
                        .always_on_top(true)
                        .resizable(false)
                        .closable(false)
                        .transparent(true)
                        .skip_taskbar(false)
                        .focused(false)
                        .build()
                        .unwrap();

                let monitor = app
                    .primary_monitor()
                    .unwrap()
                    .or_else(|| app.available_monitors().unwrap().into_iter().next())
                    .expect("No monitor available");

                let area = monitor.work_area();

                overlay
                    .set_position(Position::Physical(PhysicalPosition::new(
                        area.position.x,
                        area.position.y,
                    )))
                    .unwrap();

                overlay
                    .set_size(Size::Physical(PhysicalSize::new(
                        area.size.width,
                        area.size.height,
                    )))
                    .unwrap();

                overlay.set_ignore_cursor_events(true).unwrap();
                overlay.set_focusable(false).unwrap();

                #[cfg(debug_assertions)]
                overlay.open_devtools();
            }

            let shortcut_path = app.path().app_config_dir()?.join("overlay-shortcut.json");

            app.manage(shortcut_commands::ShortcutState::load(shortcut_path));

            #[cfg(windows)]
            wkb::install_low_level_shortcuts(app.handle())?;

            #[cfg(debug_assertions)]
            {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let config = app
                .config()
                .app
                .windows
                .iter()
                .find(|config| config.label == "main")
                .ok_or("Main window configuration missing")?;

            let _ = WebviewWindowBuilder::from_config(app, config)?.build()?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let RunEvent::ExitRequested { .. } = event {
                app.state::<AppState>().recorder.exit(EXIT_FLUSH);
            }
        });
}
