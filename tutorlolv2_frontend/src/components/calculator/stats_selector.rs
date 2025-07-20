use crate::{color, components::calculator::ChangeStatsAction, models::base::Stats, url};
use paste::paste;
use rustc_hash::FxHashMap;
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html, use_memo,
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
                value={
                    if props.disabled {
                        Some(props.value.to_string())
                    } else {
                        None
                    }
                }
                oninput={props.oninput.clone()}
            />
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct StatsSelectorProps {
    pub infer_stats: bool,
    pub champion_stats: Stats,
    pub set_stats_callback: Callback<ChangeStatsAction>,
    pub set_level_callback: Callback<u8>,
    pub level: u8,
}

#[function_component(StatsSelector)]
pub fn stats_selector(props: &StatsSelectorProps) -> Html {
    macro_rules! generate_callback {
        ($field:ident) => {
            paste! {{
                let set_stats_callback = props.set_stats_callback.clone();
                Callback::from(move |e: InputEvent| {
                    let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    let value = target.value().parse::<f64>().unwrap_or(0.0).max(0.0);
                    set_stats_callback.emit(ChangeStatsAction::[<Set $field:camel>](value));
                })
            }}
        };
    }

    let memo_callbacks = use_memo((), move |_| {
        let mut callback_map = FxHashMap::default();
        callback_map.insert("attack_damage", generate_callback!(attack_damage));
        callback_map.insert("ability_power", generate_callback!(ability_power));
        callback_map.insert("max_health", generate_callback!(max_health));
        callback_map.insert("current_health", generate_callback!(current_health));
        callback_map.insert("armor", generate_callback!(armor));
        callback_map.insert(
            "armor_penetration_flat",
            generate_callback!(armor_penetration_flat),
        );
        callback_map.insert(
            "armor_penetration_percent",
            generate_callback!(armor_penetration_percent),
        );
        callback_map.insert("magic_resist", generate_callback!(magic_resist));
        callback_map.insert(
            "magic_penetration_flat",
            generate_callback!(magic_penetration_flat),
        );
        callback_map.insert(
            "magic_penetration_percent",
            generate_callback!(magic_penetration_percent),
        );
        callback_map.insert("crit_chance", generate_callback!(crit_chance));
        callback_map.insert("crit_damage", generate_callback!(crit_damage));
        callback_map.insert("attack_speed", generate_callback!(attack_speed));
        callback_map.insert("max_mana", generate_callback!(max_mana));
        callback_map.insert("current_mana", generate_callback!(current_mana));
        callback_map
    });

    macro_rules! stat_cell {
        ($path:literal, $stat:ident, $display:literal) => {
            html! {
                <StatsCell<f64>
                    path={$path}
                    disabled={props.infer_stats}
                    value={props.champion_stats.$stat.round()}
                    display={$display}
                    oninput={memo_callbacks.get(stringify!($stat)).unwrap().clone()}
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
                path={"level.svg"}
                value={props.level}
                display={"Level"}
                oninput={
                    let set_level_callback = props.set_level_callback.clone();
                    Callback::from(move |e: InputEvent| {
                        let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        let value = target.value().parse::<u8>().unwrap_or(1).clamp(1, 18);
                        set_level_callback.emit(value);
                    })
                }
                disabled={false}
            />
            {stat_cell!("attack_damage.svg", attack_damage,  "Attack Damage")}
            {stat_cell!("ability_power.svg", ability_power, "Ability Power")}
            {stat_cell!("health.svg", max_health, "Max Health")}
            {stat_cell!("health.svg", current_health, "Current Health")}
            {stat_cell!("armor.svg", armor, "Armor")}
            {stat_cell!("armor_penetration.svg", armor_penetration_flat, "Armor Pen Flat")}
            {stat_cell!("armor_penetration.svg", armor_penetration_percent, "Armor Pen %")}
            {stat_cell!("magic_resist.svg", magic_resist, "Magic Resist")}
            {stat_cell!("magic_penetration.svg", magic_penetration_flat, "Magic Pen Flat")}
            {stat_cell!("magic_penetration.svg", magic_penetration_percent, "Magic Pen %")}
            {stat_cell!("crit_chance.svg", crit_chance, "Crit Chance")}
            {stat_cell!("crit_damage.svg", crit_damage, "Crit Damage")}
            {stat_cell!("attack_speed.svg", attack_speed, "Attack Speed")}
            {stat_cell!("mana.svg", max_mana, "Max Mana")}
            {stat_cell!("mana.svg", current_mana, "Current Mana")}
        </div>
    }
}
