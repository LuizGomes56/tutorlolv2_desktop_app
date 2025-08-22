use yew::{
    AttrValue, Callback, Children, Classes, Html, MouseEvent, Properties, function_component, html,
    use_state,
};

use crate::{components::hover::docs::hover_docs, utils::ComptimeCache};

#[derive(Properties, PartialEq)]
pub struct DocElementProps {
    pub offsets: Option<&'static (usize, usize)>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DocElement)]
pub fn doc_element(props: &DocElementProps) -> Html {
    let show = use_state(|| false);

    let onmouseenter = {
        let show = show.clone();
        Callback::from(move |e: MouseEvent| {
            show.set(e.shift_key());
        })
    };

    let onmouseleave = {
        let show = show.clone();
        Callback::from(move |e: MouseEvent| {
            show.set(e.shift_key());
        })
    };

    if let Some(offset) = props.offsets {
        html! {
            <div
                class={props.class.clone()}
                {onmouseenter}
                {onmouseleave}
            >
                {show.then_some(hover_docs(AttrValue::Static(offset.as_str())))}
                { for props.children.iter() }
            </div>
        }
    } else {
        html! {
            <div class={props.class.clone()}>
                { for props.children.iter() }
            </div>
        }
    }
}
