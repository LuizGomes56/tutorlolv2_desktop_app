use crate::{models::calculator::InputGame, url, utils::to_pascal_case};
use std::{cell::RefCell, rc::Rc};
use yew::{Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct StatsSelectorProps {
    pub data: Rc<RefCell<InputGame>>,
}

#[function_component(StatsSelector)]
pub fn stats_selector(props: &StatsSelectorProps) -> Html {
    macro_rules! stat_cell {
        (@inner $field:ident, $stat:expr, $text:expr, $value:expr) => {
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
                        value={$value.to_string()}
                        oninput={{
                            let data = props.data.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                (*data)
                                    .borrow_mut()
                                    .active_player
                                    .champion_stats
                                    .$field = target.value().parse::<f64>().unwrap_or(0.0);
                            })
                        }}
                    />
                </>
            }
        };
        ($stat:ident) => {{
            let value = (*props.data).borrow().active_player.champion_stats.$stat;
            let text = stringify!($stat);
            stat_cell!(@inner $stat, text, text, value)
        }};
        (@$img_path:ident $stat:ident) => {{
            let value = (*props.data).borrow().active_player.champion_stats.$stat;
            stat_cell!(@inner $stat, stringify!($img_path), stringify!($stat), value)
        }};
        (%$img_path:ident $stat:ident) => {{
            let value = (*props.data).borrow().active_player.champion_stats.$stat;
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
