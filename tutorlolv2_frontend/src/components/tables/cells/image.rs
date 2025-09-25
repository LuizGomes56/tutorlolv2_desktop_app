use crate::{
    components::{Image, ImageType},
    url,
};
use tutorlolv2_imports::{
    AbilityLike, BASIC_ATTACK_OFFSET, CHAMPION_ABILITIES, CHAMPION_FORMULAS,
    CRITICAL_STRIKE_OFFSET, ChampionId, ITEM_FORMULAS, ItemId, ONHIT_EFFECT_OFFSET, RUNE_FORMULAS,
    RuneId,
};
use yew::{AttrValue, Html, Properties, classes, function_component, html};

#[derive(PartialEq)]
pub enum Instances {
    Attacks,
    Abilities(ChampionId),
    Items(ItemId),
    Runes(RuneId),
    Champions(ChampionId),
}

#[derive(Properties, PartialEq)]
pub struct ImageCellProps {
    pub instance: Instances,
}

#[inline]
fn base_content(
    img_path: ImageType,
    offsets: Option<&'static (u32, u32)>,
    content: Option<Html>,
) -> Html {
    html! {
        <div
            data-offset={offsets.map(|(s, e)| format!("{s},{e}"))}
            class={classes!("flex", "items-center", "justify-center", "relative", "cell")}
        >
            <Image class={classes!("w-8", "h-8")} source={img_path} />
            { content }
        </div>
    }
}

pub fn label_html(ability_like: &AbilityLike) -> Html {
    let (first_char, ability_name) = ability_like.data();
    let (display_label, title) = ability_name.data();

    html! {
        <span title={format!("{first_char} {title}")} class={classes!("text-sm", "img-letter")}>
            {first_char}
            <sub>{ display_label }</sub>
        </span>
    }
}

#[function_component(ImageCell)]
pub fn image_cell(props: &ImageCellProps) -> Html {
    macro_rules! insert_attack {
        ($offset:ident, $url:literal) => {
            html! {
                <th>
                    {base_content(
                        ImageType::Other(AttrValue::Static(url!($url))),
                        Some(&$offset),
                        None,
                    )}
                </th>
            }
        };
    }

    match &props.instance {
        Instances::Attacks => {
            html! {
                <>
                    {insert_attack!(BASIC_ATTACK_OFFSET, "/img/other/basic_attack.png")}
                    {insert_attack!(CRITICAL_STRIKE_OFFSET, "/img/stats/crit_chance.svg")}
                    {insert_attack!(ONHIT_EFFECT_OFFSET, "/img/stats/onhit.svg")}
                </>
            }
        }
        Instances::Abilities(champion_id) => {
            html! {
                CHAMPION_ABILITIES
                    .get(*champion_id as usize).map(|offsets| offsets
                        .iter()
                        .map(|(ability_like, coord)| {
                            html! {
                                <th>
                                    {base_content(
                                        ImageType::Ability(
                                            *champion_id, *ability_like
                                        ),
                                        Some(coord),
                                        Some(label_html(ability_like))
                                    )}
                                </th>
                            }
                        })
                        .collect::<Html>())
                    .unwrap_or_default()
            }
        }
        Instances::Items(item_id) => base_content(
            ImageType::Item(*item_id),
            ITEM_FORMULAS.get(*item_id as usize),
            None,
        ),
        Instances::Runes(rune_id) => base_content(
            ImageType::Rune(*rune_id),
            RUNE_FORMULAS.get(*rune_id as usize),
            None,
        ),
        Instances::Champions(champion_id) => base_content(
            ImageType::Champion(*champion_id),
            CHAMPION_FORMULAS.get(*champion_id as usize),
            None,
        ),
    }
}
