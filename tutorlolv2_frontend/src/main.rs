use crate::pages::*;
use yew::{Html, function_component, html};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod pages;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

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
        Route::ChildProcess { id } => match id {
            1..10 => html! { <h1>{ format!("Child Process {id}") }</h1> },
            _ => html! { <h1>{ "No Child Process with this id" }</h1> },
        },
        Route::NotFound => html! { <h1>{ "404 - Página não encontrada" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
