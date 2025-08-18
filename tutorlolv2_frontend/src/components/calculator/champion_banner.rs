use crate::{
    color,
    components::{Image, ImageType, hover::docs::hover_docs},
    context::{HoverDocs, SettingsContext},
    url,
    utils::ComptimeCache,
};
use generated_code::{CHAMPION_FORMULAS, CHAMPION_ID_TO_NAME, ChampionId};
use yew::{
    AttrValue, Html, Properties, classes, function_component, html, use_callback, use_context,
    use_state,
};

#[derive(Properties, PartialEq)]
pub struct ChampionBannerProps {
    pub champion_id: ChampionId,
}

#[function_component(ChampionBanner)]
pub fn champion_banner(props: &ChampionBannerProps) -> Html {
    let hover_settings = use_context::<SettingsContext>()
        .map(|ctx| (*ctx).docs)
        .unwrap_or_default();

    let rendered = use_state(|| false);
    let onmouseenter = {
        let rendered = rendered.clone();
        use_callback((), move |_, _| {
            rendered.set(true);
        })
    };
    let onmouseleave = {
        let rendered = rendered.clone();
        use_callback((), move |_, _| {
            rendered.set(false);
        })
    };

    let maybe_coords = CHAMPION_FORMULAS.get(props.champion_id as usize);

    html! {
        <div
            class={classes!("relative", match hover_settings { HoverDocs::Full => "group", _ => "" })}
            {onmouseenter}
            {onmouseleave}
        >
            <Image
                class={classes!("w-full", "img-clipped", "h-16")}
                source={ImageType::Other(url!("/img/centered/{}_0.avif", props.champion_id.as_str()).into())}
            />
            <span class={classes!("img-letter", "left-2", "bottom-1", "text-sm")}>
                {*CHAMPION_ID_TO_NAME.get(props.champion_id as usize).unwrap_or(&"Unknown")}
            </span>
            <div class={classes!(
                "group-hover:visible",
                "group-hover:opacity-100",
                "group-hover:pointer-events-auto",
                "opacity-0", "invisible",
                "pointer-events-none",
                "transition-[visibility,opacity]",
                "duration-200", "group-hover:delay-1000",
                "flex", "flex-col",
                "absolute", "z-50", "py-3", color!(border-800),
                "gap-y-3", "overflow-auto", "max-h-96",
                "px-3.5", color!(bg-900), "border",
            )}>
                {
                    (*rendered && hover_settings == HoverDocs::Full).then(|| {
                        maybe_coords.and_then(|coords| {
                            Some(html! { hover_docs(AttrValue::Static(coords.as_str()), false) })
                        })
                    })
                }
            </div>
        </div>
    }
}
