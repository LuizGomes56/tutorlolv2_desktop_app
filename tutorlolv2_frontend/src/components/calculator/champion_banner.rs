use crate::{components::calculator::champion_selector::ChampionSelector, url};
use yew::{AttrValue, Callback, Html, Properties, classes, function_component, html, use_state};

#[derive(Properties, PartialEq)]
pub struct ChampionBannerProps {
    pub champion_id: AttrValue,
    pub set_callback: Callback<String>,
}

#[function_component(ChampionBanner)]
pub fn champion_banner(props: &ChampionBannerProps) -> Html {
    let is_open = use_state(|| false);

    html! {
        <>
            <div class={classes!(if !*is_open { "hidden" } else { "" })}>
                <ChampionSelector
                    callback={props.set_callback.clone()}
                />
            </div>
            <img
                onclick={{
                    let is_open = is_open.clone();
                    move |_| {
                        is_open.set(!*is_open);
                    }
                }}
                loading={"lazy"}
                class={classes!("w-full", "img-clipped", "h-16", "cursor-pointer")}
                src={url!("/img/centered/{}_0.avif", props.champion_id)}
                alt={""}
            />
        </>
    }
}
