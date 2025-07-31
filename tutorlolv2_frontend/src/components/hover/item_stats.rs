use crate::{
    build_imports::ITEM_DESCRIPTIONS,
    color,
    components::{Image, ImageType},
    macros::STATS_URL,
};
use yew::{AttrValue, Html, Properties, classes, function_component, html};

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
                                <Image
                                    class={classes!("min-w-3", "max-w-3", "aspect-square")}
                                    source={ImageType::Other(AttrValue::Static(stat_url))}
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
