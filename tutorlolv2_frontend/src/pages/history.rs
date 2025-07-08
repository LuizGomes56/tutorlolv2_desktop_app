use crate::{
    models::{base::ApiError, realtime::ReqRealtime},
    url,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, console};
use yew::{
    Html, InputEvent, TargetCast, classes, function_component, html, use_effect_with, use_state,
};

async fn fetch_game_by_code(game_code: &str) -> Result<ReqRealtime, Box<dyn std::error::Error>> {
    let response = reqwasm::http::Request::post(url!("/api/games/get_by_code"))
        .body(format!("{{\"game_code\":\"{}\"}}", game_code))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let bytes = response.binary().await?;

    match bincode::serde::decode_from_slice::<ReqRealtime, _>(&bytes, bincode::config::standard()) {
        Ok(decoded) => Ok(decoded.0),
        Err(_) => {
            let api_error = bincode::serde::decode_from_slice::<ApiError, _>(
                &bytes,
                bincode::config::standard(),
            )?;

            Err(format!("API returned error: {}", api_error.0.message).into())
        }
    }
}

#[function_component(History)]
pub fn history() -> Html {
    let game_code_handle = use_state(|| String::with_capacity(6));
    let game_handle = use_state(|| Option::<ReqRealtime>::None);

    use_effect_with(game_code_handle.clone(), |handle_ref| {
        let game_code_handle = handle_ref.clone();
        spawn_local(async move {
            if (*game_code_handle).len() == 6 {
                match fetch_game_by_code(&(*game_code_handle)).await {
                    Ok(data) => game_handle.set(Some(data)),
                    Err(e) => console::log_1(&e.to_string().into()),
                }
            }
        });
    });

    html! {
        <div class={classes!(
            "p-6", "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <h1 class={classes!(
                "font-semibold", "text-2xl", "text-white"
            )}>
                { "History" }
            </h1>
            <div class={classes!(
                "flex", "flex-col", "gap-2",
                "text-white"
            )}>
                <span>{ "Insert Game Code" }</span>
                <input
                    class={classes!(
                        "bg-zinc-800", "py-2", "px-4", "rounded-lg",
                    )}
                    type="text"
                    placeholder="0xABCD"
                    oninput={{
                        let game_code_handle = game_code_handle.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let value = input.value();
                            if value.len() == 6 {
                                game_code_handle.set(value);
                            }
                        }
                    }}
                />
            </div>
        </div>
    }
}
