use crate::{
    STATIC_ITEMS, STATIC_RUNES, color, components::calculator::InputGameAction,
    models::calculator::InputGame, url,
};
use yew::{Callback, Html, Properties, UseReducerHandle, classes, function_component, html};

#[derive(Clone, Copy, PartialEq)]
pub enum StaticIterator {
    Runes,
    Items,
}

#[derive(PartialEq, Properties)]
pub struct StaticSelectorProps {
    pub input_game: UseReducerHandle<InputGame>,
    pub static_iter: StaticIterator,
}

#[function_component(StaticSelector)]
pub fn static_selector(props: &StaticSelectorProps) -> Html {
    let (owned_values, iterator, path) = match props.static_iter {
        StaticIterator::Runes => (
            &props.input_game.active_player.runes,
            &STATIC_RUNES,
            "/img/runes",
        ),
        StaticIterator::Items => (
            &props.input_game.active_player.items,
            &STATIC_ITEMS,
            "/img/items",
        ),
    };

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
                    for iterator
                        .get()
                        .unwrap()
                        .iter()
                        .map(|(name, id)| {
                            html! {
                                <button
                                    class={classes!(
                                        "grid", "grid-cols-[auto_1fr]",
                                        "items-center", "gap-2", "text-sm",
                                        color!(hover:bg-800), "select-none",
                                        "cursor-pointer"
                                    )}
                                    onclick={{
                                        let input_game = props.input_game.clone();
                                        let static_iter = props.static_iter;
                                        Callback::from(move |_| {
                                            input_game.dispatch(match static_iter {
                                                StaticIterator::Runes => InputGameAction::InsertCurrentPlayerRune(*id),
                                                StaticIterator::Items => InputGameAction::InsertCurrentPlayerItem(*id),
                                            });
                                        })
                                    }}
                                >
                                    <img
                                        class={classes!("w-5", "h-5")}
                                        src={url!("{}/{}.avif", path, id)}
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
            <div class={classes!("flex", "h-fit", "flex-wrap", "gap-2")}>
                {
                    for owned_values
                        .iter()
                        .enumerate()
                        .map(|(index, id)| {
                            html! {
                                <button
                                    class={classes!("cursor-pointer", "select-none")}
                                    onclick={{
                                        let input_game = props.input_game.clone();
                                        let static_iter = props.static_iter;
                                        Callback::from(move |_| {
                                            input_game.dispatch(match static_iter {
                                                StaticIterator::Runes => InputGameAction::RemoveCurrentPlayerRune(index),
                                                StaticIterator::Items => InputGameAction::RemoveCurrentPlayerItem(index),
                                            });
                                        })
                                    }}
                                >
                                    <img
                                        class={classes!("w-5", "h-5")}
                                        src={url!("{}/{}.avif", path, id)}
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
