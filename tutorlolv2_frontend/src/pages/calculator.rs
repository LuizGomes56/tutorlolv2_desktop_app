use crate::{models::calculator::InputGame, url};
use yew::{Html, classes, function_component, html, use_state};

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let data = use_state(|| InputGame::default);

    html! {
        <div class={classes!(
            "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-4", "w-64"
            )}>
                <img
                    class={classes!("w-full")}
                    src={url!("/img/centered/Neeko_0.avif")}
                    alt={""}
                />
            </div>
        </div>
    }
}
