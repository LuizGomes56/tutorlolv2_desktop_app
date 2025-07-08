use crate::{IS_DEKTOP_PLATFORM, external::invoke};
use web_sys::console;
use yew::{Html, classes, function_component, html, platform::spawn_local};

#[function_component(Realtime)]
pub fn realtime() -> Html {
    if !IS_DEKTOP_PLATFORM.get_or_init(|| false) {
        return html! {
            <div class={classes!(
                "p-6", "flex-1", "h-screen", "overflow-y-auto",
                "flex", "flex-col", "gap-4",
            )}>
                <h1 class={classes!(
                    "font-semibold", "text-2xl", "text-white"
                )}>
                    { "Download the desktop application to use this feature" }
                </h1>
            </div>
        };
    }

    spawn_local(async move {
        let result = invoke::invoke_get_live_game().await;
        console::log_1(&format!("Result invoke: {:#?}", result).into());
    });

    html! {
        <div class={classes!(
            "p-6", "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <h1>{ "Realtime" }</h1>
        </div>
    }
}
