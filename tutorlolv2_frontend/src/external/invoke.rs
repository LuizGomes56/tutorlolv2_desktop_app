use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::{JsString, Uint8Array};

#[wasm_bindgen(module = "/public/invoke.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = invoke_get_live_game, catch)]
    pub async fn invoke_get_live_game() -> Result<Uint8Array, JsString>;

    #[wasm_bindgen(js_name = invoke_checkup)]
    pub fn invoke_checkup() -> bool;
}
