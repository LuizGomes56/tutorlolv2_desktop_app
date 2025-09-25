use std::time::Duration;
use tauri::{Manager, State, WebviewWindowBuilder, ipc::InvokeResponseBody};
use tauri_plugin_http::reqwest::Client;

pub struct AppState {
    client: Client,
}

mod keyboard;

#[tauri::command]
async fn get_live_game(state: State<'_, AppState>) -> Result<InvokeResponseBody, String> {
    // let response = state
    //     .client
    //     .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
    //     .send()
    //     .await
    //     .map_err(|e| format!("Error when fetching local: {:#?}", e))?;

    // let bytes = response
    //     .bytes()
    //     .await
    //     .map_err(|e| format!("Error transforming response to bytes: {:#?}", e))?
    //     .to_vec();

    let bytes = std::fs::read("example.json").unwrap();
    Ok(InvokeResponseBody::Raw(bytes))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_live_game])
        .setup(|app| {
            // let monitor = app.primary_monitor()?.ok_or("no primary monitor")?;
            // let mpos = monitor.position();
            // let msize = monitor.size();

            // for i in 1..3 {
            //     let window = WebviewWindowBuilder::new(
            //         app,
            //         format!("overlay_{}", i),
            //         tauri::WebviewUrl::App(format!("/overlay/{}", i).into()),
            //     )
            //     .decorations(false)
            //     .transparent(true)
            //     .always_on_top(true)
            //     .resizable(false)
            //     .closable(false)
            //     .shadow(false)
            //     .focused(false)
            //     .skip_taskbar(true)
            //     .position(mpos.x as f64, mpos.y as f64)
            //     .inner_size(msize.width as f64, msize.height as f64)
            //     .build()?;

            //     let _ = window.set_ignore_cursor_events(true);

            //     window.set_ignore_cursor_events(true).ok();
            //     #[cfg(debug_assertions)]
            //     window.open_devtools();
            // }

            if cfg!(debug_assertions) {
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
