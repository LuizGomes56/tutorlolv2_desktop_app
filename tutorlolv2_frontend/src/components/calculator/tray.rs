use crate::{
    components::{Image, calculator::StaticSelector},
    hooks::mouseout::use_mouseout,
    utils::ImportedEnum,
};
use yew::{Callback, Html, Properties, classes, function_component, html, use_node_ref, use_state};

#[derive(PartialEq, Properties)]
pub struct OpenTrayProps<T: PartialEq> {
    pub insert_callback: Callback<T>,
    pub title: &'static str,
}

#[function_component(OpenTray)]
pub fn open_tray<T: ImportedEnum>(props: &OpenTrayProps<T>) -> Html {
    let is_open = use_state(|| false);
    let selector_ref = use_node_ref();
    let button_ref = {
        let is_open = is_open.clone();
        use_mouseout(
            Callback::from(move |_| is_open.set(false)),
            [selector_ref.clone()],
        )
    };
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };
    html! {
        <>
            <button
                onclick={onclick}
                ref={button_ref}
            >
                {props.title}
            </button>
            {
                (*is_open).then_some(
                    html! {
                        <StaticSelector<T>
                            callback={props.insert_callback.clone()}
                            node_ref={selector_ref}
                        />
                    }
                )
            }
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct TrayProps<T: PartialEq> {
    pub remove_callback: Callback<usize>,
    pub array: Vec<T>,
    #[prop_or_default]
    pub translate_left: bool,
}

#[function_component(Tray)]
pub fn tray<T: ImportedEnum>(props: &TrayProps<T>) -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-5", "gap-2", "px-4")}>
            {for props.array.iter().enumerate().map(|(index, value)| {
                html! {
                    <button
                        data-classes={classes!(
                            "cursor-default",
                            props.translate_left.then_some("translate-x-[calc(-100%+32px)]")
                        )}
                        data-offset={
                            T::OFFSETS
                                .get(T::into_usize(*value))
                                .map(|(s, e)| format!("{s},{e}"))
                        }
                        onclick={props.remove_callback.reform(move |_| index)}
                    >
                        <Image
                            source={value.into_image_type()}
                            class={classes!("w-8", "h-8")}
                        />
                    </button>
                }
            })}
        </div>
    }
}
