use crate::{
    color,
    components::{Image, ImageType, calculator::StaticIterator, hover::item_stats::ItemStatsHover},
    utils::UnsafeCast,
};
use generated_code::{ITEM_ID_TO_NAME, ItemId, RUNE_ID_TO_NAME, RuneId};
use yew::{Callback, Html, Properties, classes, function_component, html, use_memo};

#[derive(PartialEq, Properties)]
pub struct StaticEventProps<T: PartialEq + 'static> {
    pub remove_callback: Callback<usize>,
    pub iterator: Vec<T>,
    pub static_iter: StaticIterator,
}

#[function_component(StaticEvent)]
pub fn static_event<T>(props: &StaticEventProps<T>) -> Html
where
    T: PartialEq + 'static + Copy,
    ImageType: From<T>,
{
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
                                    source={ImageType::from(*id)}
                                />
                            </button>
                        }
                })
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct StaticSelectorProps<T: PartialEq> {
    pub insert_callback: Callback<T>,
    pub static_iter: StaticIterator,
}

#[function_component(StaticSelector)]
pub fn static_selector<T>(props: &StaticSelectorProps<T>) -> Html
where
    T: PartialEq + UnsafeCast + 'static,
    T::Repr: TryInto<usize> + TryFrom<usize>,
{
    let static_iterator: &[&'static str] = match props.static_iter {
        StaticIterator::Items => &ITEM_ID_TO_NAME,
        StaticIterator::Runes => &RUNE_ID_TO_NAME,
    };

    let selector_memo = use_memo((), |_| {
        html! {
            <div class={classes!(
                "grid", "gap-4", "grid-cols-12",
            )}>
                {
                    for static_iterator
                        .into_iter()
                        .enumerate()
                        .map(|(index, name)| {
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
                                            insert_callback.emit(T::from_usize_unchecked(index));
                                        })
                                    }}
                                >
                                    <Image
                                        source={
                                            if props.static_iter == StaticIterator::Items {
                                                ImageType::Items(ItemId::from_usize_unchecked(index))
                                            } else {
                                                ImageType::Runes(RuneId::from_usize_unchecked(index))
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
                                                    <ItemStatsHover item_id={ItemId::from_usize_unchecked(index)} />
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
