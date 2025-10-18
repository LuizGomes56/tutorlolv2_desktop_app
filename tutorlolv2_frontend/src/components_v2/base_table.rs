use crate::{
    components::{Image, ImageType},
    model_v2::TypeMetadata,
};
use tutorlolv2_imports::{AbilityLike, ChampionId, ItemId, RuneId};
use yew::{Html, Properties, classes, function_component, html};

#[derive(Properties)]
pub struct TheadProps {
    #[prop_or(1)]
    pub skip: u8,
    pub champion_id: ChampionId,
    pub abilities_meta: Box<[TypeMetadata<AbilityLike>]>,
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
}

impl PartialEq for TheadProps {
    fn eq(&self, other: &Self) -> bool {
        self.champion_id == other.champion_id && self.items_meta.len() == other.items_meta.len()
    }
}

fn make_header<T: Copy>(metadata: &[TypeMetadata<T>], closure: impl Fn(T) -> ImageType) -> Html {
    html! {
        for metadata.iter().map(|meta| {
            html! {
                <th class={classes!("justify-items-center")}>
                    <Image
                        class={classes!("w-6", "h-6")}
                        source={closure(meta.kind)}
                    />
                </th>
            }
        })
    }
}

#[function_component(MakeThead)]
pub fn make_thead(props: &TheadProps) -> Html {
    html! {
        <thead>
            {for (0..props.skip).map(|_| html! {<th class={classes!("w-6")}></th>})}
            {make_header(&props.abilities_meta, |v| ImageType::Ability(props.champion_id, v))}
            {make_header(&props.items_meta, |v| ImageType::Item(v))}
            {make_header(&props.runes_meta, |v| ImageType::Rune(v))}
        </thead>
    }
}
