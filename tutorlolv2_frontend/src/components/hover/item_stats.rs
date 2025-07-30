use crate::{build_imports::ITEM_DESCRIPTIONS, color, macros::STATS_URL};
use yew::{Html, Properties, classes, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ItemStatsHoverProps {
    pub item_id: u32,
}

#[function_component(ItemStatsHover)]
pub fn item_stats_hover(props: &ItemStatsHoverProps) -> Html {
    let item = match ITEM_DESCRIPTIONS.get(&props.item_id) {
        Some(item) => item,
        None => return html!(),
    };

    if item.prettified_stats.len() == 0 {
        return html!();
    }

    html! {
        <div class={classes!(
            "grid", "grid-cols-[auto_auto_1fr]", "gap-y-0.5",
            "gap-x-2", color!(bg-900), "hover-docs", "z-30",
            "text-sm", "text-left", "items-center",
        )}>
            {
                for item.prettified_stats.iter().filter_map(|(key, val)| {
                    STATS_URL.get(key).map(|&stat_url| {
                        html! {
                            <>
                                <img
                                    loading={"lazy"}
                                    class={classes!(
                                        "min-w-3", "max-w-3", "aspect-square",
                                    )}
                                    src={stat_url}
                                    alt={""}
                                />
                                <span class={classes!(
                                    color!(text-300), "font-medium",
                                    "text-nowrap"
                                )}>
                                    { val }
                                </span>
                                <span class={classes!(
                                    color!(text-400), "font-normal",
                                    "text-nowrap",
                                )}>
                                    { key }
                                </span>
                            </>
                        }
                    })
                })
            }
        </div>
    }
}
