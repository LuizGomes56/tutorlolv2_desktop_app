use std::time::Duration;
use tauri::{Manager, State, WebviewWindowBuilder, utils::config::Position};
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
            let monitor = app.primary_monitor()?.ok_or("no primary monitor")?;
            let mpos = monitor.position();
            let msize = monitor.size();

            let window = WebviewWindowBuilder::new(
                app,
                "overlay_1",
                tauri::WebviewUrl::App("/overlay/1".into()),
            )
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .resizable(false)
            .closable(false)
            .shadow(false)
            .focused(false)
            .skip_taskbar(true)
            .position(mpos.x as f64, mpos.y as f64)
            .inner_size(msize.width as f64, msize.height as f64)
            .build()?;

            let _ = window.set_ignore_cursor_events(true);

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
