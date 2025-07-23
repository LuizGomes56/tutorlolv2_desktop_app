use crate::{STATIC_SPRITE_MAP, svg, url};
use yew::{Classes, Html, Properties, function_component, html};

#[derive(PartialEq)]
pub enum SpriteType {
    Abilities(String),
    Champions(String),
    Items(u32),
    Other(String),
}

#[derive(PartialEq, Properties)]
pub struct SpriteProps {
    pub source: SpriteType,
    pub size: u32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Sprite)]
pub fn sprite(props: &SpriteProps) -> Html {
    let /* mut */ class_attr = props.class.clone();
    // class_attr.push("deferred"); impl lazy loading

    if let SpriteType::Other(src) = &props.source {
        return html! {
            <img
                loading={"lazy"}
                class={class_attr}
                src={src.clone()}
                alt={""}
                width={props.size.to_string()}
                height={props.size.to_string()}
            />
        };
    }

    let sprite = STATIC_SPRITE_MAP.get().and_then(|map| match &props.source {
        SpriteType::Abilities(name) => map.abilities.get(name),
        SpriteType::Champions(name) => map.champions.get(name),
        SpriteType::Items(id) => map.items.get(id),
        _ => None,
    });

    if let Some(s) = sprite {
        let col = s.x / s.w;
        let row = s.y / s.h;

        let folder = match props.source {
            SpriteType::Abilities(_) => "abilities",
            SpriteType::Champions(_) => "champions",
            SpriteType::Items(_) => "items",
            _ => "unreachable",
        };
        let sprite_url = url!("/sprite/{}/sprite_{}.avif", folder, s.f);

        let x_off = col * props.size;
        let y_off = row * props.size;
        let bg_width = 8 * props.size;

        let style = format!(
            "width: {size}px; \
            height: {size}px; \
            background-image: url('{sprite_url}'); \
            background-position: -{x_off}px -{y_off}px; \
            background-size: {bg_width}px auto;",
            size = props.size,
        );

        html! {
            <div class={class_attr} style={style} />
        }
    } else {
        html! {
            <div class={class_attr} style={"color: #ffffff;"}>
                {svg!("../../public/image_missing", &props.size.to_string())}
            </div>
        }
    }
}
