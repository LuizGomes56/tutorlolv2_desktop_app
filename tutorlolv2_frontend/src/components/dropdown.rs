use crate::{
    color, hooks::mouseout::use_mouseout, utils::UnsafeCast, svg, utils::StringifyEnum,
};
use yew::{Callback, Html, Properties, classes, function_component, html, use_node_ref, use_state};

#[derive(Properties, PartialEq)]
pub struct DropdownProps<const N: usize, T: Copy + 'static + StringifyEnum + UnsafeCast + PartialEq>
{
    pub current_index: T,
    pub callback: Callback<T>,
    pub iterator: [&'static str; N],
    pub name: &'static str,
}

#[function_component(Dropdown)]
pub fn dropdown<const N: usize, T>(props: &DropdownProps<N, T>) -> Html
where
    T: PartialEq + UnsafeCast + StringifyEnum + Copy + 'static,
    T::Repr: TryInto<usize> + TryFrom<usize>,
{
    let is_open = use_state(|| false);

    let dropdown_ref = use_node_ref();
    let button_ref = {
        let is_open = is_open.clone();
        use_mouseout(
            Callback::from(move |_| is_open.set(false)),
            [dropdown_ref.clone()],
        )
    };

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class={classes!("relative")}>
            <button
                onclick={onclick}
                ref={button_ref}
                class={classes!(
                    "flex", "items-center", "w-full",
                    "gap-4", "max-w-full",
                    "sm:w-96", "justify-between", "px-4",
                    "py-2.5", "border", "cursor-pointer",
                    "focus:outline-none",
                    color!(focus:border-400),
                    color!(border-700), "rounded-lg"
                )}
                type={"button"}
            >
                <div class={classes!(
                    "flex", "items-center", "gap-4",
                    color!(text-200)
                )}>
                    <span>{props.current_index.as_str()}</span>
                </div>
                <span class={classes!(color!(text-400))}>
                    {svg!("../../public/svgs/chevron_down", "20")}
                </span>
            </button>
            <div
                ref={dropdown_ref}
                class={classes!(
                    "absolute", "max-w-full", "mt-2", "z-50", "w-full",
                    "sm:w-96", "max-h-56", "overflow-y-auto",
                    "flex-col", color!(bg-800), color!(border-700),
                    "py-2", "border", "rounded-xl",
                    if *is_open { "flex" } else { "hidden" }
                )}
            >
                {
                    for props.iterator
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            html! {
                                <label class={classes!(
                                    "cursor-pointer",
                                    "has-[:checked]:bg-rose-900", "relative",
                                    "py-1.5", "px-5", color!(hover:bg-700),
                                    "transition-colors", "duration-150",
                                    "has-[:checked]:font-medium",
                                )}>
                                    <input
                                        checked={index == T::into_usize_unchecked(props.current_index)}
                                        onchange={{
                                            let callback = props.callback.clone();
                                            Callback::from(move |_| {
                                                callback.emit(T::from_usize_unchecked(index));
                                            })
                                        }}
                                        type={"radio"}
                                        name={props.name}
                                        class={classes!(
                                            "appearance-none", "absolute", "peer"
                                        )}
                                    />
                                    <span class={classes!(color!(text-200))}>
                                        {field}
                                    </span>
                                </label>
                            }
                        })
                }
            </div>
        </div>
    }
}
