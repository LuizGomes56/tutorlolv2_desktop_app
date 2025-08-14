use crate::{
    components::{Image, ImageType, calculator::ChangeAbilityLevelsAction},
    models::base::AbilityLevels,
};
use generated_code::{AbilityLike, AbilityName, ChampionId};
use paste::paste;
use yew::{Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct AbilitySelectorContainerProps {
    pub text: AbilityLike,
    pub current_player_champion_id: ChampionId,
    pub value: u8,
    pub oninput: Callback<InputEvent>,
}

#[function_component(AbilitySelectorContainer)]
pub fn ability_selector_container(props: &AbilitySelectorContainerProps) -> Html {
    html! {
        <label class={classes!(
            "grid", "gap-x-2", "text-white", "grid-cols-[auto_1fr]", "justify-center",
        )}>
            <div class={classes!("flex", "justify-center", "items-center", "relative")}>
                <span class={classes!("text-sm", "img-letter")}>{props.text.as_char()}</span>
                <Image
                    class={classes!("w-8", "h-8")}
                    source={ImageType::Abilities(
                        props.current_player_champion_id,
                        props.text,
                    )}
                />
            </div>
            <input
                type={"number"}
                class={classes!("w-full", "text-center", "text-sm")}
                placeholder={"0"}
                value={props.value.to_string()}
                oninput={props.oninput.clone()}
            />
        </label>
    }
}

#[derive(PartialEq, Properties)]
pub struct AbilitySelectorProps {
    pub ability_levels: AbilityLevels,
    pub current_player_champion_id: ChampionId,
    pub callback: Callback<ChangeAbilityLevelsAction>,
}

#[function_component(AbilitySelector)]
pub fn ability_selector(props: &AbilitySelectorProps) -> Html {
    macro_rules! ability_cell {
        ($field:ident, $ability:expr) => {
            paste! {
                html! {
                    <AbilitySelectorContainer
                        value={props.ability_levels.$field}
                        text={$ability}
                        current_player_champion_id={props.current_player_champion_id}
                        oninput={{
                            let callback = props.callback.clone();
                            let max = match props.current_player_champion_id {
                                ChampionId::Jayce | ChampionId::Elise | ChampionId::Udyr => 6,
                                _ => 5
                            };
                            Callback::from(move |e: InputEvent| {
                                let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                let value = target.value().parse::<u8>().unwrap_or(0).clamp(0, max);
                                callback.emit(ChangeAbilityLevelsAction::[<Set $field:upper>](value));
                            })
                        }}
                    />
                }
            }
        };
    }

    html! {
        <>
            {ability_cell!(q, AbilityLike::Q(AbilityName::Void))}
            {ability_cell!(w, AbilityLike::W(AbilityName::Void))}
            {ability_cell!(e, AbilityLike::E(AbilityName::Void))}
            {ability_cell!(r, AbilityLike::R(AbilityName::Void))}
        </>
    }
}
