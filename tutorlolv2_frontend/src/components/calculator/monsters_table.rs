use crate::{
    components::{Image, ImageType},
    url,
};
use yew::{AttrValue, Html, Properties, classes, function_component, html, use_memo};

#[derive(PartialEq, Properties)]
pub struct MonstersTableProps {
    pub damages: Html,
}

#[function_component(MonstersTable)]
pub fn monsters_table(props: &MonstersTableProps) -> Html {
    let header_memo = use_memo((), move |_| {
        html! {
            <thead>
                <tr>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/other/tower.webp"))) }
                        />
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/other/dragon.avif"))) }
                        />
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/other/baron.avif"))) }
                        />
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/other/melee_minion.avif"))) }
                        />
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/other/ranged_minion.avif"))) }
                        />
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/other/super_minion.avif"))) }
                        />
                    </th>
                    <th class={classes!("min-w-10", "h-10", "justify-items-center")}>
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(AttrValue::Static(url!("/img/other/red_buff.avif"))) }
                        />
                    </th>
                </tr>
            </thead>
        }
    });

    html! {
        <table>
            {(*header_memo).clone()}
            <tbody>
                {props.damages.clone()}
            </tbody>
        </table>
    }
}
