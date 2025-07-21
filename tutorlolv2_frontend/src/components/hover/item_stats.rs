use crate::{STATIC_COMPARED_ITEMS, macros::STATS_URL};
use yew::{Html, Properties, classes, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ItemStatsHoverProps {
    pub item_id: u32,
}

#[function_component(ItemStatsHover)]
pub fn item_stats_hover(props: &ItemStatsHoverProps) -> Html {
    html! {
        <div class={classes!(
            "grid", "grid-cols-[auto_auto_1fr_auto_auto_1fr]", "gap-x-2",
            "items-center", "text-xs", "max-w-md", "bg-zinc-900", "hover-docs",
            "border-zinc-800", "z-30", "border", "py-2", "px-3", "gap-y-0.5"
        )}>
            {
                STATIC_COMPARED_ITEMS
                    .get()
                    .and_then(|items| items.get(&props.item_id))
                    .and_then(|item| {
                    item.prettified_stats
                    .iter()
                    .map(|(key, val)| {
                    STATS_URL
                    .get(key)
                    .and_then(|&stat_url| {
                        Some(
                            html! {
                                <>
                                    <img
                                        loading={"lazy"}
                                        class={classes!("w-3", "h-3")}
                                        src={stat_url}
                                        alt={""}
                                    />
                                    <span class={classes!(
                                        "font-medium", "text-zinc-300",
                                    )}>
                                        {val}
                                    </span>
                                    <span class={classes!("text-zinc-400")}>
                                        {key}
                                    </span>
                                </>
                            }
                        )
                    })
                    })
                    .collect::<Option<Html>>()
                    })
                    .unwrap_or_default()
            }
        </div>
    }
}
