use crate::{
    STATIC_RUNES, color,
    pages::calculator::{CalculatorExt, CalculatorState},
    url,
};
use yew::{Callback, Html, Properties, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct RuneSelectorProps {
    pub input_game: CalculatorState,
}

#[function_component(RuneSelector)]
pub fn rune_selector(props: &RuneSelectorProps) -> Html {
    html! {
        <div class={classes!(
            "absolute", "top-1/2", "left-1/2", "translate-x-[50%]", "translate-y-[-50%]",
            "w-md", "grid", "grid-cols-2", "h-96", "overflow-y-auto", "text-white",
            color!(bg-900), "p-4", "rounded-xl"
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-2",
            )}>
                {
                    for STATIC_RUNES
                        .get()
                        .unwrap()
                        .iter()
                        .map(|(rune_name, rune_id)| {
                            html! {
                                <button
                                    class={classes!(
                                        "grid", "grid-cols-[auto_1fr]",
                                        "items-center", "gap-2", "text-sm",
                                        color!(hover:bg-800),
                                    )}
                                    onclick={{
                                        let input_game = props.input_game.clone();
                                        Callback::from(move |_| {
                                            web_sys::console::log_1(&rune_id.to_string().into());
                                            let _ = input_game.try_update(|game| {
                                                game.active_player.runes.push(*rune_id);
                                            });
                                        })
                                    }}
                                >
                                    <img
                                        class={classes!("w-5", "h-5")}
                                        src={url!("/img/runes/{}.avif", rune_id)}
                                        alt={""}
                                        loading={"lazy"}
                                    />
                                    <span class={classes!("text-left")}>
                                        {rune_name.clone()}
                                    </span>
                                </button>
                            }
                        })
                }
            </div>
            <div class={classes!("flex", "h-fit", "flex-wrap", "gap-2")}>
                {
                    for props
                        .input_game
                        .get()
                        .active_player
                        .runes
                        .iter()
                        .enumerate()
                        .map(|(index, rune_id)| {
                            html! {
                                <button
                                    class={classes!("cursor-pointer")}
                                    onclick={{
                                        let input_game = props.input_game.clone();
                                        Callback::from(move |_| {
                                            let _ = input_game.try_update(|game| {
                                                game.active_player.runes.remove(index);
                                            });
                                        })
                                    }}
                                >
                                    <img
                                        class={classes!("w-5", "h-5")}
                                        src={url!("/img/runes/{}.avif", rune_id)}
                                        alt={""}
                                        loading={"lazy"}
                                    />
                                </button>
                            }
                    })
                }
            </div>
        </div>
    }
}
