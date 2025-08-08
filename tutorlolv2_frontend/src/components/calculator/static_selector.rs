use crate::{
    color,
    components::{Image, ImageType, calculator::StaticIterator, hover::item_stats::ItemStatsHover},
};
use generated_code::{ITEM_NAME_TO_ID, RUNE_NAME_TO_ID};
use yew::{Callback, Html, Properties, classes, function_component, html, use_memo};

#[derive(PartialEq, Properties)]
pub struct StaticEventProps {
    pub remove_callback: Callback<usize>,
    pub iterator: Vec<u32>,
    pub static_iter: StaticIterator,
}

#[function_component(StaticEvent)]
pub fn static_event(props: &StaticEventProps) -> Html {
    html! {
        <div class={classes!(
            "grid", "gap-4", "grid-cols-8",
            "h-fit", "w-fit"
        )}>
            {
                for props.iterator
                    .iter()
                    .enumerate()
                    .map(|(index, id)| {
                        html! {
                            <button
                                class={classes!(
                                    "items-center", "gap-2", "text-sm",
                                    "select-none", "border", "relative",
                                    color!(border-700), "cursor-pointer"
                                )}
                                onclick={{
                                    let remove_callback = props.remove_callback.clone();
                                    Callback::from(move |_| {
                                        remove_callback.emit(index);
                                    })
                                }}
                            >
                                <Image
                                    class={classes!("h-10", "w-10")}
                                    source={
                                        match props.static_iter {
                                            StaticIterator::Runes => ImageType::Runes(*id),
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
                "grid", "gap-4", "grid-cols-12",
            )}>
                {
                    for static_iterator
                        .entries()
                        .enumerate()
                        .map(|(index, (name, id))| {
                            let len = static_iterator.len();
                            html! {
                                <button
                                    class={classes!(
                                        "items-center", "gap-2", "text-sm",
                                        "select-none", "border", "relative",
                                        color!(border-700),
                                    )}
                                    onclick={{
                                        let insert_callback = props.insert_callback.clone();
                                        Callback::from(move |_| {
                                            insert_callback.emit(*id);
                                        })
                                    }}
                                >
                                    <Image
                                        source={
                                            if props.static_iter == StaticIterator::Items {
                                                ImageType::Items(*id)
                                            } else {
                                                ImageType::Runes(*id)
                                            }
                                        }
                                        class={classes!("h-10", "w-10", "peer")}
                                    />
                                    <div class={classes!(
                                        "hidden", "flex-col", "peer-hover:flex",
                                        "absolute", "z-50", "py-2", "border",
                                        color!(border-800), "gap-y-1.5", "overflow-auto",
                                        "max-h-96", "px-3.5", color!(bg-900),
                                        if index % 12 > 6 {
                                            "-translate-x-[calc(100%-41px)]"
                                        } else { "-translate-x-[1px]" },
                                        if index > len.div_ceil(2) && index > 100 {
                                            "-translate-y-[calc(100%+41px)]"
                                        } else { "translate-y-[1px]" },
                                    )}>
                                        <span class={classes!(
                                            "text-left", "font-medium", "text-lg",
                                            "text-white", "text-nowrap",
                                            "max-w-64", "truncate",
                                        )}>
                                            {name}
                                        </span>
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
