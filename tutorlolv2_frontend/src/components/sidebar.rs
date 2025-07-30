use crate::{
    Route, color,
    context::{GlobalContextActions, HoverDocs, SettingsContext},
    svg,
};
use yew::{Callback, Event, Html, TargetCast, classes, function_component, html, use_context};
use yew_router::components::Link;

const ICON_SIZE: &'static str = "22";

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let sidebar_classes = classes!(
        "flex",
        "items-center",
        "gap-3",
        color!(text - 400),
        "font-semibold",
        "w-full",
        "px-4",
        "h-12",
        color!(hover:bg-900),
        "hover:text-white",
        "cursor-pointer",
        "transition-colors",
        "duration-300",
        "rounded-lg"
    );
    html! {
        <div class={classes!(
            "bg-[#141417]", "h-screen", "p-4",
            "flex", "flex-col", "w-fit",
            "border-r", color!(border-800),
            "overflow-y-auto", "flex-shrink-0"
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-2",
            )}>
                {
                    for [
                        (svg!("../../public/sidebar/home", ICON_SIZE), "Home", Route::Home),
                        (svg!("../../public/sidebar/realtime", ICON_SIZE), "Realtime", Route::Realtime),
                        (svg!("../../public/sidebar/calculator", ICON_SIZE), "Calculator", Route::Calculator),
                        (svg!("../../public/sidebar/history", ICON_SIZE), "History", Route::History),
                        (svg!("../../public/sidebar/formulas", ICON_SIZE), "Formulas", Route::Formulas),
                    ]
                    .into_iter()
                    .map(|(icon, tab, to)| {
                        html! {
                            <Link<Route> to={to} classes={sidebar_classes.clone()}>
                                {icon}
                                <span>{tab}</span>
                            </Link<Route>>
                        }
                    })
                }
            </div>
            <div class={classes!("flex", "flex-col", "gap-2")}>
                {
                    for [
                        (svg!("../../public/sidebar/settings", ICON_SIZE), "Settings", html! { <SettingsCfg /> }),
                    ]
                    .into_iter()
                    .map(|(icon, name, cfg_box)| {
                        html! {
                            <div class={sidebar_classes.clone()}>
                                {icon}
                                <span>{name}</span>
                                {cfg_box}
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}

#[function_component(SettingsCfg)]
fn settings_cfg() -> Html {
    let context = use_context::<SettingsContext>().expect("SettingsContext não encontrado");

    let onchange = {
        let context = context.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let docs = match select.value().as_str() {
                "Full" => HoverDocs::Full,
                "Partial" => HoverDocs::Partial,
                "None" => HoverDocs::None,
                _ => HoverDocs::Full,
            };
            context.dispatch(GlobalContextActions::SetHoverDocs(docs));
        })
    };

    html! {
        <div class={classes!(
            "fixed","top-1/2","left-1/2",
            "-translate-x-1/2","-translate-y-1/2",
            "flex","items-center","gap-2","flex-col"
        )}>
            <select
                id={"slt_hover_docs"}
                name={"slt_hover_docs"}
                onchange={onchange}
                title={"Change how hovering in objects behaves"}
            >
                <option
                    value={"Full"}
                    selected={context.docs == HoverDocs::Full}
                >{ "Full" }</option>
                <option
                    value={"Partial"}
                    selected={context.docs == HoverDocs::Partial}
                >{ "Partial" }</option>
                <option
                    value={"None"}
                    selected={context.docs == HoverDocs::None}
                >{ "None" }</option>
            </select>
        </div>
    }
}
