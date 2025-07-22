use crate::{STATIC_CHAMPIONS, url};
use yew::{AttrValue, Html, Properties, classes, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ChampionBannerProps {
    pub champion_id: AttrValue,
}

#[function_component(ChampionBanner)]
pub fn champion_banner(props: &ChampionBannerProps) -> Html {
    html! {
        <div class={classes!("relative")}>
            <img
                loading={"lazy"}
                class={classes!("w-full", "img-clipped", "h-16")}
                src={url!("/img/centered/{}_0.avif", props.champion_id)}
                alt={""}
            />
            <span class={classes!("img-letter", "left-2", "bottom-1", "text-sm")}>
                {
                    STATIC_CHAMPIONS
                        .get()
                        .and_then(|champions| champions.get(&props.champion_id.to_string()))
                        .map(|champ| champ)
                        .and_then(|champ| Some(champ.as_str()))
                        .unwrap_or("Unknown")
                }
            </span>
        </div>
    }
}
