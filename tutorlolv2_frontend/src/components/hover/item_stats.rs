use crate::{STATIC_ITEMS_DEF, color, macros::STATS_URL};
use yew::{Html, Properties, classes, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ItemStatsHoverProps {
    pub item_id: u32,
}

#[function_component(ItemStatsHover)]
pub fn item_stats_hover(props: &ItemStatsHoverProps) -> Html {
    let item = match STATIC_ITEMS_DEF
        .get()
        .and_then(|map| map.get(&props.item_id))
    {
        Some(item) => item,
        None => return html!(),
    };

    if item.prettified_stats.is_empty() {
        return html!();
    }

    html! {
        <div class={classes!(
            "grid", "gap-x-2", "items-center",
            "max-w-md", color!(bg-900), "hover-docs",
            "z-30", "gap-y-2", "leading-none", "text-sm"
        )}>
            {
                for item.prettified_stats.iter().filter_map(|(key, val)| {
                    STATS_URL.get(key).map(|&stat_url| {
                        html! {
                            <div class={classes!("flex","items-center","gap-2")}>
                                <img
                                    loading={"lazy"}
                                    class={classes!("w-3","h-3")}
                                    src={stat_url}
                                    alt={""}
                                />
                                <span class={classes!(color!(text-300),"font-medium")}>
                                    { val }
                                </span>
                                <span class={classes!(color!(text-400),"font-normal")}>
                                    { key }
                                </span>
                            </div>
                        }
                    })
                })
            }
        </div>
    }
}
