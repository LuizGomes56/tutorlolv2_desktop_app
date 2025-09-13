use bincode::{Decode, Encode};
use web_sys::AbortSignal;

pub async fn send_bytes<T: Encode>(
    url: &str,
    data: &T,
    signal: Option<AbortSignal>,
) -> Option<reqwasm::http::Response> {
    match bincode::encode_to_vec(data, bincode::config::standard()) {
        Ok(body) => {
            let mut request = reqwasm::http::Request::post(url)
                .body(body)
                .header("Content-Type", "application/octet-stream");

            if let Some(ref signal) = signal {
                request = request.abort_signal(Some(signal));
            }

            request.send().await.ok()
        }
        Err(e) => {
            web_sys::console::log_1(&format!("{:#?}", e).into());
            None
        }
    }
}

pub async fn decode_bytes<T: Decode<()>>(response: reqwasm::http::Response) -> Option<T> {
    let bytes = response.binary().await.ok()?;

    match bincode::decode_from_slice::<T, _>(&bytes, bincode::config::standard()) {
        Ok((decoded, _)) => Some(decoded),
        Err(e) => {
            web_sys::console::log_1(&format!("{:#?}", e).into());
            match std::str::from_utf8(&bytes) {
                Ok(s) => {
                    web_sys::console::log_1(&s.into());
                }
                Err(e) => {
                    web_sys::console::log_1(
                        &format!("API Responded with unknown bytes: {:?}", e).into(),
                    );
                }
            };
            None
        }
    }
}
