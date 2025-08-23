use crate::{
    components::{Image, ImageType},
    url,
};
use generated_code::{ITEM_DESCRIPTIONS, ItemId};
use yew::{AttrValue, Html, Properties, classes, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ItemStatsHoverProps {
    pub item_id: ItemId,
}

#[function_component(ItemStatsHover)]
pub fn item_stats_hover(props: &ItemStatsHoverProps) -> Html {
    let item = match ITEM_DESCRIPTIONS.get(props.item_id as usize) {
        Some(item) => item,
        None => return html!(),
    };

    if item.prettified_stats.len() == 0 {
        return html!();
    }

    html! {
        <div class={classes!(
            "grid", "grid-cols-[auto_auto_1fr]", "gap-y-0.5",
            "gap-x-2", "_bg-900", "hover-docs", "z-30",
            "text-sm", "text-left", "items-center",
        )}>
            {
                for item.prettified_stats.iter().map(|stat_name| {
                    let (icon, name, value) = stat_name.info();
                    html! {
                        <>
                            <Image
                                class={classes!("min-w-3", "max-w-3", "aspect-square")}
                                source={ImageType::Other(AttrValue::from(url!(static icon)))}
                            />
                            <span class={classes!(
                                "_text-300", "font-medium",
                                "text-nowrap"
                            )}>
                                { value }
                            </span>
                            <span class={classes!(
                                "_text-400", "font-normal",
                                "text-nowrap",
                            )}>
                                { name }
                            </span>
                        </>
                    }
                })
            }
        </div>
    }
}
