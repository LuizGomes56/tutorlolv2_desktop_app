use crate::{
    components::{Image, ImageType},
    url,
    utils::{BytesExt, StringExt},
};
use generated_code::CHAMPION_ABILITIES;
use std::rc::Rc;
use yew::{
    AttrValue, Callback, Html, Properties, Reducible, classes, function_component, html,
    use_callback, use_effect_with, use_memo, use_reducer,
};

#[function_component(DamageStackTable)]
pub fn damage_stack_table() -> Html {
    html! {}
}

const ABILITY_STR_SIZE: usize = 15;

#[derive(Clone, Copy, PartialEq)]
enum StackValue {
    Ability([u8; ABILITY_STR_SIZE]),
    Item(u32),
    Rune(u32),
    BasicAttack,
    CriticalStrike,
    Ignite,
}

enum StackAction {
    Push(StackValue),
    Remove(usize),
}

#[derive(Clone, PartialEq, Default)]
struct Stack(Vec<StackValue>);

impl Stack {
    fn push(&mut self, value: StackValue) {
        self.0.push(value);
    }

    fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

impl Reducible for Stack {
    type Action = StackAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            StackAction::Push(value) => {
                new_state.push(value);
            }
            StackAction::Remove(index) => {
                new_state.remove(index);
            }
        }
        Rc::new(new_state)
    }
}

impl std::fmt::Debug for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::Ability(bytes) => {
                write!(f, "Ability(\"{}\")", bytes.to_str())
            }
            StackValue::Item(val) => write!(f, "Item({})", val),
            StackValue::Rune(val) => write!(f, "Rune({})", val),
            StackValue::BasicAttack => write!(f, "BasicAttack"),
            StackValue::CriticalStrike => write!(f, "CriticalStrike"),
            StackValue::Ignite => write!(f, "Ignite"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DamageStackSelectorProps {
    pub champion_id: AttrValue,
    pub items: Vec<u32>,
    pub runes: Vec<u32>,
}

#[function_component(DamageStackSelector)]
pub fn damage_stack_selector(props: &DamageStackSelectorProps) -> Html {
    let damage_stack = use_reducer(Stack::default);

    use_effect_with(damage_stack.clone(), move |damage_stack| {
        web_sys::console::log_1(&format!("{:#?}", damage_stack.0).into());
    });

    let push_callback = {
        let damage_stack = damage_stack.clone();
        use_callback((), move |v, _| {
            damage_stack.dispatch(StackAction::Push(v));
        })
    };

    let remove_callback = {
        let damage_stack = damage_stack.clone();
        use_callback((), move |v, _| {
            damage_stack.dispatch(StackAction::Remove(v));
        })
    };

    let base_content = |img_path, onclick, content| {
        html! {
            <div
                onclick={onclick}
                class={classes!(
                    "flex", "items-center", "justify-center", "relative",
                )}
            >
                <Image class={classes!("w-8", "h-8")} source={img_path} />
                { content }
            </div>
        }
    };

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
            </>
        }
    });

    let abilities_memo = use_memo(
        (props.champion_id.clone(), push_callback.clone()),
        move |(champion_id, push_callback)| {
            html! {
                <>
                    {
                        CHAMPION_ABILITIES
                            .get(champion_id)
                            .and_then(|value| {
                                Some(
                                    value
                                        .keys()
                                        .map(|ability_name| {
                                            let first_char = ability_name.first_char();
                                            base_content(
                                                ImageType::Abilities(
                                                    props.champion_id.as_str().concat_char(first_char),
                                                ),
                                                {
                                                    let push_callback = push_callback.clone();
                                                    Callback::from(move |_| {
                                                        push_callback.emit(StackValue::Ability(
                                                            ability_name
                                                                .to_sized_slice::<ABILITY_STR_SIZE>(),
                                                        ));
                                                    })
                                                },
                                                Some(html! {
                                                    <span class={classes!("text-sm", "img-letter")}>
                                                        {first_char}
                                                        <sub>{ ability_name.padding_chars() }</sub>
                                                    </span>
                                                }),
                                            )
                                        })
                                        .collect::<Html>(),
                                )
                            })
                            .unwrap_or_default()
                    }
                    {
                        props
                            .items
                            .iter()
                            .map(|item_id| {
                                base_content(
                                    ImageType::Items(*item_id),
                                    {
                                        let push_callback = push_callback.clone();
                                        let item_id = *item_id;
                                        Callback::from(move |_| {
                                            push_callback.emit(StackValue::Item(item_id))
                                        })
                                    },
                                    None,
                                )
                            })
                            .collect::<Html>()
                    }
                    {
                        props
                            .runes
                            .iter()
                            .map(|rune_id| {
                                base_content(
                                    ImageType::Runes(*rune_id),
                                    {
                                        let push_callback = push_callback.clone();
                                        let rune_id = *rune_id;
                                        Callback::from(move |_| {
                                            push_callback.emit(StackValue::Rune(rune_id))
                                        })
                                    },
                                    None,
                                )
                            })
                            .collect::<Html>()
                    }
                </>
            }
        },
    );

    html! {
        <div class={classes!("flex", "flex-col", "gap-2")}>
            {(*abilities_memo).clone()}
            {(*attack_memo).clone()}
        </div>
    }
}

#[function_component(DamageStackSelected)]
pub fn damage_stack_selected() -> Html {
    html! {}
}
