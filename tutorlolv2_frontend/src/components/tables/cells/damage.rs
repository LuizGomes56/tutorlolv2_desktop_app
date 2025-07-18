use crate::models::base::InstanceDamage;
use std::collections::BTreeMap;
use yew::{Html, classes, html};

pub fn damage_cells<T>(btree: &BTreeMap<T, InstanceDamage>) -> Html {
    html! {
        {
            for btree.values().map(|value| {
                let text = if value.maximum_damage != 0.0 {
                    format!("{} - {}", value.minimum_damage.round(), value.maximum_damage.round())
                } else {
                    value.minimum_damage.round().to_string()
                };
                html! {
                    <td class={classes!{
                        "text-center", "text-sm", "px-2", match value.damage_type.as_str() {
                            "PHYSICAL_DAMAGE" => "text-orange-500",
                            "MAGIC_DAMAGE" => "text-sky-500",
                            "TRUE_DAMAGE" => "text-white",
                            "ADAPTATIVE_DAMAGE" => "text-pink-500",
                            "MIXED" => "text-violet-500",
                            _ => "text-emerald-500"
                        }
                    }}>
                        {text}
                    </td>
                }
            })
        }
    }
}
