use crate::models::base::InstanceDamage;
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;
use yew::{AttrValue, Html, classes, html};

pub trait ValuesRef<U> {
    type Iter<'a>: Iterator<Item = &'a U>
    where
        Self: 'a,
        U: 'a;

    fn values<'a>(&'a self) -> Self::Iter<'a>;
}

impl<'c, C, U> ValuesRef<U> for &'c C
where
    C: ?Sized + ValuesRef<U>,
{
    type Iter<'a>
        = <C as ValuesRef<U>>::Iter<'a>
    where
        Self: 'a,
        U: 'a;

    fn values<'a>(&'a self) -> Self::Iter<'a> {
        (*self).values()
    }
}

impl<T, U> ValuesRef<U> for Vec<(T, U)> {
    type Iter<'a>
        = std::iter::Map<std::slice::Iter<'a, (T, U)>, fn(&(T, U)) -> &U>
    where
        Self: 'a;
    fn values<'a>(&'a self) -> Self::Iter<'a> {
        fn get_val<T, U>(pair: &(T, U)) -> &U {
            &pair.1
        }
        self.iter().map(get_val::<T, U>)
    }
}

macro_rules! impl_ref {
    ($typename:ident, $stdmod:ident) => {
        paste::paste! {
            impl<T: Eq + std::hash::Hash + Ord, U> ValuesRef<U> for [<$typename>]<T, U> {
                type Iter<'a>
                    = std::collections::[<$stdmod>]::Values<'a, T, U>
                where
                    Self: 'a;

                fn values<'a>(&'a self) -> Self::Iter<'a> {
                    self.values()
                }
            }
        }
    };
}

impl_ref!(BTreeMap, btree_map);
impl_ref!(FxHashMap, hash_map);

pub fn damage_cells<'a, T: ValuesRef<InstanceDamage>>(instances: T) -> Html {
    html! {
        {
            for instances.values().map(|value| {
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
                        "max-w-16", "truncate",
                    }}>
                        {text}
                    </td>
                }
            })
        }
    }
}
