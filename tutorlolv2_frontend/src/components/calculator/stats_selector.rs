use std::rc::Rc;

use crate::{
    models::base::Stats,
    pages::calculator::{CalculatorExt, CalculatorState},
    url,
    utils::to_pascal_case,
};
use paste::paste;
use yew::{
    Callback, Html, InputEvent, Properties, Reducible, TargetCast, classes, function_component,
    html, use_effect_with, use_reducer_eq,
};

#[derive(PartialEq, Properties)]
pub struct StatsSelectorProps {
    pub input_game: CalculatorState,
}

macro_rules! actions_enum {
    ($name:ident { $($field:ident),* }) => {
        paste! {
            pub enum $name {
                $(
                    [<Set $field:camel>](f64),
                )*
            }

            fn match_reducer_action(action: $name, new_field: &mut Stats) {
                match action {
                    $(
                        $name::[<Set $field:camel>](value) => new_field.$field = value,
                    )*
                }
            }
        }
    };
}

actions_enum!(StatsAction {
    ability_power,
    armor,
    armor_penetration_flat,
    armor_penetration_percent,
    attack_damage,
    // attack_range,
    attack_speed,
    crit_chance,
    crit_damage,
    current_health,
    magic_penetration_flat,
    magic_penetration_percent,
    magic_resist,
    max_health,
    max_mana,
    current_mana
});

impl Reducible for Stats {
    type Action = StatsAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_field = *self;
        match_reducer_action(action, &mut new_field);
        Rc::new(new_field)
    }
}

#[function_component(StatsSelector)]
pub fn stats_selector(props: &StatsSelectorProps) -> Html {
    let data = props.input_game.get();
    // to be externalized
    let stats_handle = use_reducer_eq(Stats::default);

    let cloned_stats_handle = stats_handle.clone();
    use_effect_with(stats_handle.clone(), move |_| {
        web_sys::console::log_1(&format!("{:#?}", *cloned_stats_handle).into());
    });

    macro_rules! stat_cell {
        (@inner $field:ident, $stat:expr, $text:expr, $value:expr) => {
            paste! {
                html! {
                    <>
                        <span class={classes!("flex", "items-center", "justify-center", "relative")}>
                            <img
                                loading={"lazy"}
                                class={classes!("h-3.5", "w-3.5")}
                                src={url!("/img/stats/{}.avif", $stat)}
                                alt={""}
                            />
                        </span>
                        <span>{to_pascal_case($text)}</span>
                        <input
                            type={"text"}
                            class={classes!("text-center", "min-w-0")}
                            placeholder={"0"}
                            value={(*stats_handle).$field.to_string()}
                            oninput={{
                                let stats_handle = stats_handle.clone();
                                Callback::from(move |e: InputEvent| {
                                    let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = target.value().parse::<f64>().unwrap_or(0.0);
                                    stats_handle.dispatch(StatsAction::[<Set $field:camel>](value))
                                })
                            }}
                        />
                    </>
                }
            }
        };
        ($stat:ident) => {{
            let value = data.active_player.champion_stats.$stat;
            let text = stringify!($stat);
            stat_cell!(@inner $stat, text, text, value)
        }};
        (@$img_path:ident $stat:ident) => {{
            let value = data.active_player.champion_stats.$stat;
            stat_cell!(@inner $stat, stringify!($img_path), stringify!($stat), value)
        }};
        (%$img_path:ident $stat:ident) => {{
            let value = data.active_player.champion_stats.$stat;
            let text = concat!(stringify!($img_path), " %");
            stat_cell!(@inner $stat, stringify!($img_path), text, value)
        }};
    }

    html! {
        <div class={classes!(
            "grid", "grid-cols-[auto_auto_1fr]", "text-white", "items-center",
            "gap-x-2", "text-sm", "gap-y-1"
        )}>
            {stat_cell!(attack_damage)}
            {stat_cell!(ability_power)}
            {stat_cell!(@health max_health)}
            {stat_cell!(@health current_health)}
            {stat_cell!(armor)}
            {stat_cell!(@armor_penetration armor_penetration_flat)}
            {stat_cell!(%armor_penetration armor_penetration_percent)}
            {stat_cell!(magic_resist)}
            {stat_cell!(@magic_penetration magic_penetration_flat)}
            {stat_cell!(%magic_penetration magic_penetration_percent)}
            {stat_cell!(crit_chance)}
            {stat_cell!(crit_damage)}
            {stat_cell!(attack_speed)}
            {stat_cell!(@mana max_mana)}
            {stat_cell!(@mana current_mana)}
        </div>
    }
}
