use crate::components::tables::cells::{ImageCell, Instances};
use generated_code::{ChampionId, ItemId, RuneId};
use yew::{Html, Properties, classes, function_component, html, use_memo};

#[derive(Properties, PartialEq)]
pub struct BaseTableProps {
    pub damaging_items: Vec<ItemId>,
    pub damaging_runes: Vec<RuneId>,
    pub champion_id: ChampionId,
    pub damages: Html,
}

#[function_component(BaseTable)]
pub fn base_table(props: &BaseTableProps) -> Html {
    let thead = {
        let items = props.damaging_items.clone();
        let runes = props.damaging_runes.clone();
        let champion_id = props.champion_id.clone();
        use_memo((items, runes, champion_id), move |_| {
            html! {
                <thead>
                    <tr>
                        <th class={classes!("h-10")}></th>
                        <ImageCell instance={Instances::Abilities(props.champion_id)} />
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
        <table>
            {(*thead).clone()}
            <tbody>{props.damages.clone()}</tbody>
        </table>
    }
}
