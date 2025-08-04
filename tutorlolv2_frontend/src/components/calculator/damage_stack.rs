use generated_code::CHAMPION_ABILITIES;
use yew::{AttrValue, Html, Properties, classes, function_component, html};

use crate::components::{Image, ImageType};

#[function_component(DamageStackTable)]
pub fn damage_stack_table() -> Html {
    html! {}
}

#[derive(Properties, PartialEq)]
pub struct DamageStackSelectorProps {
    pub champion_id: AttrValue,
}

#[function_component(DamageStackSelector)]
pub fn damage_stack_selector(props: &DamageStackSelectorProps) -> Html {
    html! {
        <div>
            {
                CHAMPION_ABILITIES
                    .get(props.champion_id.as_str())
                    .and_then(|value| {
                        Some(
                            value
                            .keys()
                            .map(|ability_name| {
                                let first_char = ability_name
                                    .chars()
                                    .next()
                                    .unwrap_or_default();
                                html! {
                                    <div class={classes!(
                                        "flex", "items-center", "justify-center",
                                        "relative", "cell"
                                    )}>
                                        <Image
                                            class={classes!("w-8", "h-8")}
                                            source={ImageType::Abilities(
                                                format!(
                                                    "{}{}",
                                                    props.champion_id,
                                                    first_char
                                                )
                                            )}
                                        />
                                        <span class={classes!("text-sm", "img-letter")}>
                                            {first_char}
                                            <sub>
                                                {
                                                    ability_name
                                                        .chars()
                                                        .filter(|c| *c != '_')
                                                        .skip(1)
                                                        .take(3)
                                                        .collect::<String>()
                                                }
                                            </sub>
                                        </span>
                                    </div>
                                }
                            })
                            .collect::<Html>()
                        )
                    })
                    .unwrap_or_default()
            }
        </div>
    }
}

#[function_component(DamageStackSelected)]
pub fn damage_stack_selected() -> Html {
    html! {}
}
