use crate::{
    build_imports::{ITEM_NAME_TO_ID, RUNE_NAME_TO_ID},
    color,
    components::{Image, ImageType, hover::item_stats::ItemStatsHover},
    url,
};
use yew::{Callback, Html, Properties, classes, function_component, html, use_memo};

#[derive(Clone, Copy, PartialEq)]
pub enum StaticIterator {
    Runes,
    Items,
}

#[derive(PartialEq, Properties)]
pub struct StaticEventProps {
    pub remove_callback: Callback<usize>,
    pub iterator: Vec<u32>,
    pub static_iter: StaticIterator,
}

#[function_component(StaticEvent)]
pub fn static_event(props: &StaticEventProps) -> Html {
    html! {
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
                                <Image
                                    size={28}
                                    source={
                                        match props.static_iter {
                                            StaticIterator::Runes => ImageType::Other(url!("/img/runes/{}.avif", id)),
                                            StaticIterator::Items => ImageType::Items(*id),
                                        }
                                    }
                                />
                            </button>
                        }
                })
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct StaticSelectorProps {
    pub insert_callback: Callback<u32>,
    pub static_iter: StaticIterator,
}

#[function_component(StaticSelector)]
pub fn static_selector(props: &StaticSelectorProps) -> Html {
    let static_iterator = match props.static_iter {
        StaticIterator::Items => &ITEM_NAME_TO_ID,
        StaticIterator::Runes => &RUNE_NAME_TO_ID,
    };

    let selector_memo = use_memo((), |_| {
        html! {
            <div class={classes!(
                "grid", "gap-2", "grid-cols-10",
            )}>
                {
                    for static_iterator
                        .entries()
                        .enumerate()
                        .map(|(index, (name, id))| {
                            let len = static_iterator.len();
                            let name_hover = html! {
                                <span class={classes!(
                                    "text-left", "font-medium", "text-lg",
                                    "text-white", "text-nowrap",
                                    "max-w-64", "truncate",
                                )}>
                                    {name}
                                </span>
                            };
                            html! {
                                <button
                                    class={classes!(
                                        "items-center", "gap-2", "text-sm",
                                        "select-none", "border", "relative",
                                        color!(border-700), "cursor-default"
                                    )}
                                    onclick={{
                                        let insert_callback = props.insert_callback.clone();
                                        Callback::from(move |_| {
                                            insert_callback.emit(*id);
                                        })
                                    }}
                                >
                                    <Image
                                        size={28}
                                        source={
                                            if props.static_iter == StaticIterator::Items {
                                                ImageType::Items(*id)
                                            } else {
                                                ImageType::Other(url!("/img/runes/{}.avif", id))
                                            }
                                        }
                                        class={classes!("cursor-pointer", "peer")}
                                    />
                                    <div class={classes!(
                                        "hidden", "flex-col", "peer-hover:flex",
                                        "fixed", "z-50", "pt-2", "pb-3", "border",
                                        color!(border-800), "gap-y-3", "overflow-auto",
                                        "max-h-96", "px-3.5", color!(bg-900),
                                        if index % 10 > 5 {
                                            "-translate-x-[calc(100%-29px)]"
                                        } else { "" },
                                        if index > len.div_ceil(2) && index > 100 {
                                            "-translate-y-[calc(100%+29px)]"
                                        } else { "translate-y-[1px]" },
                                    )}>
                                        {name_hover}
                                        {
                                            if props.static_iter == StaticIterator::Items {
                                                html! {
                                                    <ItemStatsHover item_id={*id} />
                                                }
                                            } else {
                                                html!()
                                            }
                                        }
                                    </div>
                                </button>
                            }
                        })
                }
            </div>
        }
    });

    html! { (*selector_memo).clone() }
}
