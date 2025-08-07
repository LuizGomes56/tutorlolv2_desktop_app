use crate::models::base::{DamageType, InstanceDamage};
use yew::{AttrValue, Html, classes, html};

pub fn damage_cells<'a, T>(instances: &[(T, InstanceDamage)]) -> Html {
    html! {
        {
            for instances.iter().map(|(_, value)| {
                let text = if value.maximum_damage != 0.0 {
                    let mut s = value.minimum_damage.round().to_string();
                    s.push_str(" - ");
                    s.push_str(&value.maximum_damage.round().to_string());
                    AttrValue::from(s)
                } else {
                    AttrValue::from(value.minimum_damage.round().to_string())
                };
                html! {
                    <td title={&text} class={classes!{
                        "text-center", "text-sm", "px-2", match value.damage_type {
                            DamageType::Physical => "text-orange-500",
                            DamageType::Magic => "text-sky-500",
                            DamageType::True => "text-white",
                            DamageType::Adaptative => "text-pink-500",
                            DamageType::Mixed => "text-violet-500",
                            DamageType::Unknown => "text-emerald-500"
                        },
                        "max-w-24", "truncate",
                    }}>
                        {text}
                    </td>
                }
            })
        }
    }
}
