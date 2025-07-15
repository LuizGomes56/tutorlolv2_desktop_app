use crate::{
    pages::calculator::{CalculatorExt, CalculatorState},
    url,
};
use yew::{Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct AbilitySelectorProps {
    pub input_game: CalculatorState,
}

#[function_component(AbilitySelector)]
pub fn ability_selector(props: &AbilitySelectorProps) -> Html {
    let input_game = props.input_game.clone();
    let data = props.input_game.get();
    macro_rules! ability_cell {
        ($field:ident) => {{
            let text = stringify!($field).to_uppercase();
            html! {
                <label class={classes!(
                    "grid", "gap-x-2", "text-white", "grid-cols-[auto_1fr]", "justify-center",
                )}>
                    <div class={classes!("flex", "justify-center", "items-center", "relative")}>
                        <span class={classes!("text-xs", "img-letter")}>{&text}</span>
                        <img
                            loading={"lazy"}
                            class={classes!("h-6", "w-6")}
                            src={url!(
                                "/img/abilities/{}{}.avif",
                                data.active_player.champion_id,
                                text
                            )}
                            alt={""}
                        />
                    </div>
                    <input
                        type={"text"}
                        class={classes!("w-full", "text-center", "text-sm")}
                        placeholder={"0"}
                        value={data.active_player.abilities.$field.to_string()}
                        oninput={{
                            let input_game = input_game.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                let _ = input_game.try_update(|game| {
                                    game.active_player.abilities.$field = target.value().parse::<u8>().unwrap_or(0);
                                });
                            })
                        }}
                    />
                </label>
            }
        }};
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
