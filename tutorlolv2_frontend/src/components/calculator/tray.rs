use crate::{
    components::{Image, ImageType},
    utils::{ImportedEnum, UnsafeCast},
};
use yew::{Callback, Html, Properties, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct TrayProps<T: UnsafeCast + PartialEq> {
    pub remove_callback: Callback<usize>,
    pub array: Vec<T>,
    #[prop_or_default]
    pub translate_left: bool,
}

#[function_component(Tray)]
pub fn tray<T>(props: &TrayProps<T>) -> Html
where
    T: UnsafeCast + PartialEq + ImportedEnum + Copy + 'static,
    T::Repr: TryInto<usize>,
    ImageType: From<T>,
{
    html! {
        <div class={classes!("grid", "grid-cols-5", "gap-2", "px-4")}>
            {for props.array.iter().enumerate().map(|(index, value)| {
                html! {
                    <button
                        data-classes={classes!(
                            "cursor-default",
                            props.translate_left.then_some("translate-x-[calc(-100%+240px)]")
                        )}
                        data-offset={
                            T::OFFSETS
                                .get(T::into_usize_unchecked(*value))
                                .map(|(s, e)| format!("{s},{e}"))
                        }
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
