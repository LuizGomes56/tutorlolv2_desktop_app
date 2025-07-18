use crate::{
    components::calculator::*,
    external::api::{decode_bytes, send_bytes},
    models::calculator::{InputGame, OutputGame},
    url,
};
use std::rc::Rc;
use yew::{
    Html, classes, function_component, html, platform::spawn_local, use_effect_with, use_reducer,
    use_state,
};

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let input_game = use_reducer(InputGame::default);
    let output_game = use_state(|| None::<Rc<OutputGame>>);

    {
        let output_game = output_game.clone();
        use_effect_with(input_game.clone(), move |input_game| {
            let input_game = input_game.clone();

            web_sys::console::log_1(&format!("{:#?}", *input_game).into());

            spawn_local(async move {
                let response = send_bytes(url!("/api/games/calculator"), &*input_game).await;

                if let Ok(res) = response {
                    match decode_bytes::<OutputGame>(res).await {
                        Ok(data) => {
                            output_game.set(Some(Rc::new(data)));
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("{:#?}", e).into());
                        }
                    }
                }
            });
        });
    }

    html! {
        <div class={classes!(
            "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-4", "w-56"
            )}>
                <img
                    loading={"lazy"}
                    class={classes!("w-full", "img-clipped", "h-16")}
                    src={url!("/img/centered/{}_0.avif", (*input_game).active_player.champion_id)}
                    alt={""}
                />
                <div class={classes!(
                    "grid", "grid-cols-2", "gap-x-2",
                )}>
                    <AbilitySelector input_game={input_game.clone()} />
                    <ExceptionSelector
                        current_player_champion_id={(*input_game).active_player.champion_id.clone()}
                        input_game={input_game.clone()}
                    />
                </div>
                <ItemSelector input_game={input_game.clone()} />
                <RuneSelector input_game={input_game.clone()} />
                <StatsSelector input_game={input_game.clone()} />
            </div>
            <div>
                {
                    if let Some(output_game) = &*output_game {
                        html! {
                            <div class={classes!(
                                "text-white", "text-xl"
                            )}>
                                {output_game.current_player.current_stats.armor}
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
