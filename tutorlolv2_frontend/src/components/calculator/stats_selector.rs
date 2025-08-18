use crate::{
    color,
    components::{Image, ImageType, calculator::ChangeStatsAction},
    models::base::Stats,
    url,
};
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html,
    use_callback,
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
                <Image
                    class={classes!("h-3.5", "w-3.5")}
                    source={ImageType::Other(url!("/img/stats/{}", props.path).into())}
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
    pub infer_stats: bool,
    pub champion_stats: Stats,
    pub set_stats_callback: Callback<ChangeStatsAction>,
    pub set_level_callback: Callback<u8>,
    pub level: u8,
}

#[function_component(StatsSelector)]
pub fn stats_selector(props: &StatsSelectorProps) -> Html {
    let set_level_callback = {
        let set_level_callback = props.set_level_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<u8>().unwrap_or(1).clamp(1, 18);
            set_level_callback.emit(value);
        })
    };
    let set_attack_damage_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetAttackDamage(value));
        })
    };
    let set_ability_power_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetAbilityPower(value));
        })
    };
    let set_max_health_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetMaxHealth(value));
        })
    };
    let set_current_health_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetCurrentHealth(value));
        })
    };
    let set_armor_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetArmor(value));
        })
    };
    let set_armor_penetration_flat_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetArmorPenetrationFlat(value));
        })
    };
    let set_armor_penetration_percent_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetArmorPenetrationPercent(value));
        })
    };
    let set_magic_resist_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetMagicResist(value));
        })
    };
    let set_magic_penetration_flat_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetMagicPenetrationFlat(value));
        })
    };
    let set_magic_penetration_percent_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetMagicPenetrationPercent(value));
        })
    };
    let set_crit_chance_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetCritChance(value));
        })
    };
    let set_crit_damage_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetCritDamage(value));
        })
    };
    let set_attack_speed_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetAttackSpeed(value));
        })
    };
    let set_max_mana_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetMaxMana(value));
        })
    };
    let set_current_mana_callback = {
        let set_stats_callback = props.set_stats_callback.clone();
        use_callback((), move |e: InputEvent, _| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0).max(0.0);
            set_stats_callback.emit(ChangeStatsAction::SetCurrentMana(value));
        })
    };

    macro_rules! stat_cell {
        ($path:literal, $stat:ident, $display:literal) => {
            paste::paste! {
                html! {
                    <StatsCell<f32>
                        path={$path}
                        disabled={props.infer_stats}
                        value={props.champion_stats.$stat.round()}
                        display={$display}
                        oninput={[<set_ $stat _callback>].clone()}
                    />
                }
            }
        };
    }

    html! {
        <div class={classes!(
            "grid", "grid-cols-[auto_auto_1fr]", "text-white", "items-center",
            "gap-x-3", "gap-y-1", "px-4", "mb-2",
        )}>
            <StatsCell<u8>
                path={"level.svg"}
                value={props.level}
                display={"Level"}
                oninput={set_level_callback}
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
