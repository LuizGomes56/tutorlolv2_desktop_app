use crate::pages::*;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use std::{collections::BTreeMap, hash::Hash};
use yew::{Html, function_component, html, platform::spawn_local};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod external;
mod hooks;
mod macros;
mod pages;

pub static STATIC_CHAMPIONS: OnceCell<BTreeMap<String, String>> = OnceCell::new();
pub static STATIC_ITEMS: OnceCell<BTreeMap<usize, String>> = OnceCell::new();
pub static STATIC_RUNES: OnceCell<BTreeMap<usize, String>> = OnceCell::new();
pub static STATIC_FORMULAS: OnceCell<BTreeMap<String, String>> = OnceCell::new();

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
    #[at("/")]
    Home,

    #[at("/formulas")]
    Formulas,

    #[at("/realtime")]
    Realtime,

    #[at("/child_process/:id")]
    ChildProcess { id: u8 },

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Formulas => html! { <Formulas /> },
        Route::Realtime => html! { <Realtime /> },
        Route::ChildProcess { id } => match id {
            1..10 => html! { <h1>{ format!("Child Process {id}") }</h1> },
            _ => html! { <h1>{ "No Child Process with this id" }</h1> },
        },
        Route::NotFound => html! { <h1>{ "404 - Página não encontrada" }</h1> },
    }
}

fn main() {
    spawn_local(async move {
        let _ = STATIC_CHAMPIONS.set(load_static(url!("/api/static/champions")).await);
        let _ = STATIC_ITEMS.set(load_static(url!("/api/static/items")).await);
        let _ = STATIC_RUNES.set(load_static(url!("/api/static/runes")).await);
        let _ = STATIC_FORMULAS.set(load_static(url!("/api/formulas/champions")).await);

        yew::Renderer::<App>::new().render();
    });
}
