use bincode::{Decode, Encode, config::Configuration};
use web_sys::AbortSignal;

const BINCODE_CFG: Configuration = bincode::config::standard();

pub async fn encode_bytes<T: Encode>(
    url: &str,
    data: &T,
    signal: Option<AbortSignal>,
) -> Option<gloo_net::http::Response> {
    match bincode::encode_to_vec(data, BINCODE_CFG) {
        Ok(body) => {
            let mut request = gloo_net::http::Request::post(url)
                .header("Content-Type", "application/octet-stream");

            if let Some(ref signal) = signal {
                request = request.abort_signal(Some(signal));
            }

            request.body(body).ok()?.send().await.ok()
        }
        Err(e) => {
            web_sys::console::log_1(&format!("{:#?}", e).into());
            None
        }
    }
}

pub async fn decode_bytes<T: Decode<()>>(response: gloo_net::http::Response) -> Option<T> {
    let bytes = response.binary().await.ok()?;

    match bincode::decode_from_slice::<T, _>(&bytes, BINCODE_CFG) {
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
