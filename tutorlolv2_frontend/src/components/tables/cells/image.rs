use crate::{
    build_imports::{CHAMPION_ABILITIES, ITEM_FORMULAS, RUNE_FORMULAS},
    color,
    components::{
        Image, ImageType,
        hover::{docs::hover_docs, item_stats::ItemStatsHover},
    },
    context::{HoverDocs, SettingsContext},
    url,
};
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
    Abilities(String, char, AttrValue),
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

    let (img_path, content) = match &props.instance {
        Instances::Abilities(keyname, first_char, champion_id) => match first_char {
            'A' => (
                ImageType::Other(url!("/img/other/basic_attack.png").to_string()),
                match hover_settings {
                    HoverDocs::Full => html! { hover_docs(BASIC_ATTACK_FORMULA.into(), true) },
                    _ => html!(),
                },
            ),
            'C' => (
                ImageType::Other(url!("/img/stats/crit_chance.svg").to_string()),
                match hover_settings {
                    HoverDocs::Full => html! { hover_docs(CRITICAL_STRIKE_FORMULA.into(), true) },
                    _ => html!(),
                },
            ),
            _ => {
                let hover_provider = {
                    match hover_settings {
                        HoverDocs::Full => CHAMPION_ABILITIES
                            .get(&champion_id)
                            .and_then(|&phf_formula_map| {
                                phf_formula_map.get(keyname).and_then(|&formula| {
                                    match hover_settings {
                                        HoverDocs::Full => {
                                            Some(hover_docs(AttrValue::Static(formula), true))
                                        }
                                        _ => None,
                                    }
                                })
                            })
                            .unwrap_or_default(),
                        _ => html!(),
                    }
                };
                (
                    ImageType::Abilities(format!("{}{}", champion_id, first_char)),
                    html! {
                        <>
                            <span class={classes!("text-[13px]", "img-letter")}>
                                {first_char}
                                <sub>
                                    {
                                        keyname
                                            .chars()
                                            .filter(|c| *c != '_')
                                            .skip(1)
                                            .take(3)
                                            .collect::<String>()
                                    }
                                </sub>
                            </span>
                            {hover_provider}
                        </>
                    },
                )
            }
        },
        Instances::Items(keyname) => (
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
                                {hover_docs(AttrValue::Static(formula), false)}
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
        Instances::Runes(keyname) => (
            ImageType::Other(url!("/img/runes/{}.avif", keyname)),
            match hover_settings {
                HoverDocs::Full => RUNE_FORMULAS
                    .get(keyname)
                    .and_then(|formula| Some(hover_docs(AttrValue::Static(formula), true)))
                    .unwrap_or_default(),
                _ => html!(),
            },
        ),
        Instances::Champions(champion_id) => (
            ImageType::Other(url!("/img/champions/{}.avif", champion_id)),
            html!(),
        ),
    };

    html! {
        <>
            <div class={classes!(
                "flex", "items-center", "justify-center",
                "relative", "cell"
            )}>
                <Image size={28} source={img_path} />
                { content }
            </div>
        </>
    }
}
