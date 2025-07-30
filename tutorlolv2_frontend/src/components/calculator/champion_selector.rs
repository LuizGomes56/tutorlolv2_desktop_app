use crate::{
    build_imports::CHAMPION_ID_TO_NAME,
    color,
    components::{Image, ImageType},
};
use yew::{Callback, Html, Properties, classes, function_component, html, use_memo};

#[derive(Properties, PartialEq)]
pub struct ChampionSelectorProps {
    pub set_champion_callback: Callback<&'static str>,
}

#[function_component(ChampionSelector)]
pub fn champion_selector(props: &ChampionSelectorProps) -> Html {
    let selector_memo = use_memo((), |_| {
        html! {
            <div class={classes!(
                "grid", "gap-2", "grid-cols-10",
            )}>
                {
                    for CHAMPION_ID_TO_NAME
                        .keys()
                        .map(|id| {
                            html! {
                                <button
                                    class={classes!(
                                        "items-center", "gap-2", "text-sm",
                                        color!(hover:bg-800), "select-none",
                                        "border", "relative", color!(border-700),
                                        "cursor-default",
                                    )}
                                    onclick={{
                                        let callback = props.set_champion_callback.clone();
                                        Callback::from(move |_| {
                                            callback.emit(*id);
                                        })
                                    }}
                                >
                                    <Image
                                        size={28}
                                        source={ImageType::Champions(*id)}
                                        class={classes!("cursor-pointer")}
                                    />
                                </button>
                            }
                        })
                }
            </div>
        }
    });

    html! { (*selector_memo).clone() }
}
