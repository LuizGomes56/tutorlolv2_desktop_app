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
                        "text-center", "text-sm", "px-2", value.damage_type.get_color(),
                        "max-w-24", "truncate",
                    }}>
                        {text}
                    </td>
                }
            })
        }
    }
}
