use crate::models::base::ApiError;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use web_sys::console;

pub async fn fetch_backend<T: DeserializeOwned, U: Into<JsValue>>(
    url: &str,
    body: U,
) -> Result<T, Box<dyn std::error::Error>> {
    console::log_1(&format!("Fetching url: {}", url).into());
    let response = reqwasm::http::Request::post(url)
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let bytes = response.binary().await?;

    match bincode::serde::decode_from_slice::<T, _>(&bytes, bincode::config::standard()) {
        Ok(decoded) => Ok(decoded.0),
        Err(e) => {
            let api_error = bincode::serde::decode_from_slice::<ApiError, _>(
                &bytes,
                bincode::config::standard(),
            )?;

            Err(format!(
                "API returned error: {}, error: {:#?}",
                api_error.0.message, e
            )
            .into())
        }
    }
}
