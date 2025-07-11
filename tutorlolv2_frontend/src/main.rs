use crate::{components::sidebar::Sidebar, external::invoke, models::base::ComparedItem, pages::*};
use once_cell::sync::OnceCell;
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, HashMap};
use web_sys::console;
use yew::{Html, classes, function_component, html, platform::spawn_local};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod external;
mod hooks;
mod macros;
mod models;
mod pages;

pub static STATIC_CHAMPIONS: OnceCell<BTreeMap<String, String>> = OnceCell::new();
pub static STATIC_ABILITY_FORMULAS: OnceCell<FxHashMap<String, FxHashMap<String, String>>> =
    OnceCell::new();
pub static STATIC_ITEMS: OnceCell<BTreeMap<String, usize>> = OnceCell::new();
pub static STATIC_RUNES: OnceCell<BTreeMap<String, usize>> = OnceCell::new();
pub static STATIC_CHAMPION_FORMULAS: OnceCell<HashMap<String, String>> = OnceCell::new();
pub static STATIC_ITEM_FORMULAS: OnceCell<FxHashMap<usize, String>> = OnceCell::new();
pub static STATIC_RUNE_FORMULAS: OnceCell<FxHashMap<usize, String>> = OnceCell::new();
pub static STATIC_COMPARED_ITEMS: OnceCell<FxHashMap<usize, ComparedItem>> = OnceCell::new();
pub static IS_DEKTOP_PLATFORM: OnceCell<bool> = OnceCell::new();

async fn load_static<T: DeserializeOwned>(url: &'static str) -> T {
    let request = reqwasm::http::Request::new(url).send().await;
    match request {
        Ok(response) => {
            console::log_1(&format!("Loaded {}", url).into());
            let bytes = response.binary().await.unwrap();
            let decoded =
                bincode::serde::decode_from_slice::<T, _>(&bytes, bincode::config::standard());
            match decoded {
                Ok(data) => data.0,
                Err(e) => {
                    panic!("Error decoding {} at get_static_instance: {:#?}", url, e)
                }
            }
        }
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
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::History => html! { <History /> },
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
        let _ = STATIC_COMPARED_ITEMS.set(load_static(url!("/api/static/compared_items")).await);
        let _ = STATIC_CHAMPION_FORMULAS.set(load_static(url!("/api/formulas/champions")).await);
        let _ = STATIC_ITEM_FORMULAS.set(load_static(url!("/api/formulas/items")).await);
        let _ = STATIC_RUNE_FORMULAS.set(load_static(url!("/api/formulas/runes")).await);
        let _ = STATIC_ABILITY_FORMULAS.set(load_static(url!("/api/formulas/abilities")).await);
        let _ = IS_DEKTOP_PLATFORM.set(invoke::invoke_checkup());

        yew::Renderer::<App>::new().render();
    });
}
