use crate::{
    components::sidebar::Sidebar,
    context::SettingsProvider,
    external::{api::decode_bytes, invoke},
    models::base::{ItemDef, SpriteMap},
    pages::*,
};
use once_cell::sync::OnceCell;
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use std::{
    collections::{BTreeMap, HashMap},
    sync::atomic::AtomicBool,
};
use web_sys::console;
use yew::{Html, classes, function_component, html, platform::spawn_local};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod context;
mod external;
mod hooks;
mod macros;
mod models;
mod pages;

pub static STATIC_CHAMPIONS: OnceCell<BTreeMap<String, String>> = OnceCell::new();
pub static STATIC_ABILITY_FORMULAS: OnceCell<FxHashMap<String, FxHashMap<String, String>>> =
    OnceCell::new();
pub static STATIC_ITEMS: OnceCell<BTreeMap<String, u32>> = OnceCell::new();
pub static STATIC_RUNES: OnceCell<BTreeMap<String, u32>> = OnceCell::new();
pub static STATIC_CHAMPION_FORMULAS: OnceCell<HashMap<String, String>> = OnceCell::new();
pub static STATIC_ITEM_FORMULAS: OnceCell<FxHashMap<u32, String>> = OnceCell::new();
pub static STATIC_RUNE_FORMULAS: OnceCell<FxHashMap<u32, String>> = OnceCell::new();
pub static STATIC_ITEMS_DEF: OnceCell<FxHashMap<u32, ItemDef>> = OnceCell::new();
pub static STATIC_SPRITE_MAP: OnceCell<SpriteMap> = OnceCell::new();
pub static IS_DEKTOP_PLATFORM: OnceCell<bool> = OnceCell::new();
pub static HISTORY_LOOP_FLAG: AtomicBool = AtomicBool::new(false);
pub static REALTIME_LOOP_FLAG: AtomicBool = AtomicBool::new(false);

pub const MAX_FAILURES: usize = 10; /* Attempts */
pub const RETRY_INTERVAL: u64 = 60; /* Seconds */
pub const REFRESH_RATE: u64 = 1_000_000; /* Millis */

#[macro_export]
macro_rules! loop_flag {
    (history $boolean:literal) => {
        crate::HISTORY_LOOP_FLAG.store($boolean, std::sync::atomic::Ordering::SeqCst);
    };
    (history) => {
        crate::HISTORY_LOOP_FLAG.load(std::sync::atomic::Ordering::SeqCst)
    };
    (realtime $boolean:literal) => {
        crate::REALTIME_LOOP_FLAG.store($boolean, std::sync::atomic::Ordering::SeqCst);
    };
    (realtime) => {
        crate::REALTIME_LOOP_FLAG.load(std::sync::atomic::Ordering::SeqCst)
    };
}

async fn load_static<T: DeserializeOwned>(url: &'static str) -> T {
    let request = reqwasm::http::Request::new(url).send().await;
    match request {
        Ok(response) => decode_bytes::<T>(response).await.unwrap(),
        Err(e) => {
            console::log_1(&format!("Error sending request for {}: {:#?}", url, e).into());
            panic!();
        }
    }
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
    loop_flag!(history true);
    match routes {
        Route::Home => html! { <Home /> },
        Route::History => {
            loop_flag!(history false);
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
    spawn_local(async move {
        let _ = STATIC_CHAMPIONS.set(load_static(url!("/api/static/champions")).await);
        let _ = STATIC_ITEMS.set(load_static(url!("/api/static/items")).await);
        let _ = STATIC_RUNES.set(load_static(url!("/api/static/runes")).await);
        let _ = STATIC_ITEMS_DEF.set(load_static(url!("/api/static/items_def")).await);
        let _ = STATIC_SPRITE_MAP.set(load_static(url!("/api/static/sprite_map")).await);
        let _ = STATIC_CHAMPION_FORMULAS.set(load_static(url!("/api/formulas/champions")).await);
        let _ = STATIC_ITEM_FORMULAS.set(load_static(url!("/api/formulas/items")).await);
        let _ = STATIC_RUNE_FORMULAS.set(load_static(url!("/api/formulas/runes")).await);
        let _ = STATIC_ABILITY_FORMULAS.set(load_static(url!("/api/formulas/abilities")).await);
        let _ = IS_DEKTOP_PLATFORM.set(invoke::invoke_checkup());

        yew::Renderer::<App>::new().render();
    });
}
