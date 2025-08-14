use crate::models::base::ApiError;
use bincode::{Decode, Encode};
use web_sys::AbortSignal;

pub async fn send_bytes<T: Encode>(
    url: &str,
    data: &T,
    signal: Option<AbortSignal>,
) -> Result<reqwasm::http::Response, String> {
    match bincode::encode_to_vec(data, bincode::config::standard()) {
        Ok(body) => {
            let mut request = reqwasm::http::Request::post(url)
                .body(body)
                .header("Content-Type", "application/octet-stream");

            if let Some(ref signal) = signal {
                request = request.abort_signal(Some(signal));
            }

            request.send().await.map_err(|e| e.to_string())
        }
        Err(e) => {
            web_sys::console::log_1(&format!("{:#?}", e).into());
            Err(e.to_string())
        }
    }
}

pub async fn decode_bytes<T: Decode<()>>(
    response: reqwasm::http::Response,
) -> Result<T, Box<dyn std::error::Error>> {
    let bytes = response.binary().await?;

    match bincode::decode_from_slice::<T, _>(&bytes, bincode::config::standard()) {
        Ok(decoded) => Ok(decoded.0),
        Err(e) => {
            let api_error =
                bincode::decode_from_slice::<ApiError, _>(&bytes, bincode::config::standard())?;

            Err(format!(
                "API returned error: {}, error: {:#?}",
                api_error.0.message, e
            )
            .into())
        }
    }
}
