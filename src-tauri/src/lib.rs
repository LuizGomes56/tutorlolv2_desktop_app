use shortcut_commands::{get_overlay_shortcut, set_overlay_shortcut, set_shortcut_recording};
use std::time::Duration;
use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, Size, State, WebviewUrl,
    WebviewWindowBuilder, ipc::InvokeResponseBody,
};
use tauri_plugin_http::reqwest::Client;

pub struct AppState {
    client: Client,
}

mod overlay;
mod shortcut;
mod shortcut_commands;

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

#[tauri::command]
async fn get_live_game(state: State<'_, AppState>) -> Result<InvokeResponseBody, String> {
    let response = state
        .client
        .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
        .send()
        .await
        .map_err(|e| format!("Error when fetching local: {e:#?}"))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Error transforming response to bytes: {e:#?}"))?
        .to_vec();

    // let bytes = std::fs::read("example.json").unwrap();
    Ok(InvokeResponseBody::Raw(bytes))
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

            let client = unsafe {
                Client::builder()
                    .danger_accept_invalid_certs(true)
                    .timeout(Duration::from_secs(1 << 4))
                    .build()
                    .unwrap_unchecked()
            };

            app.manage(AppState { client });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
