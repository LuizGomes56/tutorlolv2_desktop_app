use crate::{
    color,
    components::tables::cells::{ImageCell, Instances},
};
use std::collections::BTreeSet;
use yew::{AttrValue, Html, Properties, classes, function_component, html, use_memo};

#[derive(Properties, PartialEq)]
pub struct BaseTableProps {
    pub damaging_abilities: BTreeSet<String>,
    pub damaging_items: BTreeSet<u32>,
    pub damaging_runes: BTreeSet<u32>,
    pub champion_id: AttrValue,
    pub damages: Html,
}

#[function_component(BaseTable)]
pub fn base_table(props: &BaseTableProps) -> Html {
    let thead = {
        let abilities = props.damaging_abilities.clone();
        let items = props.damaging_items.clone();
        let runes = props.damaging_runes.clone();
        let champion_id = props.champion_id.clone();
        use_memo((abilities, items, runes, champion_id), move |_| {
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
                                    <ImageCell instance={Instances::Items(*key)} />
                                </th>
                            }
                        })
                    }
                    {
                        for props.damaging_runes.iter().map(|key| {
                            html! {
                                <th class={classes!("group", "min-w-10")}>
                                    <ImageCell instance={Instances::Runes(*key)} />
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
            <tbody>{props.damages.clone()}</tbody>
        </table>
    }
}
