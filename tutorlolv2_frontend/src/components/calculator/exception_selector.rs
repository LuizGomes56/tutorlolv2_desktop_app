use crate::{svg, url};
use paste::paste;
use yew::{
    AttrValue, Callback, Event, Html, InputEvent, Properties, TargetCast, classes,
    function_component, html,
};

#[derive(PartialEq, Properties)]
pub struct ExceptionSelectorProps {
    pub current_player_champion_id: AttrValue,
    pub set_ally_fire_dragons: Callback<u8>,
    pub set_ally_earth_dragons: Callback<u8>,
    pub set_current_player_stacks: Callback<u32>,
    pub set_current_player_attack_form: Callback<bool>,
    pub set_current_player_infer_stats: Callback<bool>,
}

/// Pending
#[function_component(ExceptionSelector)]
pub fn exception_selector(props: &ExceptionSelectorProps) -> Html {
    macro_rules! make {
        (@img $img_path:expr) => {
            html! {
                <img
                    loading={"lazy"}
                    class={classes!("h-7", "w-7")}
                    src={$img_path}
                    alt={""}
                />
            }
        };
        (@svg $svg_path:expr) => {
            html! {
                <>
                    {svg!($svg_path, "28")}
                </>
            }
        };
    }
    macro_rules! exception_cell {
        ($img:expr, $field:ident) => {
            paste! {
                html! {
                    <label class={classes!(
                        "grid", "gap-x-2", "text-white", "grid-cols-[auto_1fr]", "justify-center",
                    )}>
                        {$img}
                        <input
                            type={"number"}
                            class={classes!("w-full", "text-center", "text-sm")}
                            placeholder={"0"}
                            oninput={{
                                let callback = props.$field.clone();
                                Callback::from(move |e: InputEvent| {
                                    let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = target.value().parse().unwrap_or(0).max(0);
                                    callback.emit(value);
                                })
                            }}
                        />
                    </label>
                }
            }
        };
        (@bool $img:expr, $field:ident) => {
            paste! {
                html! {
                    <label class={classes!(
                        "grid", "gap-x-2", "text-white",
                        "grid-cols-[auto_1fr]", "justify-center",
                        "cursor-pointer"
                    )}>
                        {$img}
                        <div class={classes!(
                            "flex", "items-center", "justify-center"
                        )}>
                            <div class={classes!("relative")}>
                                <input
                                    type={"checkbox"}
                                    class={"sr-only peer"}
                                    onchange={{
                                        let callback = props.$field.clone();
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
        };
    }

    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-1",
        )}>
            {exception_cell!(
                make!(@img url!("/img/other/fire_dragon.avif")),
                set_ally_fire_dragons
            )}
            {exception_cell!(
                make!(@img url!("/img/other/earth_dragon.avif")),
                set_ally_earth_dragons
            )}
            {
                match props.current_player_champion_id.as_str() {
                    "Bard" | "Kindred" | "Sion" | "ChoGath" | "Smolder" |
                    "Nasus" | "AurelionSol" | "Veigar" => {
                        let image = html! {
                            <div
                                title={"Number of Stacks"}
                                class={classes!(
                                    "flex", "justify-center", "items-center", "relative"
                                )}
                            >
                                {make!(@img url!(
                                    "/img/other/{}_stacks.avif",
                                    &props.current_player_champion_id
                                ))}
                                <span class={classes!("text-[13px]", "img-letter")}>{"S"}</span>
                            </div>
                        };
                        {exception_cell!(
                            image,
                            set_current_player_stacks
                        )}
                    }
                    "Gnar" | "Nidalee" => {
                        html! {
                            <div title={"Shift to melee/ranged"}>
                                {exception_cell!(
                                    @bool
                                    make!(@svg "../../../public/svgs/shift"),
                                    set_current_player_attack_form
                                )}
                            </div>
                        }
                    }
                    _ => {
                        html! {}
                    }
                }
            }
            <div title={"Infer Stats"}>
                {exception_cell!(
                    @bool
                    make!(@svg "../../../public/svgs/infer"),
                    set_current_player_infer_stats
                )}
            </div>
        </div>
    }
}
