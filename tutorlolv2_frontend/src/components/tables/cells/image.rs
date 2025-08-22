use crate::{
    color,
    components::{
        Image, ImageType,
        hover::{docs::hover_docs, item_stats::ItemStatsHover},
    },
    context::{HoverDocs, SettingsContext},
    url,
    utils::ComptimeCache,
};
use generated_code::{
    BASIC_ATTACK_OFFSET, CHAMPION_ABILITIES, CRITICAL_STRIKE_OFFSET, ChampionId, ITEM_FORMULAS,
    ItemId, ONHIT_EFFECT_OFFSET, RUNE_FORMULAS, RuneId,
};
use yew::{AttrValue, Html, Properties, classes, function_component, html, use_context};

#[derive(PartialEq)]
pub enum Instances {
    Attacks,
    Abilities(ChampionId),
    Items(ItemId),
    Runes(RuneId),
    Champions(ChampionId),
}

#[derive(Properties, PartialEq)]
pub struct ImageCellProps {
    pub instance: Instances,
}

#[inline]
fn base_content(img_path: ImageType, content: Html) -> Html {
    html! {
        <div class={classes!(
            "flex", "items-center", "justify-center",
            "relative", "cell"
        )}>
            <Image class={classes!("w-8", "h-8")} source={img_path} />
            { content }
        </div>
    }
}

#[inline]
fn chain_th(content: Html) -> Html {
    html! {
        <th class={classes!("group", "min-w-10")}>
            {content}
        </th>
    }
}

#[function_component(ImageCell)]
pub fn image_cell(props: &ImageCellProps) -> Html {
    let hover_settings = use_context::<SettingsContext>()
        .and_then(|ctx| Some((*ctx).docs))
        .unwrap_or_default();

    macro_rules! insert_attack {
        ($offset:ident, $url:literal) => {
            html! {
                <>
                    {chain_th(base_content(
                        ImageType::Other(AttrValue::Static(url!($url))),
                        match hover_settings {
                            HoverDocs::Full => html! {
                                hover_docs(
                                    AttrValue::Static($offset.as_str()),
                                    true
                                )
                            },
                            _ => html!(),
                        },
                    ))}
                </>
            }
        };
    }

    match &props.instance {
        Instances::Attacks => {
            html! {
                <>
                    {insert_attack!(BASIC_ATTACK_OFFSET, "/img/other/basic_attack.png")}
                    {insert_attack!(CRITICAL_STRIKE_OFFSET, "/img/stats/crit_chance.svg")}
                    {insert_attack!(ONHIT_EFFECT_OFFSET, "/img/stats/onhit.svg")}
                </>
            }
        }
        Instances::Abilities(champion_id) => {
            html! {
                CHAMPION_ABILITIES
                    .get(*champion_id as usize)
                    .and_then(|coords| {
                        Some(coords
                            .into_iter()
                            .map(|(ability_like, coord)| {
                                let first_char = ability_like.as_char();
                                chain_th(
                                    base_content(
                                        ImageType::Abilities(
                                            *champion_id, *ability_like
                                        ),
                                        html! {
                                            <>
                                                <span class={classes!("text-sm", "img-letter")}>
                                                    {first_char}
                                                    // <sub>{ ability_like..padding_chars() }</sub>
                                                </span>
                                                {
                                                    match hover_settings {
                                                        HoverDocs::Full => hover_docs(
                                                            AttrValue::Static(coord.as_str()),
                                                            true
                                                        ),
                                                        _ => html!(),
                                                    }
                                                }
                                            </>
                                        },
                                    )
                                )
                            })
                            .collect::<Html>()
                        )
                    })
                    .unwrap_or_default()
            }
        }
        Instances::Items(item_id) => base_content(
            ImageType::Items(*item_id),
            match hover_settings {
                HoverDocs::None => html!(),
                _ => ITEM_FORMULAS
                    .get(*item_id as usize)
                    .and_then(|coord| match hover_settings {
                        HoverDocs::Full => Some(html! {
                            <div class={classes!(
                                "group-hover:visible",
                                "group-hover:opacity-100",
                                "group-hover:pointer-events-auto",
                                "opacity-0", "invisible",
                                "pointer-events-none",
                                "transition-[visibility,opacity]",
                                "duration-200",
                                "group-hover:delay-1000",
                                "flex-col", "flex", "absolute",
                                "translate-x-[calc(50%-16px)]",
                                "translate-y-[calc(50%+20px)]",
                                "z-50", "py-3", "hover-docs", "gap-y-2",
                                "border", color!(border-800), color!(bg-900),
                                "overflow-auto", "max-h-96", "px-3.5",
                            )}>
                                <ItemStatsHover item_id={*item_id} />
                                {hover_docs(AttrValue::Static(coord.as_str()), false)}
                            </div>
                        }),
                        _ => Some(html! {
                            <div class={classes!(
                                "group-hover:flex", "flex-col", "absolute", "hover-docs", "hidden",
                                "translate-x-[calc(50%-16px)]", "translate-y-[calc(50%+20px)]",
                                "z-50", "py-3", "border", color!(border-800), "gap-y-2",
                                "overflow-auto", "max-h-96", "px-3.5", color!(bg-900),
                            )}>
                                <ItemStatsHover item_id={*item_id} />
                            </div>
                        }),
                    })
                    .unwrap_or_default(),
            },
        ),
        Instances::Runes(rune_id) => base_content(
            ImageType::Runes(*rune_id),
            match hover_settings {
                HoverDocs::Full => RUNE_FORMULAS
                    .get(*rune_id as usize)
                    .and_then(|coord| Some(hover_docs(AttrValue::Static(coord.as_str()), true)))
                    .unwrap_or_default(),
                _ => html!(),
            },
        ),
        Instances::Champions(champion_id) => {
            base_content(ImageType::Champions(*champion_id), html!())
        }
    }
}
