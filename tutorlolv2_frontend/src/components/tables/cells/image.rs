use crate::{
    color,
    components::{
        Image, ImageType,
        hover::{docs::hover_docs, item_stats::ItemStatsHover},
    },
    context::{HoverDocs, SettingsContext},
    url,
    utils::{FromBrotliBytes, StringExt},
};
use generated_code::{CHAMPION_ABILITIES, ITEM_FORMULAS, RUNE_FORMULAS};
use yew::{AttrValue, Html, Properties, classes, function_component, html, use_context};

const BASIC_ATTACK_FORMULA: &'static str = r#"<pre><span class="control">intrinsic</span> <span class="constant">BASIC_ATTACK</span><span class="punctuation"> = {
    <span class="variable">name</span><span class="punctuation">: </span><span class="string">"Basic Attack"</span>,
    <span class="variable">damage_type</span><span class="punctuation">: </span><span class="string">"PHYSICAL_DAMAGE"</span>,
    <span class="variable">minimum_damage</span><span class="punctuation">: </span><span class="punctuation">|</span>_<span class="punctuation">, </span><span class="variable">ctx</span>|</span> {
        <span class="variable">ctx.ad</span> * <span class="variable">ctx.physical_multiplier</span>
    },
    <span class="variable">maximum_damage</span><span class="punctuation">: </span>|_, _| <span class="float">0.0</span>,
};</pre>"#;

const CRITICAL_STRIKE_FORMULA: &'static str = r#"<pre><span class="control">intrinsic</span> <span class="constant">CRITICAL_STRIKE</span><span class="punctuation"> = {
    <span class="variable">name</span><span class="punctuation">: </span><span class="string">"Critical Strike"</span>,
    <span class="variable">damage_type</span><span class="punctuation">: </span><span class="string">"PHYSICAL_DAMAGE"</span>,
    <span class="variable">minimum_damage</span><span class="punctuation">: </span><span class="punctuation">|</span>_<span class="punctuation">, </span><span class="variable">ctx</span>|</span> {
        <span class="variable">ctx.ad</span>
        <span class="punctuation"> * </span><span class="variable">ctx.physical_multiplier</span>
        <span class="punctuation"> * </span><span class="variable">ctx.crit_damage</span>
        <span class="punctuation"> / </span><span class="float">100.0</span>
    },
    <span class="variable">maximum_damage</span><span class="punctuation">: </span>|_, _| <span class="float">0.0</span>,
};</pre>"#;

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
                                        HoverDocs::Full => html! { hover_docs(BASIC_ATTACK_FORMULA.into(), true) },
                                        _ => html!(),
                                    },
                                ))
                            ))
                            .chain(std::iter::once(
                                chain_th(base_content(
                                    ImageType::Other(AttrValue::Static(url!("/img/stats/crit_chance.svg"))),
                                    match hover_settings {
                                        HoverDocs::Full => html! { hover_docs(CRITICAL_STRIKE_FORMULA.into(), true) },
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
                                "flex-col", "flex", "fixed",
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
                                "group-hover:flex", "flex-col", "fixed", "hover-docs", "hidden",
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
