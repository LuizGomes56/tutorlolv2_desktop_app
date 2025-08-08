use crate::{
    color,
    components::{
        Image, ImageType,
        hover::{docs::hover_docs, item_stats::ItemStatsHover},
    },
    context::{HoverDocs, SettingsContext},
    url,
    utils::{ComptimeCache, StringExt},
};
use generated_code::{
    BASIC_ATTACK_OFFSET, CHAMPION_ABILITIES, CRITICAL_STRIKE_OFFSET, ITEM_FORMULAS,
    ONHIT_EFFECT_OFFSET, RUNE_FORMULAS,
};
use yew::{AttrValue, Html, Properties, classes, function_component, html, use_context};

#[derive(PartialEq)]
pub enum Instances {
    Abilities(AttrValue),
    Items(u32),
    Runes(u32),
    Champions(AttrValue),
}

#[derive(Properties, PartialEq)]
pub struct ImageCellProps {
    pub instance: Instances,
}

#[function_component(ImageCell)]
pub fn image_cell(props: &ImageCellProps) -> Html {
    let hover_settings = use_context::<SettingsContext>()
        .and_then(|ctx| Some((*ctx).docs))
        .unwrap_or_default();

    let base_content = |img_path, content| {
        html! {
            <div class={classes!(
                "flex", "items-center", "justify-center",
                "relative", "cell"
            )}>
                <Image class={classes!("w-8", "h-8")} source={img_path} />
                { content }
            </div>
        }
    };

    let chain_th = |content: Html| {
        html! {
            <th class={classes!("group", "min-w-10")}>
                {content}
            </th>
        }
    };

    match &props.instance {
        Instances::Abilities(champion_id) => {
            html! {
                CHAMPION_ABILITIES
                    .get(champion_id)
                    .and_then(|phf_formula_map| {
                        Some(phf_formula_map
                            .entries()
                            .map(|(ability_name, formula)| {
                                let first_char = ability_name.first_char();
                                chain_th(
                                    base_content(
                                        ImageType::Abilities(
                                            champion_id.concat_char(first_char),
                                        ),
                                        html! {
                                            <>
                                                <span class={classes!("text-sm", "img-letter")}>
                                                    {first_char}
                                                    <sub>{ ability_name.padding_chars() }</sub>
                                                </span>
                                                {
                                                    match hover_settings {
                                                        HoverDocs::Full => hover_docs(
                                                            AttrValue::Static(formula.as_str()),
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
                            .chain(std::iter::once(
                                chain_th(base_content(
                                    ImageType::Other(AttrValue::Static(url!("/img/other/basic_attack.png"))),
                                    match hover_settings {
                                        HoverDocs::Full => html! {
                                            hover_docs(
                                                AttrValue::Static(BASIC_ATTACK_OFFSET.as_str()),
                                                true
                                            )
                                        },
                                        _ => html!(),
                                    },
                                ))
                            ))
                            .chain(std::iter::once(
                                chain_th(base_content(
                                    ImageType::Other(AttrValue::Static(url!("/img/stats/crit_chance.svg"))),
                                    match hover_settings {
                                        HoverDocs::Full => html! {
                                            hover_docs(
                                                AttrValue::Static(CRITICAL_STRIKE_OFFSET.as_str()),
                                                true
                                            )
                                        },
                                        _ => html!(),
                                    },
                                ))
                            ))
                            .chain(std::iter::once(
                                chain_th(base_content(
                                    ImageType::Other(AttrValue::Static(url!("/img/stats/onhit.svg"))),
                                    match hover_settings {
                                        HoverDocs::Full => html! {
                                            hover_docs(
                                                AttrValue::Static(ONHIT_EFFECT_OFFSET.as_str()),
                                                true
                                            )
                                        },
                                        _ => html!(),
                                    },
                                ))
                            ))
                            .collect::<Html>()
                        )
                    })
                    .unwrap_or_default()
            }
        }
        Instances::Items(keyname) => base_content(
            ImageType::Items(*keyname),
            match hover_settings {
                HoverDocs::None => html!(),
                _ => ITEM_FORMULAS
                    .get(keyname)
                    .and_then(|formula| match hover_settings {
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
                                <ItemStatsHover item_id={keyname} />
                                {hover_docs(AttrValue::Static(formula.as_str()), false)}
                            </div>
                        }),
                        _ => Some(html! {
                            <div class={classes!(
                                "group-hover:flex", "flex-col", "absolute", "hover-docs", "hidden",
                                "translate-x-[calc(50%-16px)]", "translate-y-[calc(50%+20px)]",
                                "z-50", "py-3", "border", color!(border-800), "gap-y-2",
                                "overflow-auto", "max-h-96", "px-3.5", color!(bg-900),
                            )}>
                                <ItemStatsHover item_id={keyname} />
                            </div>
                        }),
                    })
                    .unwrap_or_default(),
            },
        ),
        Instances::Runes(keyname) => base_content(
            ImageType::Runes(*keyname),
            match hover_settings {
                HoverDocs::Full => RUNE_FORMULAS
                    .get(keyname)
                    .and_then(|formula| Some(hover_docs(AttrValue::Static(formula.as_str()), true)))
                    .unwrap_or_default(),
                _ => html!(),
            },
        ),
        Instances::Champions(champion_id) => {
            base_content(ImageType::Champions(champion_id.clone()), html!())
        }
    }
}
