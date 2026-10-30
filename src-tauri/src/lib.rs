use recorder_task::{LatestSnapshot, RecorderHandle};
use shortcut_commands::{get_overlay_shortcut, set_overlay_shortcut, set_shortcut_recording};
use std::time::Duration;
use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, RunEvent, Size, State,
    WebviewUrl, WebviewWindowBuilder, ipc::InvokeResponseBody,
};

/// How long closing the app may take to save the recording in progress.
const EXIT_FLUSH: Duration = Duration::from_secs(2);

pub struct AppState {
    latest: LatestSnapshot,
    recorder: RecorderHandle,
}

pub mod delivery;
pub mod lcu;
pub mod live_client;
mod overlay;
pub mod recorder;
mod recorder_task;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(overlay::OverlayState::default())
        .invoke_handler(tauri::generate_handler![
            get_live_game,
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
            let window = app.get_webview_window("main").unwrap();

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
                window.open_devtools();
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let latest = LatestSnapshot::default();
            let recorder = recorder_task::spawn(app.handle(), latest.clone());

            app.manage(AppState { latest, recorder });

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
