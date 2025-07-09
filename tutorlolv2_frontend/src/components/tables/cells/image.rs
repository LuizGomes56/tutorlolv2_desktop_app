use yew::{Html, Properties, classes, function_component, html};

use crate::url;

#[derive(PartialEq)]
pub enum Instances {
    Abilities(String, char, String),
    Items(usize),
    Runes(usize),
    Champions(String),
}

#[derive(Properties, PartialEq)]
pub struct ImageCellProps {
    pub instance: Instances,
}

#[function_component(ImageCell)]
pub fn image_cell(props: &ImageCellProps) -> Html {
    let (img_path, content) = match &props.instance {
        Instances::Abilities(keyname, first_char, champion_id) => match first_char {
            'A' | 'C' => (
                url!("/cdn/abilities/{}.png", first_char),
                html! {
                    <span class={classes!("img-letter")}>
                        {first_char}
                    </span>
                },
            ),
            _ => (
                url!("/cdn/abilities/{}{}.png", champion_id, first_char),
                html! {
                    <span class={classes!("img-letter")}>
                        {first_char}
                        <sub>
                            {
                                keyname
                                    .chars()
                                    .filter(|c| *c != '_')
                                    .skip(1)
                                    .take(3)
                                    .collect::<String>()
                            }
                        </sub>
                    </span>
                },
            ),
        },
        Instances::Items(keyname) => (url!("/cdn/items/{}.png", keyname), html!()),
        Instances::Runes(keyname) => (url!("/cdn/runes/{}.png", keyname), html!()),
        Instances::Champions(champion_id) => (url!("/cdn/champions/{}.png", champion_id), html!()),
    };

    html! {
        <div class={classes!(
            "flex", "items-center", "justify-center",
            "relative"
        )}>
            <img
                class={classes!(
                    "w-8", "h-8"
                )}
                src={img_path}
                alt={""}
            />
            { content }
        </div>
    }
}
