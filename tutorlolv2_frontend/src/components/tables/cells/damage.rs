use crate::models::base::InstanceDamage;
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
                        "text-center", "text-sm", "px-2", match value.damage_type.as_str() {
                            "PHYSICAL_DAMAGE" => "text-orange-500",
                            "MAGIC_DAMAGE" => "text-sky-500",
                            "TRUE_DAMAGE" => "text-white",
                            "ADAPTATIVE_DAMAGE" => "text-pink-500",
                            "MIXED" => "text-violet-500",
                            _ => "text-emerald-500"
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
