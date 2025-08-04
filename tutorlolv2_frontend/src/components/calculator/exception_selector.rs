use crate::{
    components::{Image, ImageType},
    svg, url,
};
use std::str::FromStr;
use yew::{
    AttrValue, Callback, Event, Html, InputEvent, Properties, TargetCast, classes,
    function_component, html,
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
                "cursor-pointer"
            )}
            title={&props.title}
        >
            {props.image_html.clone()}
            <div class={classes!(
                "flex", "items-center", "justify-center"
            )}>
                <div class={classes!("relative")}>
                    <input
                        type={"checkbox"}
                        class={"sr-only peer"}
                        onchange={{
                            let callback = props.callback.clone();
                            Callback::from(move |e: Event| {
                                let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                callback.emit(target.checked());
                            })
                        }}
                    />
                    <div class={classes!(
                        "w-10", "h-5", "bg-gray-600", "rounded-full",
                        "peer-checked:bg-green-500", "transition-colors"
                        )}
                    />
                    <div class={classes!(
                        "absolute", "top-0.5", "left-0.5", "w-4",
                        "h-4", "bg-white", "rounded-full",
                        "peer-checked:translate-x-5",
                        "transition-transform"
                        )}
                    />
                </div>
            </div>
        </label>
    }
}

#[derive(PartialEq, Properties)]
pub struct ExceptionSelectorProps {
    pub current_player_champion_id: AttrValue,
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
        <div class={classes!(
            "flex", "flex-col", "gap-2",
        )}>
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
                match props.current_player_champion_id.as_str() {
                    "Bard" | "Kindred" | "Sion" | "ChoGath" | "Smolder" |
                    "Nasus" | "AurelionSol" | "Veigar" => {
                        html! {
                            <NumericField<u32>
                                title={"Number of this champion's stacks"}
                                source={Exception::Stack}
                                img_url={url!(
                                    "/img/other/{}_stacks.avif",
                                    &props.current_player_champion_id
                                )}
                                callback={props.set_current_player_stacks.clone()}
                            />
                        }
                    }
                    "Gnar" | "Nidalee" => {
                        html! {
                            <BooleanField
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
                callback={props.set_current_player_infer_stats.clone()}
                image_html={svg!("../../../public/svgs/infer", SIZE_SVG)}
                title={"Determine if this champion's stats will be based on its items, or manually inserted"}
            />
        </div>
    }
}
