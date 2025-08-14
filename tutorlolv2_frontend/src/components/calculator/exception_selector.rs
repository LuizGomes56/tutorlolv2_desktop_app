use crate::{
    components::{Image, ImageType},
    svg, url,
};
use generated_code::ChampionId;
use std::str::FromStr;
use yew::{
    AttrValue, Callback, Html, InputEvent, Properties, TargetCast, classes, function_component,
    html, html::onchange::Event,
};

#[derive(PartialEq)]
pub enum Exception {
    Image,
    Stack,
}

pub trait Numeric: Copy + 'static + PartialEq + FromStr {
    fn parse_unsigned(s: String) -> Self;
}

macro_rules! impl_numeric {
    ($($typename:ty),*) => {
        $(
            impl Numeric for $typename {
                fn parse_unsigned(s: String) -> Self {
                    s.parse::<$typename>().unwrap_or_default().max(0)
                }
            }
        )*
    };
}

impl_numeric!(u8, u32);

#[derive(Properties, PartialEq)]
pub struct ExceptionField<T: Numeric> {
    pub callback: Callback<T>,
    pub source: Exception,
    pub img_url: AttrValue,
    pub title: AttrValue,
}

#[function_component(NumericField)]
pub fn numeric_field<T: Numeric>(props: &ExceptionField<T>) -> Html {
    let img_html = html! {
        <Image
            class={classes!("h-8", "w-8")}
            source={ImageType::Other(props.img_url.clone())}
        />
    };

    html! {
        <label
            class={classes!(
                "grid", "gap-x-2", "text-white", "grid-cols-[auto_1fr]", "justify-center",
            )}
            title={&props.title}
        >
            {
                match props.source {
                    Exception::Image => img_html,
                    Exception::Stack => {
                        html! {
                            <div
                                class={classes!(
                                    "flex", "justify-center",
                                    "items-center", "relative"
                                )}
                            >
                                {img_html}
                                <span class={classes!(
                                    "text-sm", "img-letter"
                                )}>
                                    {"S"}
                                </span>
                            </div>
                        }
                    },
                }
            }
            <input
                type={"number"}
                class={classes!("w-full", "text-center", "text-sm")}
                placeholder={"0"}
                oninput={{
                    let callback = props.callback.clone();
                    Callback::from(move |e: InputEvent| {
                        let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        let value = T::parse_unsigned(target.value());
                        callback.emit(value);
                    })
                }}
            />
        </label>
    }
}

#[derive(Properties, PartialEq)]
pub struct BooleanFieldProps {
    pub callback: Callback<bool>,
    pub enabled: bool,
    pub image_html: Html,
    pub title: AttrValue,
}

#[function_component(BooleanField)]
pub fn boolean_field(props: &BooleanFieldProps) -> Html {
    html! {
        <label
            class={classes!(
                "grid", "gap-x-2", "text-white",
                "grid-cols-[auto_1fr]", "justify-center",
                "cursor-pointer", "items-center"
            )}
            title={&props.title}
        >
            {props.image_html.clone()}
            <input
                type={"checkbox"}
                checked={props.enabled}
                onchange={{
                    let callback = props.callback.clone();
                    Callback::from(move |e: Event| {
                        let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        callback.emit(target.checked());
                    })
                }}
                class={classes!("sr-only", "peer")}
            />
            <div class={classes!(
                "relative", "h-6", "w-12", "rounded-full",
                "bg-pink-800", "transition-colors", "duration-200",
                "peer-checked:bg-emerald-700"
            )}>
                <span class={classes!(
                    "absolute", "left-0.5", "top-0.5",
                    "w-5", "h-5", "bg-white", "rounded-full",
                    "transform", "transition-transform", "duration-200",
                    if props.enabled { "translate-x-6" } else { "" },
                    "flex", "items-center", "justify-center"
                )}>
                    {
                        if props.enabled {
                            svg!("../../../public/svgs/enabled", "12")
                        } else {
                            svg!("../../../public/svgs/disabled", "12")
                        }
                    }
                </span>
            </div>
        </label>
    }
}

#[derive(PartialEq, Properties)]
pub struct ExceptionSelectorProps {
    pub current_player_champion_id: ChampionId,
    pub attack_form: bool,
    pub infer_stats: bool,
    pub set_ally_fire_dragons: Callback<u8>,
    pub set_ally_earth_dragons: Callback<u8>,
    pub set_current_player_stacks: Callback<u32>,
    pub set_current_player_attack_form: Callback<bool>,
    pub set_current_player_infer_stats: Callback<bool>,
}

const SIZE_SVG: &'static str = "32";

/// Pending
#[function_component(ExceptionSelector)]
pub fn exception_selector(props: &ExceptionSelectorProps) -> Html {
    html! {
        <>
            <NumericField<u8>
                title={"Number of ally fire dragons"}
                source={Exception::Image}
                img_url={url!("/img/other/fire_dragon.avif")}
                callback={props.set_ally_fire_dragons.clone()}
            />
            <NumericField<u8>
                title={"Number of ally earth dragons"}
                source={Exception::Image}
                img_url={url!("/img/other/earth_dragon.avif")}
                callback={props.set_ally_earth_dragons.clone()}
            />
            {
                match props.current_player_champion_id {
                    ChampionId::Bard | ChampionId::Kindred | ChampionId::Sion |
                    ChampionId::Chogath | ChampionId::Smolder | ChampionId::Nasus
                    | ChampionId::AurelionSol | ChampionId::Veigar => {
                        html! {
                            <NumericField<u32>
                                title={"Number of this champion's stacks"}
                                source={Exception::Stack}
                                img_url={url!(
                                    "/img/other/{}_stacks.avif",
                                    props.current_player_champion_id.as_str()
                                )}
                                callback={props.set_current_player_stacks.clone()}
                            />
                        }
                    }
                    ChampionId::Gnar | ChampionId::Nidalee => {
                        html! {
                            <BooleanField
                                enabled={props.attack_form}
                                callback={props.set_current_player_attack_form.clone()}
                                image_html={svg!("../../../public/svgs/shift", SIZE_SVG)}
                                title={"Toggle if this champion is melee or ranged"}
                            />
                        }
                    }
                    _ => {
                        html! {}
                    }
                }
            }
            <BooleanField
                enabled={props.infer_stats}
                callback={props.set_current_player_infer_stats.clone()}
                image_html={svg!("../../../public/svgs/infer", SIZE_SVG)}
                title={"Determine if this champion's stats will be based on its items, or manually inserted"}
            />
        </>
    }
}
