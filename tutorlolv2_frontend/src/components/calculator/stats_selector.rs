use crate::{
    color,
    components::calculator::{ChangeStatsAction, InputGameAction},
    models::calculator::InputGame,
    url,
};
use paste::paste;
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, UseReducerHandle, classes,
    function_component, html,
};

#[derive(PartialEq, Properties)]
struct StatsCellProps<T: ToString + PartialEq> {
    path: &'static str,
    display: &'static str,
    value: T,
    oninput: Callback<InputEvent>,
    disabled: bool,
}

#[function_component(StatsCell)]
fn stats_cell<T: ToString + PartialEq>(props: &StatsCellProps<T>) -> Html {
    html! {
        <>
            <span class={classes!("flex", "items-center", "justify-center", "relative")}>
                <img
                    loading={"lazy"}
                    class={classes!("h-3.5", "w-3.5")}
                    src={url!("/img/stats/{}", props.path)}
                    alt={""}
                />
            </span>
            <span>{props.display}</span>
            <input
                type={"number"}
                class={classes!(
                    "text-center", "min-w-0",
                    if props.disabled { color!(text-400) }
                    else { "text-white" }
                )}
                disabled={props.disabled}
                placeholder={"0"}
                value={props.value.to_string()}
                oninput={props.oninput.clone()}
            />
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct StatsSelectorProps {
    pub input_game: UseReducerHandle<InputGame>,
}

#[function_component(StatsSelector)]
pub fn stats_selector(props: &StatsSelectorProps) -> Html {
    let stats = props.input_game.active_player.champion_stats;

    macro_rules! stat_cell {
        (@dispatch $field:ident) => {
            paste! {{
                let input_game = props.input_game.clone();
                Callback::from(move |e: InputEvent| {
                    let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    let value = target.value().parse::<f64>().unwrap_or(0.0).max(0.0);
                    input_game.dispatch(
                        InputGameAction::SetCurrentPlayerStats(
                            ChangeStatsAction::[<Set $field:camel>](value)
                        )
                    );
                })
            }}
        };
        ($path:literal, $stat:ident, $display:literal) => {
            html! {
                <StatsCell<f64>
                    path={$path}
                    disabled={props.input_game.active_player.infer_stats}
                    value={stats.$stat.round()}
                    display={$display}
                    oninput={stat_cell!(@dispatch $stat)}
                />
            }
        };
    }

    html! {
        <div class={classes!(
            "grid", "grid-cols-[auto_auto_1fr]", "text-white", "items-center",
            "gap-x-3", "text-sm", "gap-y-1", "px-4"
        )}>
            <StatsCell<u8>
                path={"level.avif"}
                value={props.input_game.active_player.level}
                display={"Level"}
                oninput={
                    let input_game = props.input_game.clone();
                    Callback::from(move |e: InputEvent| {
                        let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        let value = target.value().parse::<u8>().unwrap_or(1).clamp(1, 18);
                        input_game.dispatch(InputGameAction::SetCurrentPlayerLevel(value));
                    })
                }
                disabled={false}
            />
            {stat_cell!("attack_damage.avif", attack_damage,  "Attack Damage")}
            {stat_cell!("ability_power.svg", ability_power, "Ability Power")}
            {stat_cell!("health.svg", max_health, "Max Health")}
            {stat_cell!("health.svg", current_health, "Current Health")}
            {stat_cell!("armor.svg", armor, "Armor")}
            {stat_cell!("armor_penetration.svg", armor_penetration_flat, "Armor Pen Flat")}
            {stat_cell!("armor_penetration.svg", armor_penetration_percent, "Armor Pen %")}
            {stat_cell!("magic_resist.svg", magic_resist, "Magic Resist")}
            {stat_cell!("magic_penetration.svg", magic_penetration_flat, "Magic Pen Flat")}
            {stat_cell!("magic_penetration.svg", magic_penetration_percent, "Magic Pen %")}
            {stat_cell!("crit_chance.avif", crit_chance, "Crit Chance")}
            {stat_cell!("crit_damage.svg", crit_damage, "Crit Damage")}
            {stat_cell!("attack_speed.avif", attack_speed, "Attack Speed")}
            {stat_cell!("mana.svg", max_mana, "Max Mana")}
            {stat_cell!("mana.svg", current_mana, "Current Mana")}
        </div>
    }
}
