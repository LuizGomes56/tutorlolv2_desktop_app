use crate::{components::calculator::*, models::calculator::InputGame, url};
use std::{cell::RefCell, rc::Rc};
use yew::{Html, classes, function_component, html, use_state};

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let data = use_state(|| Rc::new(RefCell::new(InputGame::default())));

    html! {
        <div class={classes!(
            "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-4", "w-56"
            )}>
                <img
                    loading={"lazy"}
                    class={classes!("w-full", "img-clipped", "h-16")}
                    src={url!("/img/centered/{}_0.avif", (*data).borrow().active_player.champion_id)}
                    alt={""}
                />
                <div class={classes!(
                    "grid", "grid-cols-2", "gap-x-2",
                )}>
                    <AbilitySelector data={(*data).clone()} />
                    <ExceptionSelector data={(*data).clone()} />
                </div>
                <ItemSelector data={(*data).clone()} />
                <RuneSelector data={(*data).clone()} />
                <StatsSelector data={(*data).clone()} />
            </div>
        </div>
    }
}
