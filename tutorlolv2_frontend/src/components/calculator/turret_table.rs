use crate::{
    components::{Image, ImageType},
    url,
};
use yew::{AttrValue, Html, Properties, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct TurretTableProps {
    pub damages: Html,
}

#[function_component(TurretTable)]
pub fn turret_table(props: &TurretTableProps) -> Html {
    html! {
        <table class={classes!("w-fit")}>
            <thead>
                <tr>
                    {
                        (0..6).into_iter().map(|i| {
                            html! {
                                <th class={classes!(
                                    "min-w-10"
                                )}>
                                    <div class={classes!(
                                        "flex", "items-center",
                                        "justify-center", "relative"
                                    )}>
                                        <Image
                                            class={classes!(
                                                "w-8", "h-8"
                                            )}
                                            source={ImageType::Other(
                                                AttrValue::Static(url!("/img/other/tower.webp"))
                                            )}
                                        />
                                        <span class={classes!(
                                            "text-sm", "img-letter"
                                        )}>
                                            {i}
                                        </span>
                                    </div>
                                </th>
                            }
                        })
                        .collect::<Html>()
                    }
                </tr>
            </thead>
            <tbody><tr>{props.damages.clone()}</tr></tbody>
        </table>
    }
}
