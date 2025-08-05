use crate::{
    color,
    components::{Image, ImageType, hover::docs::hover_docs},
    context::{HoverDocs, SettingsContext},
    url,
    utils::FromBrotliBytes,
};
use generated_code::{CHAMPION_FORMULAS, CHAMPION_ID_TO_NAME};
use yew::{AttrValue, Html, Properties, classes, function_component, html, use_context};

#[derive(Properties, PartialEq)]
pub struct ChampionBannerProps {
    pub champion_id: AttrValue,
}

#[function_component(ChampionBanner)]
pub fn champion_banner(props: &ChampionBannerProps) -> Html {
    let hover_settings = use_context::<SettingsContext>()
        .and_then(|ctx| Some((*ctx).docs))
        .unwrap_or_default();

    html! {
        <div class={classes!("relative", match hover_settings {
            HoverDocs::Full => "group",
            _ => "",
        })}>
            <Image
                class={classes!("w-full", "img-clipped", "h-16")}
                source={ImageType::Other(url!("/img/centered/{}_0.avif", props.champion_id).into())}
            />
            <span class={classes!("img-letter", "left-2", "bottom-1", "text-sm")}>
                {*CHAMPION_ID_TO_NAME.get(&props.champion_id).unwrap_or(&"Unknown")}
            </span>
            {
                CHAMPION_FORMULAS
                    .get(&props.champion_id)
                    .and_then(|formula| {
                        if hover_settings == HoverDocs::Full {
                            Some(html! {
                                <div class={classes!(
                                    "group-hover:visible",
                                    "group-hover:opacity-100",
                                    "group-hover:pointer-events-auto",
                                    "opacity-0", "invisible",
                                    "pointer-events-none",
                                    "transition-[visibility,opacity]",
                                    "duration-200", "group-hover:delay-1000",
                                    "flex", "flex-col",
                                    "fixed", "z-50", "py-3", color!(border-800),
                                    "gap-y-3", "overflow-auto", "max-h-96",
                                    "px-3.5", color!(bg-900), "border",
                                )}>
                                    {hover_docs(AttrValue::Static(formula.as_str()), false)}
                                </div>
                            })
                        }
                        else {
                            None
                        }
                    })
                    .unwrap_or_default()
            }
        </div>
    }
}
