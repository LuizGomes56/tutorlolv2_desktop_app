use crate::{
    build_imports::{ITEM_FORMULAS, ITEM_NAME_TO_ID, RUNE_FORMULAS, RUNE_NAME_TO_ID},
    color,
    components::{
        Image, ImageType,
        hover::{docs::hover_docs, item_stats::ItemStatsHover},
    },
    context::{HoverDocs, SettingsContext},
    url,
};
use yew::{
    AttrValue, Callback, Html, Properties, classes, function_component, html, use_context, use_memo,
};

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
    let (static_iterator, static_formulas) = match props.static_iter {
        StaticIterator::Items => (&ITEM_NAME_TO_ID, &ITEM_FORMULAS),
        StaticIterator::Runes => (&RUNE_NAME_TO_ID, &RUNE_FORMULAS),
    };

    let hover_settings = use_context::<SettingsContext>()
        .and_then(|ctx| Some((*ctx).docs))
        .unwrap_or_default();

    let selector_memo = use_memo((), |_| {
        let len = static_iterator.len();
        html! {
            <div class={classes!(
                "grid", "gap-2", "grid-cols-10",
            )}>
                {
                    for static_iterator
                        .entries()
                        .enumerate()
                        .map(|(index, (name, id))| {
                            html! {
                                <button
                                    class={classes!(
                                        "items-center", "gap-2", "text-sm",
                                        color!(hover:bg-800), "select-none",
                                        "border", "relative", color!(border-700),
                                        "cursor-default",
                                        match hover_settings {
                                            HoverDocs::Full => "group",
                                            _ => "",
                                        },
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
                                        class={classes!(
                                            match hover_settings {
                                                HoverDocs::Partial | HoverDocs::None => "peer",
                                                _ => "",
                                            },
                                            "cursor-pointer"
                                        )}
                                    />
                                    {
                                        static_formulas
                                            .get(id)
                                            .and_then(|formula| {
                                                let name_hover = html! {
                                                    <span class={classes!(
                                                        "text-left", "font-medium", "text-lg",
                                                        "text-white", "leading-none", "max-w-52",
                                                    )}>
                                                        {name}
                                                    </span>
                                                };
                                                match hover_settings {
                                                    HoverDocs::Full | HoverDocs::Partial => {
                                                        Some(html! {
                                                            <div class={classes!(
                                                                match hover_settings {
                                                                    HoverDocs::Full => {
                                                                        vec![
                                                                            "group-hover:visible",
                                                                            "group-hover:opacity-100",
                                                                            "group-hover:pointer-events-auto",
                                                                            "opacity-0", "invisible",
                                                                            "pointer-events-none",
                                                                            "transition-[visibility,opacity]",
                                                                            "duration-200",
                                                                            "group-hover:delay-1000",
                                                                            "flex",
                                                                        ]
                                                                    },
                                                                    _ => vec!["hidden", "peer-hover:flex"],
                                                                },
                                                                "flex-col", "fixed", "z-50", "py-3",
                                                                color!(border-800), "gap-y-3", "overflow-auto",
                                                                "max-h-96", "px-3.5", color!(bg-900), "border",
                                                                match hover_settings {
                                                                    HoverDocs::Full => {
                                                                        format!("-translate-x-[calc(1px+37.6*{}px)]", index % 10)
                                                                    }
                                                                    _ => {
                                                                        if index % 10 > 4 {
                                                                            "-translate-x-[calc(100%-29px)]".to_string()
                                                                        } else { "-translate-x-[calc(1px)]".to_string() }
                                                                    }
                                                                },
                                                                if index > len.div_ceil(2) && index > 100 {
                                                                    "-translate-y-[calc(100%+29px)]"
                                                                } else { "translate-y-[1px]" }
                                                            )}>
                                                                {
                                                                    match hover_settings {
                                                                        HoverDocs::Full => html! {
                                                                            <>
                                                                                {name_hover}
                                                                                <ItemStatsHover item_id={id} />
                                                                                {hover_docs(AttrValue::Static(formula), false)}
                                                                            </>
                                                                        },
                                                                        _ => html! {
                                                                            <>
                                                                                {name_hover}
                                                                                <ItemStatsHover item_id={id} />
                                                                            </>
                                                                        },
                                                                    }
                                                                }
                                                            </div>
                                                        })
                                                    }
                                                    HoverDocs::None => {
                                                        Some(html! {
                                                            <div class={classes!(
                                                                "hidden", "flex-col", "peer-hover:flex",
                                                                "fixed", "z-50", "py-3", "border",
                                                                color!(border-800), "gap-y-3", "overflow-auto",
                                                                "max-h-96", "px-3.5", color!(bg-900),
                                                                if index % 10 > 5 {
                                                                    "-translate-x-[calc(100%-29px)]"
                                                                } else { "" },
                                                                if index > len.div_ceil(2) && index > 100 {
                                                                    "-translate-y-[calc(100%+29px)]"
                                                                } else { "translate-y-[1px]" }
                                                            )}>
                                                                {name_hover}
                                                            </div>
                                                        })
                                                    },
                                                }
                                            })
                                            .unwrap_or_default()
                                        }
                                </button>
                            }
                        })
                }
            </div>
        }
    });

    html! { (*selector_memo).clone() }
}
