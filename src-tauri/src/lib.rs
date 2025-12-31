use std::time::Duration;
use tauri::{Manager, State, WebviewWindowBuilder, ipc::InvokeResponseBody};
use tauri_plugin_http::reqwest::Client;

pub struct AppState {
    client: Client,
}

mod keyboard;

#[tauri::command]
async fn get_live_game(state: State<'_, AppState>) -> Result<InvokeResponseBody, String> {
    let response = state
        .client
        .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
        .send()
        .await
        .map_err(|e| format!("Error when fetching local: {:#?}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Error transforming response to bytes: {:#?}", e))?
        .to_vec();

    // let bytes = std::fs::read("example.json").unwrap();
    Ok(InvokeResponseBody::Raw(bytes))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_live_game])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_shadow(false);
            let _ = window.set_decorations(false);
            let _ = window.set_ignore_cursor_events(true);
            let _ = window.set_decorations(false);
            let _ = window.set_always_on_top(true);
            let _ = window.set_resizable(false);
            let _ = window.set_closable(false);
            let _ = window.set_shadow(false);
            let _ = window.set_skip_taskbar(true);
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
