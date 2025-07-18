use crate::{components::calculator::InputGameAction, models::calculator::InputGame, svg, url};
use paste::paste;
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, UseReducerHandle, classes,
    function_component, html,
};

#[derive(PartialEq, Properties)]
pub struct ExceptionSelectorProps {
    pub current_player_champion_id: String,
    pub input_game: UseReducerHandle<InputGame>,
}

/// Pending
#[function_component(ExceptionSelector)]
pub fn exception_selector(props: &ExceptionSelectorProps) -> Html {
    let data = &*props.input_game;

    macro_rules! exception_cell {
        (img $img_path:expr) => {
            html! {
                <img
                    loading={"lazy"}
                    class={classes!("h-7", "w-7")}
                    src={$img_path}
                    alt={""}
                />
            }
        };
        (svg $svg_path:literal) => {
            html! {
                <span class={classes!("h-7", "w-7")}>
                    {svg!($svg_path, "24")}
                </span>
            }
        };
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
                            value={data.$field.to_string()}
                            oninput={{
                                let input_game = props.input_game.clone();
                                Callback::from(move |e: InputEvent| {
                                    let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = target.value().parse::<u8>().unwrap_or(0);
                                    input_game.dispatch(
                                        InputGameAction::[<Set $field:camel>](value)
                                    );
                                })
                            }}
                        />
                    </label>
                }
            }
        };
    }

    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-1",
        )}>
            {exception_cell!(exception_cell!(img url!("/img/other/fire_dragon.avif")), ally_fire_dragons)}
            {exception_cell!(exception_cell!(img url!("/img/other/earth_dragon.avif")), ally_earth_dragons)}
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
                                {exception_cell!(img url!(
                                    "/img/other/{}_stacks.avif",
                                    &props.current_player_champion_id
                                ))}
                                <span class={classes!("text-[13px]", "img-letter")}>{"S"}</span>
                            </div>
                        };
                        {exception_cell!(
                            image,
                            ally_earth_dragons
                        )}
                    }
                    "Gnar" | "Nidalee" => {
                        html! {
                            <div title={"Shift to melee/ranged"}>
                                {exception_cell!(
                                    exception_cell!(svg "../../../public/svgs/shift"), ally_fire_dragons
                                )}
                            </div>
                        }
                    }
                    _ => {
                        html! {}
                    }
                }
            }
            // <div title={"Infer Stats"}>
            //     {exception_cell!(exception_cell!(svg "../../../public/svgs/infer"), ally_fire_dragons)}
            // </div>
        </div>
    }
}
