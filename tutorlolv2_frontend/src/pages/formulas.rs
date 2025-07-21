use crate::components::{ChampionSelector, formulas::source_code::SourceCode};
use yew::{Html, classes, function_component, html, use_callback, use_state};

#[function_component(Formulas)]
pub fn formulas() -> Html {
    let current_champion = use_state(|| String::from("Aatrox"));
    let callback = {
        let current_champion = current_champion.clone();
        use_callback((), move |v, _| {
            current_champion.set(v);
        })
    };

    html! {
        // <FormulaSidebar />
        <div class={classes!(
            "p-6", "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <div class={classes!("flex", "flex-wrap", "gap-4", "items-center")}>
                <h1 class={classes!(
                    "font-semibold", "text-2xl", "text-white"
                )}>
                    {"Formulas and Generator Code"}
                </h1>
                <ChampionSelector
                    callback={callback.clone()}
                    current_champion={(*current_champion).clone()}
                />
            </div>
            <SourceCode champion_id={(*current_champion).clone()} />
        </div>
    }
}
