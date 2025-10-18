use crate::{
    calculator_v2::reducer::*,
    components::{Image, ImageType},
    model_v2::*,
    utils::RandomInput,
};
use paste::paste;
use tutorlolv2_imports::{AbilityLike, AbilityName, ChampionId};
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
    let random_id = RandomInput::rand_id();
    html! {
        <label for={&random_id} class={classes!(
            "flex", "flex-col", "text-white", "justify-center",
        )}>
            <input
                id={&random_id}
                type={"number"}
                class={classes!("w-full", "text-center", "text-sm", "pt-1.5", "pb-1", "bg-transparent")}
                placeholder={"0"}
                value={props.value.to_string()}
                oninput={props.oninput.clone()}
            />
            <div class={classes!("flex", "justify-center", "items-center", "relative")}>
                <span class={classes!("text-sm", "img-letter")}>{props.text.data().0}</span>
                <Image
                    class={classes!("w-8", "h-8")}
                    source={ImageType::Ability(
                        props.current_player_champion_id,
                        props.text,
                    )}
                />
            </div>
        </label>
    }
}

#[derive(PartialEq, Properties)]
pub struct AbilitySelectorProps {
    pub ability_levels: AbilityLevels,
    pub current_player_champion_id: ChampionId,
    pub callback: Callback<AbilityLevels>,
}

#[function_component(AbilitySelector)]
pub fn ability_selector(props: &AbilitySelectorProps) -> Html {
    let ability_levels = props.ability_levels;

    let make_container = |value: u8,
                          enum_fn: fn(AbilityName) -> AbilityLike,
                          cb_fn: fn(u8, AbilityLevels) -> AbilityLevels| {
        html! {
            <AbilitySelectorContainer
                value={value}
                text={enum_fn(AbilityName::Void)}
                current_player_champion_id={props.current_player_champion_id}
                oninput={{
                    let callback = props.callback.clone();
                    Callback::from(move |e: InputEvent| {
                        let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        let value = target.value().parse::<u8>().unwrap_or(0);
                        callback.emit(cb_fn(value, ability_levels));
                    })
                }}
            />
        }
    };

    html! {
        <>
            {make_container(ability_levels.q, AbilityLike::Q, |v, args| AbilityLevels {q: v, ..args})}
            {make_container(ability_levels.w, AbilityLike::W, |v, args| AbilityLevels {w: v, ..args})}
            {make_container(ability_levels.e, AbilityLike::E, |v, args| AbilityLevels {e: v, ..args})}
            {make_container(ability_levels.r, AbilityLike::R, |v, args| AbilityLevels {r: v, ..args})}
        </>
    }
}
