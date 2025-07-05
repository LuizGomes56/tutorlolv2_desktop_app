use crate::{
    c,
    components::{formulas::formula_sidebar::FormulaSidebar, sidebar::Sidebar},
};
use yew::{Html, classes, function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!(
            "flex", "w-full"
        )}>
            <Sidebar />
            <div class={classes!(
                "flex", "w-full", "min-h-screen", "overflow-y-auto",
                c!(bg-900)
            )}>
                <FormulaSidebar />
                {"Ok"}
            </div>
        </div>
    }
}
