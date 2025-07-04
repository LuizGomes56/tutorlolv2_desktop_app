use reqwest::Client;
use tauri::{Manager, State, WebviewWindowBuilder};

pub struct AppState {
    client: Client,
}

#[tauri::command]
async fn get_live_game(state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    let response = state
        .client
        .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
        .send()
        .await
        .map_err(|e| format!("Error when fetching local: {:#?}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Error transforming response to bytes: {:#?}", e))?;

    Ok(bytes.to_vec())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_live_game])
        .setup(|app| {
            for id in 1..2 {
                let label = format!("child_process_{}", id);
                let url = format!("/child_process/{}", id);
                let window = WebviewWindowBuilder::new(
                    app,
                    label.clone(),
                    tauri::WebviewUrl::App(url.into()),
                )
                .title(format!("Child Process #{}", id))
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .closable(false)
                .shadow(false)
                .center()
                .accept_first_mouse(false)
                .focused(false)
                .resizable(false)
                .minimizable(false)
                .maximizable(false)
                .skip_taskbar(true)
                .fullscreen(true)
                .build()?;

                window.set_ignore_cursor_events(true).ok();
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let client = Client::builder()
                .danger_accept_invalid_certs(true)
                .timeout(std::time::Duration::from_secs(1 << 4))
                .build()
                .unwrap();

            app.manage(AppState { client });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
