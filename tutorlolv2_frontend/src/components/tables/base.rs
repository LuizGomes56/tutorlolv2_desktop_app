use crate::{
    components::tables::cells::{ImageCell, Instances},
    models::base::{Damages, InstanceDamage},
};
use rustc_hash::FxHashMap;
use std::{
    collections::{BTreeMap, btree_map::Values},
    rc::Rc,
};
use yew::{Html, Properties, classes, function_component, html, use_memo};

pub fn damage_cells<T>(btree: Values<'_, T, InstanceDamage>) -> Html {
    html! {
        {
            for btree.map(|value| {
                let text = if value.maximum_damage != 0.0 {
                    format!("{} - {}", value.minimum_damage.round(), value.maximum_damage.round())
                } else {
                    value.minimum_damage.round().to_string()
                };
                html! {
                    <td class={classes!{
                        "text-center", match value.damage_type.as_str() {
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

#[derive(Properties, PartialEq)]
pub struct BaseTableProps {
    pub damaging_abilities: Rc<BTreeMap<String, String>>,
    pub damaging_items: Rc<BTreeMap<usize, String>>,
    pub damaging_runes: Rc<BTreeMap<usize, String>>,
    pub champion_id: String,
    pub damages: FxHashMap<String, Rc<Damages>>,
}

#[function_component(BaseTable)]
pub fn base_table(props: &BaseTableProps) -> Html {
    let thead = {
        let abilities = props.damaging_abilities.clone();
        let items = props.damaging_items.clone();
        let runes = props.damaging_runes.clone();
        let champ_id = props.champion_id.clone();
        use_memo((abilities, items, runes, champ_id), move |_| {
            html! {
                <thead>
                  <tr>
                    <th></th>
                    {
                        for props.damaging_abilities.iter().map(|(key, _)| {
                            let first_char = key.chars().next().unwrap();
                            html! {
                                <th>
                                    <ImageCell
                                        instance={
                                            Instances::Abilities(
                                                key.clone(),
                                                first_char,
                                                props.champion_id.clone()
                                            )
                                        }
                                    />
                                </th>
                            }
                        })
                    }
                    {
                        for props.damaging_items.iter().map(|(key, _)| {
                            html! {
                                <th>
                                    <ImageCell
                                        instance={
                                            Instances::Items(*key)
                                        }
                                    />
                                </th>
                            }
                        })
                    }
                    {
                        for props.damaging_runes.iter().map(|(key, _)| {
                            html! {
                                <th>
                                    <ImageCell
                                        instance={
                                            Instances::Runes(*key)
                                        }
                                    />
                                </th>
                            }
                        })
                    }
                  </tr>
                </thead>
            }
        })
    };

    html! {
        <table class={classes!()}>
            {(*thead).clone()}
            <tbody>
                {
                    for props.damages
                        .iter()
                        .map(|(enemy_champion_id, damages)| {
                            html! {
                                <tr>
                                    <td>
                                        <ImageCell
                                            instance={
                                                Instances::Champions(
                                                    enemy_champion_id.clone(),
                                                )
                                            }
                                        />
                                    </td>
                                    {damage_cells(damages.abilities.values())}
                                    {damage_cells(damages.items.values())}
                                    {damage_cells(damages.runes.values())}
                                </tr>
                            }
                        })
                }
            </tbody>
        </table>
    }
}
