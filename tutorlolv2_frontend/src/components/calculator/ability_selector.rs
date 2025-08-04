use crate::{
    components::{Image, ImageType, calculator::ChangeAbilityLevelsAction},
    models::base::AbilityLevels,
};
use paste::paste;
use yew::{
    AttrValue, Callback, Html, InputEvent, Properties, TargetCast, classes, function_component,
    html,
};

#[derive(PartialEq, Properties)]
pub struct AbilitySelectorContainerProps {
    pub text: &'static str,
    pub current_player_champion_id: AttrValue,
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
                <span class={classes!("text-sm", "img-letter")}>{props.text}</span>
                <Image
                    class={classes!("w-8", "h-8")}
                    source={ImageType::Abilities(
                        format!(
                            "{}{}",
                            &props.current_player_champion_id,
                            props.text
                        )
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
    pub current_player_champion_id: AttrValue,
    pub callback: Callback<ChangeAbilityLevelsAction>,
}

#[function_component(AbilitySelector)]
pub fn ability_selector(props: &AbilitySelectorProps) -> Html {
    macro_rules! ability_cell {
        ($field:ident) => {
            paste! {
                html! {
                    <AbilitySelectorContainer
                        value={props.ability_levels.$field}
                        text={stringify!([<$field:upper>])}
                        current_player_champion_id={props.current_player_champion_id.clone()}
                        oninput={{
                            let callback = props.callback.clone();
                            let max = match props.current_player_champion_id.as_str() {
                                "Jayce" | "Elise" | "Udyr" => 6,
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
        <div class={classes!(
            "flex", "flex-col", "gap-2",
        )}>
            {ability_cell!(q)}
            {ability_cell!(w)}
            {ability_cell!(e)}
            {ability_cell!(r)}
        </div>
    }
}
