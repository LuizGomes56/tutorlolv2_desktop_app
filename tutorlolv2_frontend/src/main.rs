use crate::{components::sidebar::Sidebar, context::SettingsProvider, external::invoke, pages::*};
use std::sync::atomic::AtomicBool;
use yew::{Html, classes, function_component, html};
use yew_router::{BrowserRouter, Routable, Switch};

mod build_imports;
mod components;
mod context;
mod external;
mod hooks;
mod macros;
mod models;
mod pages;

pub static IS_DEKTOP_PLATFORM: AtomicBool = AtomicBool::new(false);
pub static HISTORY_LOOP_FLAG: AtomicBool = AtomicBool::new(false);
pub static REALTIME_LOOP_FLAG: AtomicBool = AtomicBool::new(false);

pub const MAX_FAILURES: usize = 10; /* Attempts */
pub const RETRY_INTERVAL: u64 = 60; /* Seconds */
pub const REFRESH_RATE: u64 = 1_000_000; /* Millis */

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

    #[at("/child_process/:id")]
    ChildProcess { id: u8 },
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <SettingsProvider>
            <BrowserRouter>
                <div class={classes!(
                    "flex", "w-full"
                )}>
                    <Sidebar />
                    <div class={classes!(
                        "flex", "flex-1",
                        color!(bg-900)
                    )}>
                        <Switch<Route> render={switch} />
                    </div>
                </div>
            </BrowserRouter>
        </SettingsProvider>
    }
}

fn switch(routes: Route) -> Html {
    global_bool!(set HISTORY_LOOP_FLAG, true);
    match routes {
        Route::Home => html! { <Home /> },
        Route::History => {
            global_bool!(set HISTORY_LOOP_FLAG, false);
            html! { <History /> }
        }
        Route::Formulas => html! { <Formulas /> },
        Route::Realtime => html! { <Realtime /> },
        Route::Calculator => html! { <Calculator /> },
        Route::ChildProcess { id } => match id {
            1..10 => html! { <h1>{ format!("Child Process {id}") }</h1> },
            _ => html! { <h1>{ "No Child Process with this id" }</h1> },
        },
    }
}

fn main() {
    let _ = global_bool!(set IS_DEKTOP_PLATFORM, invoke::invoke_checkup());
    yew::Renderer::<App>::new().render();
}
