use crate::{Route, color, svg};
use yew::{Html, classes, function_component, html};
use yew_router::components::Link;

const ICON_SIZE: &'static str = "22";

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let sidebar_classes = classes!(
        "flex",
        "items-center",
        "gap-3",
        color!(text - 300),
        "font-semibold",
        "w-full",
        "px-4",
        "h-12",
        color!(hover:bg-900),
        "hover:text-white",
        "cursor-pointer",
        "transition-colors",
        "duration-300",
        "rounded-lg",
    );
    macro_rules! make_tabs {
        ($($icon:expr, $tab:expr, $to:expr),*) => {
            [$(($icon, $tab, $to),)*]
            .into_iter()
            .map(|(icon, tab, to)| {
                html! {
                    <Link<Route> to={to} classes={sidebar_classes.clone()}>
                        {icon}
                        <span>{tab}</span>
                    </Link<Route>>
                }
            })
            .collect::<Html>()
        };
    }
    html! {
        <div class={classes!(
            "bg-[#17171b]", "h-screen", "p-4",
            "flex", "flex-col", "w-fit", "oxanium",
            "overflow-y-auto", "flex-shrink-0", "gap-2",
            "justify-between"
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-2",
            )}>
                {make_tabs!(
                    svg!("../../public/sidebar/home", ICON_SIZE), "Home", Route::Home,
                    svg!("../../public/sidebar/realtime", ICON_SIZE), "Realtime", Route::Realtime,
                    svg!("../../public/sidebar/calculator", ICON_SIZE), "Calculator", Route::Calculator,
                    svg!("../../public/sidebar/history", ICON_SIZE), "History", Route::History,
                    svg!("../../public/sidebar/formulas", ICON_SIZE), "Formulas", Route::Formulas
                )}
            </div>
            <div class={classes!("flex", "flex-col", "gap-2")}>
                {make_tabs!(
                    svg!("../../public/sidebar/settings", ICON_SIZE), "Settings", Route::Settings
                )}
            </div>
        </div>
    }
}
