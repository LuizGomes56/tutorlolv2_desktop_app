use crate::components::image::{Image, ImageType};
use yew::{Html, classes, function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!(
            "p-6", "flex-1", "h-screen", "overflow-y-auto"
        )}>
            <div class={classes!("max-w-3xl", "flex", "flex-col", "gap-4")}>
                <h1 class={classes!(
                    "font-semibold", "text-2xl", "text-white"
                )}>
                    {"Automated damage evaluation"}
                </h1>
            </div>
            <Image size={28} source={ImageType::Items(224403)} />
        </div>
    }
}
