use crate::{
    STATIC_ABILITY_FORMULAS, STATIC_ITEM_FORMULAS, STATIC_RUNE_FORMULAS, color,
    components::hover::{docs::hover_docs, item_stats::ItemStatsHover},
    context::{HoverDocs, SettingsContext},
    url,
};
use yew::{AttrValue, Html, Properties, classes, function_component, html, use_context};

const BASIC_ATTACK_FORMULA: &'static str = r#"<pre><span class="control">intrinsic</span> <span class="constant">BASIC_ATTACK</span><span class="punctuation"> = {
    <span class="variable">name</span><span class="punctuation">: </span><span class="string">"Basic Attack"</span>,
    <span class="variable">damage_type</span><span class="punctuation">: </span><span class="string">"PHYSICAL_DAMAGE"</span>,
    <span class="variable">minimum_damage</span><span class="punctuation">: </span><span class="punctuation">|</span>_<span class="punctuation">, </span><span class="variable">ctx</span><span class="punctuation">: </span>&amp;<span class="type">EvalContext</span><span class="punctuation">|</span> {
        <span class="variable">ctx.ad</span> * <span class="variable">ctx.physical_multiplier</span>
    },
    <span class="variable">maximum_damage</span><span class="punctuation">: </span>|_, _| <span class="float">0.0f64</span>,
};</pre>"#;

const CRITICAL_STRIKE_FORMULA: &'static str = r#"<pre><span class="control">intrinsic</span> <span class="constant">CRITICAL_STRIKE</span><span class="punctuation"> = {
    <span class="variable">name</span><span class="punctuation">: </span><span class="string">"Critical Strike"</span>,
    <span class="variable">damage_type</span><span class="punctuation">: </span><span class="string">"PHYSICAL_DAMAGE"</span>,
    <span class="variable">minimum_damage</span><span class="punctuation">: </span><span class="punctuation">|</span>_<span class="punctuation">, </span><span class="variable">ctx</span><span class="punctuation">: </span>&amp;<span class="type">EvalContext</span><span class="punctuation">|</span> {
        <span class="variable">ctx.ad</span>
        <span class="punctuation"> * </span><span class="variable">ctx.physical_multiplier</span>
        <span class="punctuation"> * </span><span class="variable">ctx.crit_damage</span>
        <span class="punctuation"> / </span><span class="float">100.0f64</span>
    },
    <span class="variable">maximum_damage</span><span class="punctuation">: </span>|_, _| <span class="float">0.0f64</span>,
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
                url!("/img/other/basic_attack.png").to_string(),
                match hover_settings {
                    HoverDocs::Full => html! { hover_docs(BASIC_ATTACK_FORMULA.into(), true) },
                    _ => html!(),
                },
            ),
            'C' => (
                url!("/img/stats/crit_chance.svg").to_string(),
                match hover_settings {
                    HoverDocs::Full => html! { hover_docs(CRITICAL_STRIKE_FORMULA.into(), true) },
                    _ => html!(),
                },
            ),
            _ => {
                let hover_provider = match hover_settings {
                    HoverDocs::Full => STATIC_ABILITY_FORMULAS
                        .get()
                        .and_then(|map| map.get(&champion_id.to_string()))
                        .and_then(|champ_map| champ_map.get(keyname))
                        .map(|formula| hover_docs(formula.as_str().into(), true))
                        .unwrap_or_default(),
                    _ => html!(),
                };
                (
                    url!("/img/abilities/{}{}.avif", champion_id, first_char),
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
            url!("/img/items/{}.avif", keyname),
            STATIC_ITEM_FORMULAS
                .get()
                .and_then(|map| map.get(keyname))
                .map(|formula| match hover_settings {
                    HoverDocs::Full | HoverDocs::Partial => html! {
                        <div class={classes!(
                            "hidden", "group-hover:flex", "flex-col",
                            "fixed", "translate-x-[calc(50%-16px)]",
                            "translate-y-[calc(50%+20px)]", "z-50",
                            "py-3", "border", color!(border-800), "gap-y-2",
                            "overflow-auto", "max-h-96", "px-3.5", color!(bg-900),
                            "hover-docs"
                        )}>
                            {
                                match hover_settings {
                                    HoverDocs::Full => {
                                        html! {
                                            <>
                                                <ItemStatsHover item_id={keyname} />
                                                {hover_docs(formula.as_str().into(), false)}
                                            </>
                                        }
                                    },
                                    _ => {
                                        html! { <ItemStatsHover item_id={keyname} /> }
                                    },
                                }
                            }
                        </div>
                    },
                    _ => html!(),
                })
                .unwrap_or_default(),
        ),
        Instances::Runes(keyname) => (
            url!("/img/runes/{}.avif", keyname),
            match hover_settings {
                HoverDocs::Full => STATIC_RUNE_FORMULAS
                    .get()
                    .and_then(|map| map.get(keyname))
                    .map(|formula| hover_docs(formula.as_str().into(), true))
                    .unwrap_or_default(),
                _ => html!(),
            },
        ),
        Instances::Champions(champion_id) => (url!("/img/champions/{}.avif", champion_id), html!()),
    };

    html! {
        <>
            <div class={classes!(
                "flex", "items-center", "justify-center",
                "relative", "cell"
            )}>
                <img
                    loading={"lazy"}
                    class={classes!(
                        "w-7", "h-7",
                    )}
                    src={img_path}
                    alt={""}
                />
                { content }
            </div>
        </>
    }
}
