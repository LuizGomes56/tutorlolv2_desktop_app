use crate::{components::sidebar::Sidebar, context::SettingsProvider, pages::*, utils::init_cache};
use yew::{Html, classes, function_component, html};
use yew_router::{BrowserRouter, Routable, Switch};

mod components;
mod context;
mod hooks;
mod macros;
mod models;
mod pages;
mod utils;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,

    #[at("/formulas")]
    Formulas,

    #[at("/history")]
    History,

    #[at("/calculator")]
    Calculator,

    #[at("/help")]
    Help,

    #[at("/settings")]
    Settings,
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
    html! {
        <div class={classes!("flex", "w-full")}>
            <Sidebar />
            <div class={classes!(
                "flex", "flex-1", "bg-[#121214]",
                "h-screen", "overflow-y-auto",
            )}>
                {
                    match routes {
                        Route::Home => html! { <Home /> },
                        Route::Help => html! { <Help /> },
                        Route::History => html! { <History /> },
                        Route::Formulas => html! { <Formulas /> },
                        Route::Calculator => html! { <Calculator /> },
                        Route::Settings => html! { <Settings /> },
                    }
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    init_cache();
}
