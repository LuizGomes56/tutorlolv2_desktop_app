use std::time::Duration;
use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, State, WebviewUrl,
    WebviewWindow, WebviewWindowBuilder, ipc::InvokeResponseBody,
};
use tauri_plugin_http::reqwest::Client;

pub struct AppState {
    client: Client,
}

mod kb;
mod kb2;
mod keyboard;

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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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
                    area.size.width,
                    area.size.height,
                )))
                .unwrap();

            overlay.set_ignore_cursor_events(true).unwrap();
            overlay.open_devtools();

            let app_handle = app.handle().clone();

            // let (tx, rx) = std::sync::mpsc::channel::<()>();
            // crate::kb::set_hotkey_sender(tx);
            // unsafe { crate::kb::install_hook() };

            let (tx, rx) = std::sync::mpsc::channel::<kb2::KbEvent>();
            crate::kb2::set_hotkey_sender(tx);
            unsafe { crate::kb2::install_hook() };

            std::thread::spawn(move || {
                let mut ignore_cursor_events = true;

                while let Ok(event) = rx.recv() {
                    let app_handle = app_handle.clone();
                    let overlay = overlay.clone();

                    match event {
                        kb2::KbEvent::ToggleOverlay => {
                            ignore_cursor_events = !ignore_cursor_events;
                            let next_value = ignore_cursor_events;

                            let _ = app_handle.run_on_main_thread(move || {
                                println!("Toggle overlay was called: {next_value}");
                                let _ = overlay.emit("focus", !next_value);
                                let _ = overlay.set_ignore_cursor_events(next_value);
                                let _ = overlay.set_focusable(!next_value);

                                if !next_value {
                                    let _ = overlay.set_focus();
                                }
                            });
                        }

                        kb2::KbEvent::Change => {
                            println!("Change event was called");
                            let _ = app_handle.run_on_main_thread(move || {
                                let _ = overlay.emit("change", ());
                            });
                        }
                    }
                }
            });

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
