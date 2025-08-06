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
    macro_rules! table_head {
        ($name:ident, $extension:literal) => {
            paste::paste! {
                html! {
                    <th
                        title={stringify!([<$name:snake:camel>])}
                        class={classes!("min-w-10", "h-10", "justify-items-center")}
                    >
                        <Image
                            class={classes!("w-8", "h-8")}
                            source={ImageType::Other(
                                AttrValue::Static(
                                    concat!(
                                        url!(@inner),
                                        "/img/other/",
                                        stringify!($name),
                                        ".",
                                        $extension
                                    )
                                )
                            ) }
                        />
                    </th>
                }
            }
        };
    }

    let header_memo = use_memo((), move |_| {
        html! {
            <thead>
                <tr>
                    {table_head!(tower, "webp")}
                    {table_head!(dragon, "avif")}
                    {table_head!(baron, "avif")}
                    {table_head!(atakhan, "avif")}
                    {table_head!(voidgrubs, "avif")}
                    {table_head!(melee_minion, "avif")}
                    {table_head!(ranged_minion, "avif")}
                    {table_head!(super_minion, "avif")}
                    {table_head!(red_buff, "avif")}
                    {table_head!(gromp, "avif")}
                    {table_head!(krug, "avif")}
                    {table_head!(wolves, "avif")}
                    {table_head!(raptor, "avif")}
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
