use crate::{
    color,
    components::{Selector, calculator::StaticIterator, formulas::source_code::SourceCode},
};
use generated_code::{
    CHAMPION_FORMULAS, CHAMPION_GENERATOR, ChampionId, ITEM_FORMULAS, ItemId, RUNE_FORMULAS, RuneId,
};
use yew::{Callback, Html, classes, function_component, html, use_callback, use_state};

#[derive(Clone, Copy)]
enum FormulaDropdown {
    Champions,
    Items,
    Runes,
    Generator,
}

impl FormulaDropdown {
    fn unsafe_cast(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    fn to_array() -> [&'static str; 4] {
        ["Champions", "Items", "Runes", "Generator"]
    }
}

#[function_component(Formulas)]
pub fn formulas() -> Html {
    let current_dropdown_id = use_state(|| FormulaDropdown::Champions);
    let current_champion = use_state(|| ChampionId::Aatrox);
    let current_item = use_state(|| ItemId::NashorsTooth);
    let current_rune = use_state(|| RuneId::Electrocute);
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
                        .map(|(index, value)| html! {
                            <label class={classes!(
                                "px-4", "py-2", color!(text-400), "rounded-md",
                                "hover:bg-[#1d1d25]", "transition-colors",
                                "duration-200", "cursor-pointer",
                                "has-[:checked]:bg-[#1D1D23]", "relative",
                                "text-center", "has-[:checked]:text-white",
                                "hover:text-[#c3c3c3]",
                            )}>
                                <input
                                    checked={index == *current_dropdown_id as usize}
                                    onchange={{
                                        let current_dropdown_id = current_dropdown_id.clone();
                                        Callback::from(move |_| {
                                            current_dropdown_id.set(FormulaDropdown::unsafe_cast(index as u8));
                                        })
                                    }}
                                    type={"radio"}
                                    name={"formula_dropdown"}
                                    class={classes!(
                                        "appearance-none", "absolute", "peer"
                                    )}
                                />
                                <span>
                                    {value}
                                </span>
                            </label>
                        })
                        .collect::<Html>()
                    }
                </div>
                {
                    match *current_dropdown_id {
                        FormulaDropdown::Champions | FormulaDropdown::Generator => html! {
                            <Selector<ChampionId>
                                static_iter={StaticIterator::Champions}
                                callback={champion_callback.clone()}
                                current_value={*current_champion}
                            />
                        },
                        FormulaDropdown::Items => html! {
                            <Selector<ItemId>
                                static_iter={StaticIterator::Items}
                                callback={item_callback.clone()}
                                current_value={*current_item}
                            />
                        },
                        FormulaDropdown::Runes => html! {
                            <Selector<RuneId>
                                static_iter={StaticIterator::Runes}
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
