use crate::{
    models::shared::{ChampionId, ItemId, RuneId},
    url,
    utils::StringExt,
};
use generated_code::AbilityLike;
use yew::{AttrValue, Classes, Html, Properties, function_component, html};

#[derive(PartialEq)]
pub enum ImageType {
    Abilities(ChampionId, AbilityLike),
    Champions(ChampionId),
    Items(ItemId),
    Runes(RuneId),
    Other(AttrValue),
}

impl From<RuneId> for ImageType {
    fn from(value: RuneId) -> Self {
        ImageType::Runes(value)
    }
}

impl From<ItemId> for ImageType {
    fn from(value: ItemId) -> Self {
        ImageType::Items(value)
    }
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
        ImageType::Abilities(champion_id, ability) => url!(
            "/img/abilities/{}.avif",
            champion_id.as_str().concat_char(ability.as_char())
        )
        .into(),
        ImageType::Champions(v) => url!("/img/champions/{}.avif", v.as_str()).into(),
        ImageType::Items(v) => url!("/img/items/{}.avif", v.to_u32()).into(),
        ImageType::Runes(v) => url!("/img/runes/{}.avif", v.to_u32()).into(),
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
