use crate::utils::init_cache;
use overlay::realtime::RealtimeOverlay;
use yew::{Html, classes, function_component, html};

mod calculator_v2;
mod components;
mod context;
mod hooks;
mod macros;
mod model_v2;
mod models;
mod overlay;
mod pages;
mod utils;

#[function_component(App)]
fn app() -> Html {
    html! {
        // <div class={classes!("flex", "w-full")}>
        //     <div class={classes!(
        //         "flex", "flex-1", "bg-[#121214]",
        //         "h-screen", "overflow-y-auto",
        //     )}></div>
        // </div>
        <RealtimeOverlay />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    init_cache();
}
