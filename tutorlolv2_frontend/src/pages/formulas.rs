use crate::{
    color,
    components::{ChampionSelector, formulas::source_code::SourceCode},
};
use yew::{Callback, Html, classes, function_component, html, use_callback, use_state};

#[function_component(Formulas)]
pub fn formulas() -> Html {
    let current_index = use_state(|| 0);
    let current_champion = use_state(|| "Aatrox");
    let callback = {
        let current_champion = current_champion.clone();
        use_callback((), move |v, _| {
            current_champion.set(v);
        })
    };

    html! {
        <div
            class={classes!(
                "p-6", "flex-1", "h-screen", "overflow-y-auto",
                "flex", "flex-col", "gap-4",
            )}
        >
            <div class={classes!("flex", "flex-wrap", "gap-4", "items-center")}>
                <div class={classes!("grid", "grid-cols-3", "gap-x-2")}>
                    {
                        [
                            "Champions",
                            "Items",
                            "Runes"
                        ]
                        .into_iter()
                        .enumerate()
                        .map(|(index, value)| html! {
                            <label class={classes!(
                                "px-4", "py-2", color!(text-400), "rounded-md",
                                "hover:bg-[#1d1d25]", "transition-colors",
                                "duration-200", "cursor-pointer",
                                "has-[:checked]:bg-[#1D1D23]", "relative",
                                "has-[:checked]:font-medium",
                                "has-[:checked]:text-white",
                            )}>
                                <input
                                    checked={index == *current_index}
                                    onchange={{
                                        let current_index = current_index.clone();
                                        Callback::from(move |_| {
                                            current_index.set(index);
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
                <ChampionSelector callback={callback.clone()} current_champion={*current_champion} />
            </div>
            <SourceCode champion_id={*current_champion} />
        </div>
    }
}
