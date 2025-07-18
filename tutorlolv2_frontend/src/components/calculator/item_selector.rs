use crate::{
    STATIC_ITEMS, color, components::calculator::InputGameAction, models::calculator::InputGame,
    url,
};
use yew::{Callback, Html, Properties, UseReducerHandle, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct ItemSelectorProps {
    pub input_game: UseReducerHandle<InputGame>,
}

#[function_component(ItemSelector)]
pub fn item_selector(props: &ItemSelectorProps) -> Html {
    html! {
        <div class={classes!(
            "absolute", "top-1/2", "left-1/2", "translate-x-[-50%]", "translate-y-[-50%]",
            "w-md", "grid", "grid-cols-2", "h-96", "overflow-y-auto", "text-white",
            color!(bg-900), "p-4", "rounded-xl"
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-2",
            )}>
                {
                    for STATIC_ITEMS
                        .get()
                        .unwrap()
                        .iter()
                        .map(|(item_name, item_id)| {
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
                                        Callback::from(move |_| {
                                            input_game.dispatch(InputGameAction::InsertCurrentPlayerItem(*item_id));
                                        })
                                    }}
                                >
                                    <img
                                        class={classes!("w-5", "h-5")}
                                        src={url!("/img/items/{}.avif", item_id)}
                                        alt={""}
                                        loading={"lazy"}
                                    />
                                    <span class={classes!("text-left")}>
                                        {item_name}
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
                        .active_player
                        .items
                        .iter()
                        .enumerate()
                        .map(|(index, item_id)| {
                            html! {
                                <button
                                    class={classes!("cursor-pointer", "select-none")}
                                    onclick={{
                                        let input_game = props.input_game.clone();
                                        Callback::from(move |_| {
                                            input_game.dispatch(InputGameAction::RemoveCurrentPlayerItem(index));
                                        })
                                    }}
                                >
                                    <img
                                        class={classes!("w-5", "h-5")}
                                        src={url!("/img/items/{}.avif", item_id)}
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
