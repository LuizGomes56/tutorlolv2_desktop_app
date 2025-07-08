use crate::{components::sidebar::Sidebar, external::invoke, pages::*};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use std::{collections::BTreeMap, hash::Hash};
use yew::{Html, classes, function_component, html, platform::spawn_local};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod external;
mod hooks;
mod macros;
mod models;
mod pages;

pub static STATIC_CHAMPIONS: OnceCell<BTreeMap<String, String>> = OnceCell::new();
pub static STATIC_ITEMS: OnceCell<BTreeMap<usize, String>> = OnceCell::new();
pub static STATIC_RUNES: OnceCell<BTreeMap<usize, String>> = OnceCell::new();
pub static STATIC_FORMULAS: OnceCell<BTreeMap<String, String>> = OnceCell::new();
pub static IS_DEKTOP_PLATFORM: OnceCell<bool> = OnceCell::new();

async fn load_static<T: DeserializeOwned + Eq + Hash + Ord>(
    url: &'static str,
) -> BTreeMap<T, String> {
    let request = reqwasm::http::Request::new(url).send().await;
    match request {
        Ok(response) => {
            let bytes = response.binary().await.unwrap();
            let decoded = bincode::serde::decode_from_slice::<BTreeMap<T, String>, _>(
                &bytes,
                bincode::config::standard(),
            );
            match decoded {
                Ok(data) => data.0,
                Err(e) => {
                    panic!("Error decoding {} at get_static_instance: {:#?}", url, e)
                }
            }
        }
        Err(e) => {
            panic!("Error sending request for {}: {:#?}", url, e)
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
        let _ = STATIC_FORMULAS.set(load_static(url!("/api/formulas/champions")).await);
        let _ = IS_DEKTOP_PLATFORM.set(invoke::invoke_checkup());

        yew::Renderer::<App>::new().render();
    });
}
