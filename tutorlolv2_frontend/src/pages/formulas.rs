use crate::{
    color,
    components::{
        ChampionSelector, U32Selector, calculator::StaticIterator,
        formulas::source_code::SourceCode,
    },
};
use generated_code::{CHAMPION_FORMULAS, CHAMPION_GENERATOR, ITEM_FORMULAS, RUNE_FORMULAS};
use yew::{Callback, Html, classes, function_component, html, use_callback, use_state};

#[derive(Clone, Copy)]
enum FormulaDropdown {
    Champions = 0,
    Items = 1,
    Runes = 2,
    Generator = 3,
}

impl From<usize> for FormulaDropdown {
    fn from(value: usize) -> Self {
        match value {
            0 => FormulaDropdown::Champions,
            1 => FormulaDropdown::Items,
            2 => FormulaDropdown::Runes,
            3 => FormulaDropdown::Generator,
            _ => FormulaDropdown::Champions,
        }
    }
}

#[function_component(Formulas)]
pub fn formulas() -> Html {
    let current_dropdown_id = use_state(|| FormulaDropdown::Champions);
    let current_champion = use_state(|| "Aatrox");
    let current_item = use_state(|| 3115u32);
    let current_rune = use_state(|| 8112u32);
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
                        [
                            "Champions",
                            "Items",
                            "Runes",
                            "Generator",
                        ]
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
                                            current_dropdown_id.set(FormulaDropdown::from(index));
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
                            <ChampionSelector
                                callback={champion_callback.clone()}
                                current_champion={*current_champion}
                            />
                        },
                        FormulaDropdown::Items => html! {
                            <U32Selector
                                static_iter={StaticIterator::Items}
                                callback={item_callback.clone()}
                                current_value={*current_item}
                            />
                        },
                        FormulaDropdown::Runes => html! {
                            <U32Selector
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
                        <SourceCode offset={CHAMPION_FORMULAS.get(*current_champion)} />
                    },
                    FormulaDropdown::Items => html! {
                        <SourceCode offset={ITEM_FORMULAS.get(&*current_item)} />
                    },
                    FormulaDropdown::Runes => html! {
                        <SourceCode offset={RUNE_FORMULAS.get(&*current_rune)} />
                    },
                    FormulaDropdown::Generator => html! {
                        <SourceCode offset={CHAMPION_GENERATOR.get(*current_champion)} />
                    },
                }
            }
        </div>
    }
}
