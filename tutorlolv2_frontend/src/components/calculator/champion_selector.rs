use crate::{
    STATIC_CHAMPIONS, color, components::calculator::InputGameAction,
    models::calculator::InputGame, url,
};
use yew::{Callback, Html, Properties, UseReducerHandle, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct ChampionSelectorProps {
    pub input_game: UseReducerHandle<InputGame>,
}

#[function_component(ChampionSelector)]
pub fn champion_selector(props: &ChampionSelectorProps) -> Html {
    html! {
        <div class={classes!(
            "absolute", "top-1/2", "left-1/2", "translate-x-[-50%]", "translate-y-[-50%]",
            "w-md", "flex", "flex-col", "h-96", "overflow-y-auto", "text-white",
            color!(bg-900), "p-4", "rounded-xl"
        )}>
            {
                for STATIC_CHAMPIONS
                    .get()
                    .unwrap()
                    .iter()
                    .map(|(id, name)| {
                        html! {
                            <button
                                class={classes!(
                                    "grid", "grid-cols-[auto_1fr]",
                                    "items-center", "gap-2", "p-1", "text-sm",
                                    color!(hover:bg-800), "select-none",
                                    "cursor-pointer"
                                )}
                                onclick={{
                                    let input_game = props.input_game.clone();
                                    Callback::from(move |_| {
                                        input_game.dispatch(InputGameAction::SetCurrentPlayerChampionId(id.clone()));
                                    })
                                }}
                            >
                                <img
                                    class={classes!("w-5", "h-5")}
                                    src={url!("/img/champions/{}.avif", id)}
                                    alt={""}
                                    loading={"lazy"}
                                />
                                <span class={classes!("text-left")}>
                                    {name}
                                </span>
                            </button>
                        }
                    })
            }
        </div>
    }
}
