use crate::{
    models::{base::ApiError, realtime::ReqRealtime},
    url,
};
use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};
use web_sys::{HtmlInputElement, console};
use yew::{
    Html, InputEvent, TargetCast, classes, function_component, html, platform::spawn_local,
    use_effect_with, use_state, use_state_eq,
};

async fn fetch_game_by_code(game_code: &str) -> Result<ReqRealtime, Box<dyn std::error::Error>> {
    console::log_1(&format!("Fetching game, Code: {}", game_code).into());
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

static LOOP_FLAG: AtomicBool = AtomicBool::new(false);

macro_rules! loop_flag {
    (set $boolean:literal) => {
        LOOP_FLAG.store($boolean, Ordering::SeqCst);
    };
    () => {
        LOOP_FLAG.load(Ordering::SeqCst)
    };
}

#[function_component(History)]
pub fn history() -> Html {
    let game_code = use_state(|| String::with_capacity(6));
    let game_data = use_state_eq(|| Rc::new(None::<ReqRealtime>));

    {
        let game_data = game_data.clone();
        let game_code = game_code.clone();
        use_effect_with(game_code.clone(), move |_| {
            loop_flag!(set true);
            if (*game_code).len() != 6 {
                return;
            }

            loop_flag!(set false);

            spawn_local(async move {
                let mut failures = 0usize;

                loop {
                    if loop_flag!() {
                        break;
                    }

                    match fetch_game_by_code(&(*game_code)).await {
                        Ok(response) => {
                            game_data.set(Rc::new(Some(response)));
                            failures = 0;
                        }
                        Err(e) => {
                            console::log_1(&e.to_string().into());
                            failures += 1;
                        }
                    }

                    let delay = if failures > 10 {
                        std::time::Duration::from_secs(60)
                    } else {
                        std::time::Duration::from_secs(1)
                    };

                    gloo_timers::future::sleep(delay).await;
                }
            });
        });
    }

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
                    placeholder="ABC123"
                    oninput={{
                        let game_code = game_code.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let value = input.value();
                            game_code.set(value);
                        }
                    }}
                />
            </div>
        </div>
    }
}
