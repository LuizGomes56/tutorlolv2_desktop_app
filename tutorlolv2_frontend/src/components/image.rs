use crate::url;
use yew::{AttrValue, Classes, Html, Properties, function_component, html};

#[derive(PartialEq)]
pub enum ImageType {
    Abilities(String),
    Champions(AttrValue),
    Items(u32),
    Runes(u32),
    Other(AttrValue),
}

#[derive(PartialEq, Properties)]
pub struct ImageProps {
    pub source: ImageType,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let class_attr = props.class.clone();

    let url = match &props.source {
        ImageType::Abilities(v) => url!("/img/abilities/{}.avif", v).into(),
        ImageType::Champions(v) => url!("/img/champions/{}.avif", v).into(),
        ImageType::Items(v) => url!("/img/items/{}.avif", v).into(),
        ImageType::Runes(v) => url!("/img/runes/{}.avif", v).into(),
        ImageType::Other(v) => v.clone(),
    };

    html! {
        <img
            loading={"lazy"}
            class={class_attr}
            src={url}
            alt={""}
        />
    }
}
