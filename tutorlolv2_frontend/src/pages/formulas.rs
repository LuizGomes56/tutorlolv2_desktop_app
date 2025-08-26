use crate::{
    components::Selector,
    utils::{ComptimeCache, RandomInput},
};
use generated_code::{
    CHAMPION_FORMULAS, CHAMPION_GENERATOR, ChampionId, ITEM_FORMULAS, ItemId, RUNE_FORMULAS, RuneId,
};
use yew::{
    AttrValue, Callback, Html, Properties, classes, function_component, html, use_callback,
    use_state,
};

#[derive(Properties, PartialEq)]
pub struct SourceCodeProps {
    pub offset: Option<&'static (u32, u32)>,
}

#[function_component(SourceCode)]
pub fn source_code(props: &SourceCodeProps) -> Html {
    if let Some(offset) = props.offset {
        html! {
            <code class={classes!(
                "text-[#D4D4D4]", "text-left",
                "text-wrap", "break-all"
            )}>
                { Html::from_html_unchecked(AttrValue::Static(offset.as_str())) }
            </code>
        }
    } else {
        html! {}
    }
}

#[derive(Clone, Copy)]
enum FormulaDropdown {
    Champions,
    Items,
    Runes,
    Generator,
}

impl FormulaDropdown {
    fn from_index(value: usize) -> Self {
        match value {
            0 => Self::Champions,
            1 => Self::Items,
            2 => Self::Runes,
            _ => Self::Generator,
        }
    }

    fn to_array() -> [&'static str; 4] {
        ["Champions", "Items", "Runes", "Generator"]
    }
}

#[function_component(Formulas)]
pub fn formulas() -> Html {
    let current_dropdown_id =
        use_state(|| FormulaDropdown::from_index(RandomInput::rand_u8(4) as usize));
    let current_champion = use_state(|| RandomInput::champion_id());
    let current_item = use_state(|| RandomInput::item_id());
    let current_rune = use_state(|| RandomInput::rune_id());
    let champion_callback = {
        let current_champion = current_champion.clone();
        use_callback((), move |v, _| {
            current_champion.set(v);
        })
    };
    let item_callback = {
        let current_item = current_item.clone();
        use_callback((), move |v, _| {
            current_item.set(v);
        })
    };
    let rune_callback = {
        let current_rune = current_rune.clone();
        use_callback((), move |v, _| {
            current_rune.set(v);
        })
    };

    html! {
        <div
            class={classes!(
                "p-6", "flex-1", "flex", "flex-col", "gap-4",
            )}
        >
            <div class={classes!("flex", "flex-wrap", "gap-2", "items-center")}>
                <div class={classes!("grid", "grid-cols-4", "gap-x-2")}>
                    {
                        FormulaDropdown::to_array()
                        .into_iter()
                        .enumerate()
                        .map(|(index, value)| {
                            let random_id = RandomInput::rand_id();
                            html! {
                                <label for={&random_id} class={classes!(
                                    "px-4", "py-2", "_text-400", "rounded-md",
                                    "hover:bg-[#1d1d25]", "transition-colors",
                                    "duration-200", "cursor-pointer",
                                    "has-[:checked]:bg-[#1D1D23]", "relative",
                                    "text-center", "has-[:checked]:text-white",
                                    "hover:text-[#c3c3c3]",
                                )}>
                                    <input
                                        id={random_id}
                                        checked={index == *current_dropdown_id as usize}
                                        onchange={{
                                            let current_dropdown_id = current_dropdown_id.clone();
                                            Callback::from(move |_| {
                                                current_dropdown_id.set(FormulaDropdown::from_index(index));
                                            })
                                        }}
                                        type={"radio"}
                                        name={"formula_dropdown"}
                                        class={classes!(
                                            "appearance-none", "absolute"
                                        )}
                                    />
                                    <span>
                                        {value}
                                    </span>
                                </label>
                            }
                        })
                        .collect::<Html>()
                    }
                </div>
                {
                    match *current_dropdown_id {
                        FormulaDropdown::Champions | FormulaDropdown::Generator => html! {
                            <Selector<ChampionId>
                                callback={champion_callback.clone()}
                                current_value={*current_champion}
                            />
                        },
                        FormulaDropdown::Items => html! {
                            <Selector<ItemId>
                                callback={item_callback.clone()}
                                current_value={*current_item}
                            />
                        },
                        FormulaDropdown::Runes => html! {
                            <Selector<RuneId>
                                callback={rune_callback.clone()}
                                current_value={*current_rune}
                            />
                        },
                    }
                }
                </div>
            {
                match *current_dropdown_id {
                    FormulaDropdown::Champions => html! {
                        <SourceCode offset={CHAMPION_FORMULAS.get(*current_champion as usize)} />
                    },
                    FormulaDropdown::Items => html! {
                        <SourceCode offset={ITEM_FORMULAS.get(*current_item as usize)} />
                    },
                    FormulaDropdown::Runes => html! {
                        <SourceCode offset={RUNE_FORMULAS.get(*current_rune as usize)} />
                    },
                    FormulaDropdown::Generator => html! {
                        <SourceCode offset={CHAMPION_GENERATOR.get(*current_champion as usize)} />
                    },
                }
            }
        </div>
    }
}
