use crate::{STATIC_ABILITY_FORMULAS, STATIC_ITEM_FORMULAS, STATIC_RUNE_FORMULAS, color, url};
use yew::{AttrValue, Html, Properties, classes, function_component, html, virtual_dom::VNode};

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

fn hover_docs(formula: AttrValue) -> Html {
    html! {
        <div class={classes!(
            "hidden", "group-hover:flex", "fixed",
            "border", color!(bg-900), "leading-6",
            "transform", "max-w-md",
            "translate-x-[calc(50%-16px)]",
            "translate-y-[calc(50%+20px)]",
            "overflow-auto",
            "max-h-96", "hover-docs",
            color!(border-800), "z-50"
        )}>
            {
                html! {
                    <code class={classes!(
                        "text-[#D4D4D4]", "font-normal",
                        "text-left", "p-2", "text-wrap"
                    )}>
                        { VNode::from_html_unchecked(formula) }
                    </code>
                }
            }
        </div>
    }
}

#[derive(PartialEq)]
pub enum Instances {
    Abilities(String, char, String),
    Items(usize),
    Runes(usize),
    Champions(String),
}

#[derive(Properties, PartialEq)]
pub struct ImageCellProps {
    pub instance: Instances,
}

#[function_component(ImageCell)]
pub fn image_cell(props: &ImageCellProps) -> Html {
    let (img_path, content) = match &props.instance {
        Instances::Abilities(keyname, first_char, champion_id) => match first_char {
            'A' => (
                url!("/cdn/abilities/{}.png", first_char),
                html! { hover_docs(BASIC_ATTACK_FORMULA.into()) },
            ),
            'C' => (
                url!("/cdn/abilities/{}.png", first_char),
                html! { hover_docs(CRITICAL_STRIKE_FORMULA.into()) },
            ),
            _ => (
                url!("/cdn/abilities/{}{}.png", champion_id, first_char),
                html! {
                    <>
                        <span class={classes!("img-letter")}>
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
                        {
                            hover_docs(
                                STATIC_ABILITY_FORMULAS
                                    .get()
                                    .unwrap()
                                    .get(champion_id)
                                    .unwrap()
                                    .get(keyname)
                                    .unwrap()
                                    .as_str()
                                    .into(),
                            )
                        }
                    </>
                },
            ),
        },
        Instances::Items(keyname) => (
            url!("/cdn/items/{}.png", keyname),
            html! {
                hover_docs(
                    STATIC_ITEM_FORMULAS
                        .get()
                        .unwrap()
                        .get(keyname)
                        .unwrap()
                        .as_str()
                        .into(),
                )
            },
        ),
        Instances::Runes(keyname) => (
            url!("/cdn/runes/{}.png", keyname),
            html! {
                hover_docs(
                    STATIC_RUNE_FORMULAS
                        .get()
                        .unwrap()
                        .get(keyname)
                        .unwrap()
                        .as_str()
                        .into(),
                )
            },
        ),
        Instances::Champions(champion_id) => (url!("/cdn/champions/{}.png", champion_id), html!()),
    };

    html! {
        <>
            <div class={classes!(
                "flex", "items-center", "justify-center",
                "relative", "cell"
            )}>
                <img
                    class={classes!(
                        "w-8", "h-8",
                    )}
                    src={img_path}
                    alt={""}
                />
                { content }
            </div>
        </>
    }
}
