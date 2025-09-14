use crate::{
    components::{Image, ImageType, calculator::StaticSelector},
    hooks::mouseout::use_mouseout,
    url,
    utils::ImportedEnum,
};
use tutorlolv2_imports::ChampionId;
use yew::{
    Callback, Html, Properties, classes, function_component, html, use_callback, use_memo,
    use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct ChampionBannerProps {
    pub callback: Callback<ChampionId>,
    pub champion_id: ChampionId,
    #[prop_or_default]
    pub translate_left: bool,
}

#[function_component(ChampionBanner)]
pub fn champion_banner(props: &ChampionBannerProps) -> Html {
    let is_open = use_state(|| false);
    let selector_ref = use_node_ref();
    let button_ref = {
        let is_open = is_open.clone();
        use_mouseout(
            Callback::from(move |_| is_open.set(false)),
            [selector_ref.clone()],
        )
    };
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };
    let selector_callback = {
        use_callback(
            (is_open.clone(), props.callback.clone()),
            move |v, (is_open, callback)| {
                is_open.set(false);
                callback.emit(v)
            },
        )
    };

    let banner = use_memo(
        (button_ref.clone(), onclick.clone(), props.champion_id),
        |(button_ref, onclick, champion_id)| {
            html! {
                <div
                    data-classes={classes!(
                        "cursor-default",
                        props.translate_left.then_some("translate-x-[calc(-100%+240px)]")
                    )}
                    data-offset={
                        ChampionId::OFFSETS
                            .get(*champion_id as usize)
                            .and_then(|(s, e)| Some(format!("{s},{e}")))
                    }
                    class={classes!("relative", "cursor-pointer")}
                    ref={button_ref}
                    {onclick}
                >
                    <Image
                        class={classes!("w-full", "img-clipped", "h-16")}
                        source={ImageType::Other(url!("/img/centered/{}_0.avif", champion_id.as_str()).into())}
                    />
                    <span class={classes!("img-letter", "left-2", "bottom-1", "text-sm")}>
                        {*ChampionId::ID_TO_NAME.get(*champion_id as usize).unwrap_or(&"Unknown")}
                    </span>
                </div>
            }
        },
    );

    html! {
        <>
            {(*banner).clone()}
            {
                (*is_open).then_some(
                    html! {
                        <StaticSelector<ChampionId>
                            callback={selector_callback}
                            node_ref={selector_ref}
                        />
                    }
                )
            }
        </>
    }
}
