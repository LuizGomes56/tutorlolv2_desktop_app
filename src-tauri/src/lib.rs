use std::time::Duration;
use tauri::{Manager, State, WebviewWindowBuilder};
use tauri_plugin_http::reqwest::Client;

pub struct AppState {
    client: Client,
}

mod realtime;

#[tauri::command]
async fn get_live_game(state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    realtime::get_live_game(state.client.clone()).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_live_game])
        .setup(|app| {
            let label = "overlay_1";
            let url = "/overlay/1";
            let window = WebviewWindowBuilder::new(app, label, tauri::WebviewUrl::App(url.into()))
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .closable(false)
                .shadow(false)
                .accept_first_mouse(false)
                .focused(false)
                .resizable(false)
                .minimizable(false)
                .maximizable(false)
                .skip_taskbar(true)
                .build()?;

            window.set_ignore_cursor_events(true).ok();

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let client = Client::builder()
                .danger_accept_invalid_certs(true)
                .timeout(Duration::from_secs(1 << 4))
                .build()
                .unwrap();

            app.manage(AppState { client });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
