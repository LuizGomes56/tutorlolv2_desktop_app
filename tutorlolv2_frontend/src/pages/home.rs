use crate::{color, components::sidebar::Sidebar};
use yew::{Html, classes, function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!(
            "flex", "w-full"
        )}>
            <Sidebar />
            <div class={classes!(
                "flex", "flex-1",
                color!(bg-900)
            )}>
                <div class={classes!(
                    "p-6", "flex-1", "h-screen", "overflow-y-auto"
                )}>
                    <div class={classes!("max-w-3xl", "flex", "flex-col", "gap-4")}>
                        <h1 class={classes!(
                            "font-semibold", "text-2xl", "text-white"
                        )}>
                            {"Automated damage evaluation"}
                        </h1>
                    </div>
                </div>
            </div>
        </div>
    }
}
