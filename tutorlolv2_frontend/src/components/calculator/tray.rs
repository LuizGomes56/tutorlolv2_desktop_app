use crate::{
    components::{Image, ImageType},
    utils::UnsafeCast,
};
use yew::{Callback, Html, Properties, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct TrayProps<T: UnsafeCast + PartialEq> {
    pub remove_callback: Callback<usize>,
    pub array: Vec<T>,
}

#[function_component(Tray)]
pub fn tray<T>(props: &TrayProps<T>) -> Html
where
    T: UnsafeCast + PartialEq + Copy + 'static,
    ImageType: From<T>,
{
    html! {
        <div class={classes!("grid", "grid-cols-5", "gap-2", "px-4")}>
            {for props.array.iter().enumerate().map(|(index, value)| {
                html! {
                    <button
                        onclick={props.remove_callback.reform(move |_| index)}
                    >
                        <Image
                            source={ImageType::from(*value)}
                            class={classes!("w-8", "h-8")}
                        />
                    </button>
                }
            })}
        </div>
    }
}
