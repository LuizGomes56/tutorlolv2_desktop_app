use crate::{
    color,
    components::tables::cells::{ImageCell, Instances, damage_cells},
    models::base::Damages,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};
use yew::{Html, Properties, classes, function_component, html, use_memo};

#[derive(Properties, PartialEq)]
pub struct BaseTableProps {
    pub damaging_abilities: Rc<BTreeSet<String>>,
    pub damaging_items: Rc<BTreeSet<usize>>,
    pub damaging_runes: Rc<BTreeSet<usize>>,
    pub champion_id: String,
    pub damages: BTreeMap<String, Rc<Damages>>,
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
                    <tr class={classes!(
                        color!(odd:bg-950),
                    )}>
                    <th class={classes!("h-10")}></th>
                    {
                        for props.damaging_abilities.iter().map(|key| {
                            let first_char = key.chars().next().unwrap_or_default();
                            html! {
                                <th class={classes!("group", "min-w-10")}>
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
                        for props.damaging_items.iter().map(|key| {
                            html! {
                                <th class={classes!("group", "min-w-10")}>
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
                        for props.damaging_runes.iter().map(|key| {
                            html! {
                                <th class={classes!("group", "min-w-10")}>
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
                                <tr class={classes!(
                                    // color!(odd:bg-900), color!(even:bg-800)
                                )}>
                                    <td class={classes!("w-10", "h-10")}>
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
