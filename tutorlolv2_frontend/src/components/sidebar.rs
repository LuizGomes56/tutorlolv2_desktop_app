use crate::{Route, color, svg};
use yew::{Html, classes, function_component, html};
use yew_router::components::Link;

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
                    (svg!("../../public/sidebar/home", ICON_SIZE), "Home", Route::Home),
                    (svg!("../../public/sidebar/realtime", ICON_SIZE), "Realtime", Route::NotFound),
                    (svg!("../../public/sidebar/calculator", ICON_SIZE), "Calculator", Route::NotFound),
                    (svg!("../../public/sidebar/source_code", ICON_SIZE), "Formulas", Route::Formulas),
                ]
                .into_iter()
                .map(|(icon, tab, to)| {
                    html! {
                        <Link<Route> to={to} classes={classes!(
                            "flex", "items-center", "gap-2", color!(text-400),
                            "font-semibold", "w-full", "px-4", "h-12",
                            color!(hover:bg-900), "hover:text-white",
                            "cursor-pointer", "transition-colors",
                            "duration-300", "rounded-lg"
                        )}>
                            {icon}
                            <span>{tab}</span>
                        </Link<Route>>
                    }
                })
                .collect::<Html>()
            }
            </div>
        </div>
    }
}
