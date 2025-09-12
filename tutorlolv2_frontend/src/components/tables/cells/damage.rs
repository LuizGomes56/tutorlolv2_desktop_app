use crate::models::base::{Attacks, DamageType, DamageValue, InstanceDamage};
use yew::{AttrValue, Html, classes, html};

pub struct DamageCell {
    pub damage_type: DamageType,
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

impl DamageValue {
    #[inline]
    fn into_damage_cell(&self, damage_type: DamageType) -> DamageCell {
        DamageCell {
            damage_type,
            minimum_damage: self.minimum_damage,
            maximum_damage: self.maximum_damage,
        }
    }
}

impl Into<DamageCell> for &InstanceDamage {
    #[inline]
    fn into(self) -> DamageCell {
        DamageCell {
            damage_type: self.damage_type,
            minimum_damage: self.minimum_damage,
            maximum_damage: self.maximum_damage,
        }
    }
}

pub fn damage_cell<'a, T: Into<DamageCell>>(value: T) -> Html {
    let value: DamageCell = value.into();
    let text = if value.maximum_damage != 0 {
        let mut s = value.minimum_damage.to_string();
        s.push_str(" - ");
        s.push_str(&value.maximum_damage.to_string());
        AttrValue::from(s)
    } else {
        AttrValue::from(value.minimum_damage.to_string())
    };
    html! {
        <td title={&text} class={classes!{
            "text-center", "text-sm", "px-2", value.damage_type.get_color(),
            "max-w-24", "truncate",
        }}>
            {text}
        </td>
    }
}

pub trait DisplayDamage {
    fn display_damage(&self) -> Html;
}

impl<T> DisplayDamage for Box<[(T, InstanceDamage)]> {
    fn display_damage(&self) -> Html {
        html! {
            for self.iter().map(|(_, value)| {
                damage_cell(value)
            })
        }
    }
}

impl DisplayDamage for Box<[InstanceDamage]> {
    fn display_damage(&self) -> Html {
        html! {
            for self.iter().map(|value| {
                damage_cell(value)
            })
        }
    }
}

impl DisplayDamage for Attacks {
    fn display_damage(&self) -> Html {
        html! {
            <>
                {damage_cell(self.basic_attack.into_damage_cell(DamageType::Physical))}
                {damage_cell(self.critical_strike.into_damage_cell(DamageType::Physical))}
                {damage_cell(self.onhit_damage.into_damage_cell(DamageType::Mixed))}
            </>
        }
    }
}
