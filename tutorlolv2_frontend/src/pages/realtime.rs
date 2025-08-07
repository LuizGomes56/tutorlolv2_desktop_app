use crate::{external::invoke, global_bool};
use web_sys::console;
use yew::{Html, classes, function_component, html, platform::spawn_local};

#[function_component(Realtime)]
pub fn realtime() -> Html {
    if !global_bool!(get IS_DEKTOP_PLATFORM) {
        return html! {
            <div class={classes!(
                "p-6", "flex-1", "flex", "flex-col", "gap-4",
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
            "p-6", "flex-1", "flex", "flex-col", "gap-4",
        )}>
            <h1>{ "Realtime" }</h1>
        </div>
    }
}
