use crate::{
    components::{Image, ImageType, calculator::StackValue},
    svg, url,
};
use generated_code::{CHAMPION_ABILITIES, ChampionId, ItemId, RuneId};
use yew::{
    AttrValue, Callback, Html, MouseEvent, Properties, classes, function_component, html, use_memo,
};

fn base_content(img_path: ImageType, onclick: Callback<MouseEvent>, content: Option<Html>) -> Html {
    html! {
        <div
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
                    <th class={classes!("min-w-10")}></th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        {svg!("../../../public/svgs/sigma", "24")}
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-6", "h-6")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/stats/health.svg"))) }
                        />
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
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
    pub items: Vec<ItemId>,
    pub runes: Vec<RuneId>,
    pub stack: Vec<StackValue>,
    pub push_callback: Callback<StackValue>,
    pub remove_callback: Callback<usize>,
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
    remove_callback: Callback<usize>,
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
                            StackValue::Ability(name) => ImageType::Abilities(
                                props.champion_id,
                                *name,
                            ),
                            StackValue::Item(val) => ImageType::Items(*val),
                            StackValue::Rune(val) => ImageType::Runes(*val),
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
                            {
                                let remove_callback = remove_callback.clone();
                                Callback::from(move |_| {
                                    remove_callback.emit(index);
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
    items: Vec<ItemId>,
    runes: Vec<RuneId>,
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
                        ImageType::Items(*item_id),
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
                        ImageType::Runes(*rune_id),
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
        (props.champion_id.clone(), push_callback.clone()),
        move |(champion_id, push_callback)| {
            CHAMPION_ABILITIES
                .get(*champion_id as usize)
                .and_then(|value| {
                    Some(
                        value
                            .into_iter()
                            .map(|(ability_name, _)| {
                                let first_char = ability_name.as_char();
                                base_content(
                                    ImageType::Abilities(props.champion_id, *ability_name),
                                    {
                                        let push_callback = push_callback.clone();
                                        Callback::from(move |_| {
                                            push_callback.emit(StackValue::Ability(*ability_name));
                                        })
                                    },
                                    Some(html! {
                                        <span class={classes!("text-sm", "img-letter")}>
                                            {first_char}
                                            // <sub>{ ability_name.padding_chars() }</sub>
                                        </span>
                                    }),
                                )
                            })
                            .collect::<Html>(),
                    )
                })
                .unwrap_or_default()
        },
    );

    html! {
        <div class={classes!("flex", "flex-wrap", "gap-2")}>
            {(*abilities_memo).clone()}
            {(*attack_memo).clone()}
            {(*items_memo).clone()}
            {(*runes_memo).clone()}
        </div>
    }
}
