use crate::models::base::ApiError;
use serde::{Serialize, de::DeserializeOwned};

pub async fn send_bytes<T: Serialize>(
    url: &str,
    data: &T,
) -> Result<reqwasm::http::Response, String> {
    match bincode::serde::encode_to_vec(data, bincode::config::standard()) {
        Ok(body) => reqwasm::http::Request::post(url)
            .body(body)
            .header("Content-Type", "application/octet-stream")
            .send()
            .await
            .map_err(|e| e.to_string()),
        Err(e) => {
            web_sys::console::log_1(&format!("{:#?}", e).into());
            Err(e.to_string())
        }
    }
}

pub async fn decode_bytes<T: DeserializeOwned>(
    response: reqwasm::http::Response,
) -> Result<T, Box<dyn std::error::Error>> {
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
