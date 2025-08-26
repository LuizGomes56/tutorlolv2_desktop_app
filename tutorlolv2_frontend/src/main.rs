use crate::{
    components::sidebar::Sidebar, context::SettingsProvider, external::invoke, pages::*,
    utils::init_cache,
};
use std::sync::atomic::AtomicBool;
use web_sys::{js_sys::Function, window};
use yew::{Html, classes, function_component, html};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod context;
mod external;
mod hooks;
mod macros;
mod models;
mod pages;
mod utils;

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
                )}>{component}</div>
            </div>
        }
    };
    match routes {
        Route::Help => make(html! { <Help /> }),
        Route::Home => make(html! { <Home /> }),
        Route::History => {
            global_bool!(set HISTORY_LOOP_FLAG, false);
            make(html! { <History /> })
        }
        Route::Formulas => make(html! { <Formulas /> }),
        Route::Realtime => make(html! { <Realtime /> }),
        Route::Calculator => make(html! { <Calculator /> }),
        Route::Settings => make(html! { <Settings /> }),
        Route::Overlay { id } => match id {
            1..10 => html! { <h1>{ format!("Child Process {id}") }</h1> },
            _ => html! { <h1>{ "No Child Process with this id" }</h1> },
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    init_cache();
    let _ = global_bool!(set IS_DEKTOP_PLATFORM, invoke::invoke_checkup());

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let _ = body.add_event_listener_with_callback(
        "keydown", 
        &Function::new_with_args(
        "e",
        r#"if(e.key==="Shift"){const a=document.querySelectorAll("[data-offset]:hover");if(a.length===0)return;const b=a[a.length-1];if(b.querySelector(".hover-docs"))return;const c=b.getAttribute("data-offset").split(",");const s=parseInt(c[0]);const f=parseInt(c[1]);const t=document.createElement("div");const l=b.getAttribute("data-classes")||"";t.className="flex flex-col absolute max-w-md max-h-96 overflow-auto p-2 leading-6 text-base z-50 hover-docs border _border-800 _bg-900"+(l?" "+l:"");const d=document.createElement("code");d.className="text-[#D4D4D4] font-normal text-left text-wrap";d.innerHTML=window.decodeCacheSlice(s,f);t.appendChild(d);b.appendChild(t);const r=()=>{t.remove();b.removeEventListener("mouseleave",r)};b.addEventListener("mouseleave",r,{once:true})}"#
        ),
    );
}
