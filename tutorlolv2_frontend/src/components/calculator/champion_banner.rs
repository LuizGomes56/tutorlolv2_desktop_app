use crate::{STATIC_CHAMPIONS, components::ChampionSelector, hooks::mouseout::use_mouseout, url};
use yew::{
    AttrValue, Callback, Html, Properties, classes, function_component, html, use_callback,
    use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct ChampionBannerProps {
    pub champion_id: AttrValue,
    pub set_callback: Callback<String>,
}

#[function_component(ChampionBanner)]
pub fn champion_banner(props: &ChampionBannerProps) -> Html {
    let is_open = use_state(|| false);

    let dropdown_ref = use_node_ref();
    let label_ref = {
        let is_open = is_open.clone();
        use_mouseout(
            Callback::from(move |_| is_open.set(false)),
            [dropdown_ref.clone()],
        )
    };

    let onclick = {
        let is_open = is_open.clone();
        use_callback((), move |_, _| is_open.set(true))
    };

    html! {
        <div class={classes!("relative")}>
            <div ref={label_ref} onclick={onclick} class={classes!("relative")}>
                <img
                    loading={"lazy"}
                    class={classes!("w-full", "img-clipped", "h-16", "cursor-pointer")}
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
            <div ref={dropdown_ref} class={classes!(
                if !*is_open { "hidden" } else { "flex" },
                "flex-col"
            )}>
                <ChampionSelector
                    current_champion={props.champion_id.clone()}
                    callback={props.set_callback.clone()}
                />
            </div>
        </div>
    }
}
