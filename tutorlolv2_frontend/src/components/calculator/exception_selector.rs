use crate::{models::calculator::InputGame, svg, url};
use std::{cell::RefCell, rc::Rc};
use yew::{Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct ExceptionSelectorProps {
    pub data: Rc<RefCell<InputGame>>,
}

#[function_component(ExceptionSelector)]
pub fn exception_selector(props: &ExceptionSelectorProps) -> Html {
    macro_rules! exception_cell {
        (img $img_path:expr) => {
            html! {
                <img
                    loading={"lazy"}
                    class={classes!("h-6", "w-6")}
                    src={$img_path}
                    alt={""}
                />
            }
        };
        (svg $svg_path:literal) => {
            html! {
                <span class={classes!("h-6", "w-6")}>
                    {svg!($svg_path, "24")}
                </span>
            }
        };
        ($img:expr, $field:ident) => {
            html! {
                <label class={classes!(
                    "grid", "gap-x-2", "text-white", "grid-cols-[auto_1fr]", "justify-center",
                )}>
                    {$img}
                    // {
                    //     if $is_svg { exception_cell!(@inner svg $img_path) }
                    //     else { exception_cell!(@inner img $img_path) }
                    // }
                    <input
                        type={"text"}
                        class={classes!("w-full", "text-center", "text-sm")}
                        placeholder={"0"}
                        value={(*props.data).borrow().$field.to_string()}
                        oninput={{
                            let data = props.data.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                (*data)
                                    .borrow_mut()
                                    .$field = target.value().parse::<u8>().unwrap_or(0);
                            })
                        }}
                    />
                </label>
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
                match (*props.data).borrow().active_player.champion_id.as_str() {
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
                                    (*props.data).borrow().active_player.champion_id
                                ))}
                                <span class={classes!("text-xs", "img-letter")}>{"S"}</span>
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
            <div title={"Infer Stats"}>
                {exception_cell!(exception_cell!(svg "../../../public/svgs/infer"), ally_fire_dragons)}
            </div>
        </div>
    }
}
