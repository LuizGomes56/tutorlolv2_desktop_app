use crate::{
    components::{Image, ImageType, calculator::StackValue, tables::cells::image::label_html},
    svg, url,
};
use tutorlolv2_imports::{
    BASIC_ATTACK_OFFSET, CHAMPION_ABILITIES, CRITICAL_STRIKE_OFFSET, ChampionId, ITEM_FORMULAS,
    ItemId, ONHIT_EFFECT_OFFSET, RUNE_FORMULAS, RuneId,
};
use yew::{
    AttrValue, Callback, Html, MouseEvent, Properties, classes, function_component, html, use_memo,
};

fn base_content(
    img_path: ImageType,
    offsets: Option<&'static (u32, u32)>,
    onclick: Callback<MouseEvent>,
    content: Option<Html>,
) -> Html {
    html! {
        <div
            data-classes={offsets.map(|_| "cursor-default")}
            data-offset={offsets.map(|(s, e)| format!("{s},{e}"))}
            onclick={onclick}
            class={classes!(
                "flex", "items-center", "justify-center", "relative",
                "cursor-pointer", "select-none",
            )}
        >
            <Image class={classes!("w-8", "h-8")} source={img_path} />
            { content }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DamageStackTableProps {
    pub damages: Html,
}

#[function_component(DamageStackTable)]
pub fn damage_stack_table(props: &DamageStackTableProps) -> Html {
    let header_memo = use_memo((), move |_| {
        html! {
            <thead>
                <tr>
                    <th></th>
                    <th class={classes!("h-10", "justify-items-center")}>
                        {svg!("../../../public/svgs/sigma", "24")}
                    </th>
                    <th class={classes!("h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-6", "h-6")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/stats/health.svg"))) }
                        />
                    </th>
                    <th class={classes!("h-10", "justify-items-center")}>
                        {svg!("../../../public/svgs/weakness", "24")}
                    </th>
                </tr>
            </thead>
        }
    });

    html! {
        <table class={classes!("h-fit")}>
            {(*header_memo).clone()}
            <tbody>
                {props.damages.clone()}
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct DamageStackSelectorProps {
    pub champion_id: ChampionId,
    pub items: Box<[ItemId]>,
    pub runes: Box<[RuneId]>,
    pub stack: Vec<StackValue>,
    pub push_callback: Callback<StackValue>,
    pub remove_callback: Callback<u16>,
    pub damages: Html,
}

#[function_component(DamageStackSelector)]
pub fn damage_stack_selector(props: &DamageStackSelectorProps) -> Html {
    html! {
        <div class={classes!(
            "grid", "grid-cols-[auto_1fr]", "gap-x-4",
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-4"
            )}>
                <InsertDamageStackSelector
                    champion_id={props.champion_id}
                    items={props.items.clone()}
                    runes={props.runes.clone()}
                    push_callback={props.push_callback.clone()}
                />
                <RemoveDamageStackSelector
                    champion_id={props.champion_id}
                    stack={props.stack.clone()}
                    remove_callback={props.remove_callback.clone()}
                />
            </div>
            <DamageStackTable damages={props.damages.clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RemoveDamageStackSelectorProps {
    champion_id: ChampionId,
    stack: Vec<StackValue>,
    remove_callback: Callback<u16>,
}

#[function_component(RemoveDamageStackSelector)]
fn remove_damage_stack_selector(props: &RemoveDamageStackSelectorProps) -> Html {
    let remove_callback = &props.remove_callback;

    html! {
        <div class={classes!("flex", "flex-wrap", "gap-2")}>
            {
                props
                    .stack
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        let image_url = match value {
                            StackValue::Ability(name) => ImageType::Ability(
                                props.champion_id,
                                *name,
                            ),
                            StackValue::Item(val) => ImageType::Item(*val),
                            StackValue::Rune(val) => ImageType::Rune(*val),
                            StackValue::BasicAttack => {
                                ImageType::Other(AttrValue::Static(url!("/img/other/basic_attack.png")))
                            }
                            StackValue::CriticalStrike => {
                                ImageType::Other(AttrValue::Static(url!("/img/stats/crit_chance.svg")))
                            }
                            StackValue::Onhit => {
                                ImageType::Other(AttrValue::Static(url!("/img/stats/onhit.svg")))
                            }
                            StackValue::Ignite => {
                                ImageType::Other(AttrValue::Static(url!("/img/other/ignite.avif")))
                            }
                        };
                        base_content(
                            image_url,
                            None,
                            {
                                let remove_callback = remove_callback.clone();
                                Callback::from(move |_| {
                                    remove_callback.emit(index as u16);
                                })
                            },
                            None,
                        )
                    })
                    .collect::<Html>()
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct InsertDamageStackSelectorProps {
    champion_id: ChampionId,
    items: Box<[ItemId]>,
    runes: Box<[RuneId]>,
    push_callback: Callback<StackValue>,
}

#[function_component(InsertDamageStackSelector)]
fn insert_damage_stack_selector(props: &InsertDamageStackSelectorProps) -> Html {
    let push_callback = &props.push_callback;

    let attack_memo = use_memo(push_callback.clone(), move |push_callback| {
        html! {
            <>
                {base_content(
                    ImageType::Other(AttrValue::Static(url!("/img/other/basic_attack.png"))),
                    Some(&BASIC_ATTACK_OFFSET),
                    {
                        let push_callback = push_callback.clone();
                        Callback::from(move |_| {
                            push_callback.emit(StackValue::BasicAttack)
                        })
                    },
                    None,
                )}
                {base_content(
                    ImageType::Other(AttrValue::Static(url!("/img/stats/crit_chance.svg"))),
                    Some(&CRITICAL_STRIKE_OFFSET),
                    {
                        let push_callback = push_callback.clone();
                        Callback::from(move |_| {
                            push_callback.emit(StackValue::CriticalStrike)
                        })
                    },
                    None,
                )}
                {base_content(
                    ImageType::Other(AttrValue::Static(url!("/img/stats/onhit.svg"))),
                    Some(&ONHIT_EFFECT_OFFSET),
                    {
                        let push_callback = push_callback.clone();
                        Callback::from(move |_| {
                            push_callback.emit(StackValue::Onhit)
                        })
                    },
                    None,
                )}
            </>
        }
    });

    let items_memo = use_memo(
        (props.items.clone(), push_callback.clone()),
        move |(items, push_callback)| {
            items
                .iter()
                .map(|item_id| {
                    base_content(
                        ImageType::Item(*item_id),
                        ITEM_FORMULAS.get(*item_id as usize),
                        {
                            let push_callback = push_callback.clone();
                            let item_id = *item_id;
                            Callback::from(move |_| push_callback.emit(StackValue::Item(item_id)))
                        },
                        None,
                    )
                })
                .collect::<Html>()
        },
    );

    let runes_memo = use_memo(
        (props.runes.clone(), push_callback.clone()),
        move |(runes, push_callback)| {
            runes
                .iter()
                .map(|rune_id| {
                    base_content(
                        ImageType::Rune(*rune_id),
                        RUNE_FORMULAS.get(*rune_id as usize),
                        {
                            let push_callback = push_callback.clone();
                            let rune_id = *rune_id;
                            Callback::from(move |_| push_callback.emit(StackValue::Rune(rune_id)))
                        },
                        None,
                    )
                })
                .collect::<Html>()
        },
    );

    let abilities_memo = use_memo(
        (props.champion_id, push_callback.clone()),
        move |(champion_id, push_callback)| {
            CHAMPION_ABILITIES
                .get(*champion_id as usize)
                .map(|value| {
                    value
                        .iter()
                        .map(|(ability_name, offset)| {
                            base_content(
                                ImageType::Ability(props.champion_id, *ability_name),
                                Some(offset),
                                {
                                    let push_callback = push_callback.clone();
                                    Callback::from(move |_| {
                                        push_callback.emit(StackValue::Ability(*ability_name));
                                    })
                                },
                                Some(label_html(ability_name)),
                            )
                        })
                        .collect::<Html>()
                })
                .unwrap_or_default()
        },
    );

    html! {
        <div id={"i_stack"} class={classes!("flex", "flex-wrap", "gap-2")}>
            {(*attack_memo).clone()}
            {(*abilities_memo).clone()}
            {(*items_memo).clone()}
            {(*runes_memo).clone()}
        </div>
    }
}
