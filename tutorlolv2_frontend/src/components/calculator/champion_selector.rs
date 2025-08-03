use crate::{
    build_imports::CHAMPION_NAME_TO_ID,
    color,
    components::{Image, ImageType},
};
use yew::{AttrValue, Callback, Html, Properties, classes, function_component, html, use_memo};

#[derive(Properties, PartialEq)]
pub struct ChampionSelectorProps {
    pub set_champion_callback: Callback<&'static str>,
}

#[function_component(ChampionSelector)]
pub fn champion_selector(props: &ChampionSelectorProps) -> Html {
    let selector_memo = use_memo((), |_| {
        html! {
            <div class={classes!(
                "grid", "gap-4", "grid-cols-12",
                "w-fit"
            )}>
                {
                    for CHAMPION_NAME_TO_ID
                        .entries()
                        .enumerate()
                        .map(|(index, (name, id))| {
                            let len = CHAMPION_NAME_TO_ID.len();
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
                                        source={ImageType::Champions(AttrValue::Static(*id))}
                                        class={classes!("h-10", "w-10", "peer")}
                                    />
                                    <div class={classes!(
                                        "hidden", "flex-col", "peer-hover:flex",
                                        "fixed", "z-50", "py-2", "border",
                                        color!(border-800), "gap-y-1.5", "overflow-auto",
                                        "max-h-96", "px-3.5", color!(bg-900),
                                        if index % 12 > 6 {
                                            "-translate-x-[calc(100%-41px)]"
                                        } else { "-translate-x-[1px]" },
                                        if index > len.div_ceil(2) && index > 100 {
                                            "-translate-y-[calc(100%+41px)]"
                                        } else { "translate-y-[1px]" },
                                    )}>
                                        <span class={classes!(
                                            "text-left", "font-medium", "text-lg",
                                            "text-white", "text-nowrap",
                                            "max-w-64", "truncate",
                                        )}>
                                            {name}
                                        </span>
                                    </div>
                                </button>
                            }
                        })
                }
            </div>
        }
    });

    html! { (*selector_memo).clone() }
}
