use crate::{
    STATIC_ITEM_FORMULAS, STATIC_ITEMS, STATIC_RUNE_FORMULAS, STATIC_RUNES, color,
    components::hover::{docs::hover_docs, item_stats::ItemStatsHover},
    context::{HoverDocs, SettingsContext},
    url,
};
use yew::{Callback, Html, Properties, classes, function_component, html, use_context, use_memo};

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
    let path = match props.static_iter {
        StaticIterator::Runes => "/img/runes",
        StaticIterator::Items => "/img/items",
    };

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
                                <img
                                    class={classes!("w-7", "h-7")}
                                    src={url!("{}/{}.avif", path, id)}
                                    alt={""}
                                    loading={"lazy"}
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
    let (static_iterator, static_formulas, path) = match props.static_iter {
        StaticIterator::Items => (&STATIC_ITEMS, &STATIC_ITEM_FORMULAS, "/img/items"),
        StaticIterator::Runes => (&STATIC_RUNES, &STATIC_RUNE_FORMULAS, "/img/runes"),
    };

    let hover_settings = use_context::<SettingsContext>()
        .and_then(|ctx| Some((*ctx).docs))
        .unwrap_or_default();

    let selector_memo = use_memo((), |_| {
        let static_cell = static_iterator.get().and_then(|v| Some(v));
        match static_cell {
            Some(static_cell) => {
                let len = static_cell.len();
                html! {
                    <div class={classes!(
                        "grid", "gap-2", "grid-cols-10",
                    )}>
                        {
                            for static_cell
                                .iter()
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
                                            <img
                                                class={classes!(
                                                    match hover_settings {
                                                        HoverDocs::Partial | HoverDocs::None => "peer",
                                                        _ => "",
                                                    },
                                                    "w-7", "h-7", "cursor-pointer"
                                                )}
                                                src={url!("{}/{}.avif", path, id)}
                                                alt={""}
                                                loading={"lazy"}
                                            />
                                            {
                                                static_formulas
                                                    .get()
                                                    .and_then(|map| map.get(id))
                                                    .map(|formula| {
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
                                                                html! {
                                                                    <div class={classes!(
                                                                        match hover_settings {
                                                                            HoverDocs::Full => "group-hover:flex",
                                                                            _ => "peer-hover:flex",
                                                                        },
                                                                        "hidden", "flex-col",
                                                                        "fixed", "z-50", "py-3", "border",
                                                                        color!(border-800), "gap-y-3", "overflow-auto",
                                                                        "max-h-96", "px-3.5", color!(bg-900),
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
                                                                                        {hover_docs(formula.as_str().into(), false)}
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
                                                                }
                                                            }
                                                            HoverDocs::None => {
                                                                html! {
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
                                                                }
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
            }
            None => html!(),
        }
    });

    html! { (*selector_memo).clone() }
}
