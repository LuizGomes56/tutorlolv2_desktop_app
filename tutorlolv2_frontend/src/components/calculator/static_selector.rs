use crate::{
    color,
    components::{Image, ImageType, calculator::StaticIterator, hover::item_stats::ItemStatsHover},
    svg,
    utils::UnsafeCast,
};
use generated_code::{
    CHAMPION_ID_TO_NAME, ChampionId, ITEM_ID_TO_NAME, ItemId, RUNE_ID_TO_NAME, RuneId,
};
use yew::{
    Callback, Html, InputEvent, NodeRef, Properties, TargetCast, classes, function_component, html,
    use_callback, use_memo, use_state,
};

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
    pub callback: Callback<T>,
    pub static_iter: StaticIterator,
    pub node_ref: NodeRef,
}

#[derive(Clone, PartialEq)]
struct StaticSelectorItem {
    index: usize,
    name: &'static str,
    html: Html,
}

#[function_component(StaticSelector)]
pub fn static_selector<T>(props: &StaticSelectorProps<T>) -> Html
where
    T: PartialEq + UnsafeCast + 'static,
    T::Repr: TryInto<usize> + TryFrom<usize>,
{
    let search_query = use_state(|| String::new());
    let static_iterator: &[&'static str] = match props.static_iter {
        StaticIterator::Items => &ITEM_ID_TO_NAME,
        StaticIterator::Runes => &RUNE_ID_TO_NAME,
        StaticIterator::Champions => &CHAMPION_ID_TO_NAME,
    };
    let oninput = {
        let search_query = search_query.clone();
        use_callback((), move |e: InputEvent, _| {
            let input = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            search_query.set(input);
        })
    };

    let all_values = use_memo((), |_| {
        let len = static_iterator.len();
        static_iterator
            .into_iter()
            .enumerate()
            .map(|(index, &name)| {
                let html = html! {
                    <button
                        class={classes!(
                            "items-center", "gap-2", "text-sm",
                            "select-none", "border", "relative",
                            color!(border-700),
                        )}
                        onclick={{
                            let callback = props.callback.clone();
                            Callback::from(move |_| {
                                callback.emit(T::from_usize_unchecked(index));
                            })
                        }}
                    >
                        <Image
                            source={
                                match props.static_iter {
                                    StaticIterator::Items => {
                                        ImageType::Items(ItemId::from_usize_unchecked(index))
                                    }
                                    StaticIterator::Champions => {
                                        ImageType::Champions(ChampionId::from_usize_unchecked(index))
                                    }
                                    StaticIterator::Runes => {
                                        ImageType::Runes(RuneId::from_usize_unchecked(index))
                                    }
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
                                (props.static_iter == StaticIterator::Items).then_some(
                                    html! {
                                        <ItemStatsHover item_id={ItemId::from_usize_unchecked(index)} />
                                    }
                                )
                            }
                        </div>
                    </button>
                };
                StaticSelectorItem { index, name, html }
            })
            .collect::<Vec<_>>()
    });

    let visible_values = all_values
        .iter()
        .filter(|value| {
            value
                .name
                .to_lowercase()
                .contains(&search_query.to_lowercase())
        })
        .collect::<Vec<_>>();

    html! {
        <div
            ref={props.node_ref.clone()}
            class={classes!(
                "fixed", "top-1/2", "left-1/2", "-translate-x-1/2", "-translate-y-1/2",
                "z-50", "max-h-96", "overflow-y-auto", "bg-[#121214]",
                "flex", "flex-col", "gap-4", "h-full"
            )}
        >
            <label
                class={classes!(
                    "bg-[#1f1f25]", "flex", "items-center", "gap-2",
                    color!(text-200), "pl-10", "pr-4", "relative",
                )}
            >
                <span class={classes!("absolute", "left-4", color!(text-400))}>
                    {svg!("../../../public/svgs/search", "14")}
                </span>
                <input
                    type={"text"}
                    class={classes!(
                        "text-white", "focus:outline-none", "w-full", "ml-1",
                        "h-10"
                    )}
                    value={(*search_query).clone()}
                    placeholder={"Search by name"}
                    oninput={oninput}
                />
            </label>
            <div class={classes!(
                "grid", "gap-4", "grid-cols-12",
            )}>
                {for visible_values.iter().map(|item| item.html.clone())}
            </div>
        </div>
    }
}
