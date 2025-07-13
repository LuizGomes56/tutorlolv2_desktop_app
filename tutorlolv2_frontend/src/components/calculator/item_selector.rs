use crate::{
    STATIC_ITEMS, color,
    pages::calculator::{CalculatorExt, CalculatorState},
    url,
};
use yew::{Callback, Html, Properties, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct ItemSelectorProps {
    pub data: CalculatorState,
}

#[function_component(ItemSelector)]
pub fn item_selector(props: &ItemSelectorProps) -> Html {
    let data = props.data.clone();
    let remove_item = Callback::from(move |index: usize| {
        data.update(|game| {
            game.active_player.items.remove(index);
        });
    });

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
                                        color!(hover:bg-800),
                                    )}
                                    onclick={{
                                        let data = props.data.clone();
                                        Callback::from(move |_| {
                                            web_sys::console::log_1(&item_id.to_string().into());
                                            data.update(|game| {
                                                game.active_player.items.push(*item_id);
                                            });
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
                                        {item_name.clone()}
                                    </span>
                                </button>
                            }
                        })
                }
            </div>
            <div class={classes!("flex", "h-fit", "flex-wrap", "gap-2")}>
                {
                    for props
                        .data
                        .get()
                        .active_player
                        .items
                        .iter()
                        .map(|item_id| {
                            html! {
                                <button
                                    class={classes!("cursor-pointer")}
                                    // onclick={remove_item.reform(move |_| *item_id)}
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
