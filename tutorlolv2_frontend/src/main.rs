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
                        "flex", "flex-1", "bg-[#121214]",
                        "h-screen", "overflow-y-auto",
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
        Route::Help => html! { <Help /> },
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
        Route::Settings => html! { <Settings /> },
    }
}

fn main() {
    init_cache();
    let _ = global_bool!(set IS_DEKTOP_PLATFORM, invoke::invoke_checkup());
    yew::Renderer::<App>::new().render();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let _ = body.add_event_listener_with_callback(
        "keydown", 
        &Function::new_with_args(
            "e",
            concat!(
                r#"if(e.key==="Shift"){const a=document.querySelectorAll("[data-offset]:hover");if(a.length===0)return;const b=a[a.length-1];if(b.querySelector(".hover-docs"))return;const c=b.getAttribute("data-offset").split(",");const s=parseInt(c[0]);const f=parseInt(c[1]);const t=document.createElement("div");t.className="flex flex-col absolute max-w-md max-h-96 overflow-auto p-2 leading-6 text-base z-50 hover-docs translate-x-[calc(50%-16px)] translate-y-[calc(50%+16px)] border "#,
                color!(border - 800),
                " ",
                color!(bg - 900),
                r#"";const d=document.createElement("code");d.className="text-[#D4D4D4] font-normal text-left text-wrap";d.innerHTML=window.decodeCacheSlice(s,f);t.appendChild(d);b.appendChild(t);const r=()=>{t.remove();b.removeEventListener("mouseleave",r)};b.addEventListener("mouseleave",r,{once:true})}"#
            ),
        ),
    );
}
