use crate::{
    components::sidebar::Sidebar, context::SettingsProvider, external::invoke, overlay::*,
    pages::*, utils::init_cache,
};
use std::sync::atomic::AtomicBool;
use yew::{Html, classes, function_component, html};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod context;
mod external;
mod hooks;
mod macros;
mod models;
mod overlay;
mod pages;
mod utils;

pub static IS_DEKTOP_PLATFORM: AtomicBool = AtomicBool::new(false);
pub static HISTORY_LOOP_FLAG: AtomicBool = AtomicBool::new(false);
pub static REALTIME_LOOP_FLAG: AtomicBool = AtomicBool::new(false);

pub const MAX_FAILURES: usize = 10;
pub const RETRY_INTERVAL: u64 = 10_000;
pub const REFRESH_RATE: u64 = 1_000;

#[macro_export]
macro_rules! global_bool {
    (set $varname:ident, $boolean:expr) => {
        crate::$varname.store($boolean, std::sync::atomic::Ordering::SeqCst)
    };
    (get $varname:ident) => {
        crate::$varname.load(std::sync::atomic::Ordering::SeqCst)
    };
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,

    #[at("/formulas")]
    Formulas,

    #[at("/realtime")]
    Realtime,

    #[at("/history")]
    History,

    #[at("/calculator")]
    Calculator,

    #[at("/help")]
    Help,

    #[at("/settings")]
    Settings,

    #[at("/overlay/:id")]
    Overlay { id: u8 },
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <SettingsProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </SettingsProvider>
    }
}

fn switch(routes: Route) -> Html {
    global_bool!(set HISTORY_LOOP_FLAG, true);
    let make = |component| {
        html! {
            <div class={classes!("flex", "w-full")}>
                <Sidebar />
                <div class={classes!(
                    "flex", "flex-1", "bg-[#121214]",
                    "h-screen", "overflow-y-auto",
                )}>
                    {component}
                </div>
            </div>
        }
    };
    match routes {
        // Route::Home => html! { <Process1 /> },
        Route::Home => make(html! { <Home /> }),
        Route::Help => make(html! { <Help /> }),
        Route::History => {
            global_bool!(set HISTORY_LOOP_FLAG, false);
            make(html! { <History /> })
        }
        Route::Formulas => make(html! { <Formulas /> }),
        Route::Realtime => make(html! { <Realtime /> }),
        Route::Calculator => make(html! { <Calculator /> }),
        Route::Settings => make(html! { <Settings /> }),
        Route::Overlay { id } => match id {
            1 => {
                global_bool!(set REALTIME_LOOP_FLAG, false);
                html! { <Process1 /> }
            }
            2 => {
                global_bool!(set REALTIME_LOOP_FLAG, false);
                html! { <Process2 /> }
            }
            _ => html! { <h1>{ "No Child Process with this id" }</h1> },
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    init_cache();
    let _ = global_bool!(set IS_DEKTOP_PLATFORM, invoke::invoke_checkup());
}
