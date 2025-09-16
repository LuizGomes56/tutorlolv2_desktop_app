use crate::{url, utils::StringExt};
use tutorlolv2_imports::{AbilityLike, ChampionId, ItemId, RuneId};
use yew::{AttrValue, Classes, Html, Properties, function_component, html};

#[derive(PartialEq)]
pub enum ImageType {
    Ability(ChampionId, AbilityLike),
    Champion(ChampionId),
    Item(ItemId),
    Rune(RuneId),
    Other(AttrValue),
}

impl From<ChampionId> for ImageType {
    fn from(value: ChampionId) -> Self {
        Self::Champion(value)
    }
}

impl From<RuneId> for ImageType {
    fn from(value: RuneId) -> Self {
        Self::Rune(value)
    }
}

impl From<ItemId> for ImageType {
    fn from(value: ItemId) -> Self {
        Self::Item(value)
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
        ImageType::Ability(champion_id, ability) => url!(
            "/img/abilities/{}.avif",
            champion_id.as_str().concat_char(ability.as_char())
        )
        .into(),
        ImageType::Champion(v) => url!("/img/champions/{}.avif", v.as_str()).into(),
        ImageType::Item(v) => url!("/img/items/{}.avif", v.to_riot_id()).into(),
        ImageType::Rune(v) => url!("/img/runes/{}.avif", v.to_riot_id()).into(),
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
