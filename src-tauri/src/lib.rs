use std::{sync::atomic::AtomicBool, time::Duration};
use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, State, WebviewUrl,
    WebviewWindow, WebviewWindowBuilder, ipc::InvokeResponseBody,
};
use tauri_plugin_http::reqwest::Client;

pub struct AppState {
    client: Client,
}

mod kb3;

#[tauri::command]
fn blur_overlay(app: AppHandle) -> Result<(), String> {
    let overlay = app
        .get_webview_window("overlay")
        .ok_or("overlay window not found")?;

    overlay
        .set_ignore_cursor_events(true)
        .as_ref()
        .map_err(tauri::Error::to_string)?;
    overlay
        .set_focusable(false)
        .as_ref()
        .map_err(tauri::Error::to_string)?;

    Ok(())
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
        .invoke_handler(tauri::generate_handler![get_live_game, blur_overlay])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

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
                    area.size.width / 2,
                    area.size.height,
                )))
                .unwrap();

            overlay.set_ignore_cursor_events(true).unwrap();
            overlay.open_devtools();

            #[cfg(target_os = "windows")]
            kb3::install_low_level_shortcuts(app.handle()).unwrap();

            #[cfg(debug_assertions)]
            {
                // window.open_devtools();
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
        // todo!()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
