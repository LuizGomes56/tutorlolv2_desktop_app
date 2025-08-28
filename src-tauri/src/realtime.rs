use tauri::http::{HeaderMap, HeaderValue, header};
use tauri_plugin_http::reqwest::Client;

pub async fn get_live_game(client: Client) -> Result<Vec<u8>, String> {
    let response = client
        .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
        .send()
        .await
        .map_err(|e| format!("Error when fetching local: {:#?}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Error transforming response to bytes: {:#?}", e))?;

    println!("byte len: {:#?}", bytes.len());

    // let bytes = std::fs::read("example.json").unwrap();

    let mut headers = HeaderMap::with_capacity(1);
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    let server_response = client
        .post("http://localhost:8082/api/games/realtime")
        .body(bytes)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Error when fetching server: {:#?}", e))?;

    server_response
        .bytes()
        .await
        .map_err(|e| format!("Error transforming response to bytes: {:#?}", e))
        .and_then(|bytes| Ok(bytes.to_vec()))
}
