use crate::{STATIC_ITEMS, STATIC_RUNES, color, url};
use yew::{Callback, Html, Properties, classes, function_component, html, use_memo};

#[derive(Clone, Copy, PartialEq)]
pub enum StaticIterator {
    Runes,
    Items,
}

#[derive(PartialEq, Properties)]
pub struct StaticSelectorProps {
    pub insert_callback: Callback<usize>,
    pub remove_callback: Callback<usize>,
    pub iterator: Vec<usize>,
    pub static_iter: StaticIterator,
}

#[function_component(StaticSelector)]
pub fn static_selector(props: &StaticSelectorProps) -> Html {
    let (static_iterator, path) = match props.static_iter {
        StaticIterator::Runes => (&STATIC_RUNES, "/img/runes"),
        StaticIterator::Items => (&STATIC_ITEMS, "/img/items"),
    };

    let selector_memo = use_memo((), |_| {
        html! {
            <div class={classes!(
                "flex", "flex-col",
            )}>
                {
                    for static_iterator
                        .get()
                        .unwrap()
                        .iter()
                        .map(|(name, id)| {
                            html! {
                                <button
                                    class={classes!(
                                        "grid", "p-1", "grid-cols-[auto_1fr]",
                                        "items-center", "gap-2", "text-sm",
                                        color!(hover:bg-800), "select-none",
                                        "cursor-pointer"
                                    )}
                                    onclick={{
                                        let insert_callback = props.insert_callback.clone();
                                        Callback::from(move |_| {
                                            insert_callback.emit(*id);
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
        }
    });

    html! {
        <div class={classes!(
            "absolute", "top-1/2", "left-1/2", "translate-x-[-50%]", "translate-y-[-50%]",
            "w-md", "grid", "grid-cols-2", "h-96", "overflow-y-auto", "text-white",
            color!(bg-900), "p-4", "rounded-xl"
        )}>
            {(*selector_memo).clone()}
            <div class={classes!("flex", "h-fit", "flex-wrap", "gap-2")}>
                {
                    for props.iterator
                        .iter()
                        .enumerate()
                        .map(|(index, id)| {
                            html! {
                                <button
                                    class={classes!("cursor-pointer", "select-none")}
                                    onclick={{
                                        let remove_callback = props.remove_callback.clone();
                                        Callback::from(move |_| {
                                            remove_callback.emit(index);
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
