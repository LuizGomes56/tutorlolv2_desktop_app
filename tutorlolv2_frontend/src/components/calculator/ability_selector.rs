use crate::{
    components::calculator::{ChangeAbilityLevelsAction, InputGameAction},
    models::calculator::InputGame,
    url,
};
use paste::paste;
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, UseReducerHandle, classes,
    function_component, html,
};

#[derive(PartialEq, Properties)]
pub struct AbilitySelectorContainerProps {
    pub text: &'static str,
    pub current_player_champion_id: String,
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
                <span class={classes!("text-[13px]", "img-letter")}>{props.text}</span>
                <img
                    loading={"lazy"}
                    class={classes!("h-7", "w-7")}
                    src={url!(
                        "/img/abilities/{}{}.avif",
                        &props.current_player_champion_id,
                        props.text
                    )}
                    alt={""}
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
    pub input_game: UseReducerHandle<InputGame>,
}

#[function_component(AbilitySelector)]
pub fn ability_selector(props: &AbilitySelectorProps) -> Html {
    let data = props.input_game.active_player.abilities;
    macro_rules! ability_cell {
        ($field:ident) => {
            paste! {
                html! {
                    <AbilitySelectorContainer
                        value={data.$field}
                        text={stringify!([<$field:upper>])}
                        current_player_champion_id={props.input_game.active_player.champion_id.clone()}
                        oninput={{
                            let input_game = props.input_game.clone();
                            let max = match props.input_game.active_player.champion_id.as_str() {
                                "Jayce" | "Elise" | "Udyr" => 6,
                                _ => 5
                            };
                            Callback::from(move |e: InputEvent| {
                                let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                let value = target.value().parse::<u8>().unwrap_or(0).clamp(0, max);
                                input_game.dispatch(
                                    InputGameAction::SetAbilityLevels(
                                        ChangeAbilityLevelsAction::[<Set $field:upper>](value)
                                    )
                                );
                            })
                        }}
                    />
                }
            }
        };
    }

    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-1",
        )}>
            {ability_cell!(q)}
            {ability_cell!(w)}
            {ability_cell!(e)}
            {ability_cell!(r)}
        </div>
    }
}
