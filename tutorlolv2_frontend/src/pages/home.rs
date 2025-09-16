use crate::components::image::{Image, ImageType};
use tutorlolv2_imports::ItemId;
use yew::{Html, classes, function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!(
            "p-6", "flex-1",
        )}>
            <div class={classes!("max-w-3xl", "flex", "flex-col", "gap-4")}>
                <h1 class={classes!(
                    "font-semibold", "text-2xl", "text-white"
                )}>
                    {"Automated damage evaluation"}
                </h1>
            </div>
            <Image class={classes!("w-8", "h-8")} source={ImageType::Item(ItemId::TheGoldenSpatula)} />
        </div>
    }
}
