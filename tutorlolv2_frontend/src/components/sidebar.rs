use crate::{color, svg};
use yew::{Html, classes, function_component, html};

const ICON_SIZE: &str = "24";
#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div class={classes!(
            color!(bg-950), "h-screen", "p-4",
            "flex", "flex-col", "w-fit"
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-2",
            )}>
            {
                [
                    (svg!("../../public/sidebar/home", ICON_SIZE), "Home"),
                    (svg!("../../public/sidebar/realtime", ICON_SIZE), "Realtime"),
                    (svg!("../../public/sidebar/calculator", ICON_SIZE), "Calculator"),
                    (svg!("../../public/sidebar/formulas", ICON_SIZE), "Formulas"),
                    (svg!("../../public/sidebar/source_code", ICON_SIZE), "API Docs")
                ]
                .into_iter()
                .map(|(icon, tab)| {
                    html! {
                        <div class={classes!(
                            "flex", "items-center", "gap-2", color!(text-400),
                            "font-semibold", "w-full", "px-4", "h-12",
                            color!(hover:bg-900), "hover:text-white",
                            "cursor-pointer", "transition-colors",
                            "duration-300", "rounded-lg"
                        )}>
                            {icon}
                            <span>{tab}</span>
                        </div>
                    }
                })
                .collect::<Html>()
            }
            </div>
        </div>
    }
}
