use crate::{
    color,
    components::Dropdown,
    context::{GlobalContextActions, HoverDocs, SettingsContext},
};
use yew::{Html, classes, function_component, html, use_callback, use_context};

#[function_component(Settings)]
pub fn settings() -> Html {
    let context = use_context::<SettingsContext>().expect("SettingsContext não encontrado");
    let hover_docs_callback = {
        let context = context.clone();
        use_callback((), move |v, _| {
            context.dispatch(GlobalContextActions::SetHoverDocs(HoverDocs::from(v)));
        })
    };

    html! {
        <div class={classes!(
            "flex", "gap-6", "flex-col",
            "p-6", "max-w-3xl", "mb-48"
        )}>
            <h1 class={classes!(
                "text-2xl", "sm:text-3xl", "font-medium"
            )}>
                { "Application Settings" }
            </h1>
            <div class={classes!(
                "w-full", "border-b",
                color!(border-700),
                "pb-5", "flex", "flex-col", "gap-4"
            )}>
                <div class={classes!(
                    "flex", "flex-col", "gap-4"
                )}>
                    <h2>{ "Object-hover behavior" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p>{ "Choose how hover interactions reveal object information:" }</p>
                        <ol class="list-disc list-inside mt-2 space-y-1">
                            <li>
                                <strong>{ "None" }</strong>
                                { ": Disables nearly all hover effects." }
                            </li>
                            <li>
                                <strong>{ "Partial" }</strong>
                                { ": Displays only names, stats, and limited item information. Source code is hidden." }
                            </li>
                            <li>
                                <strong>{ "Full" }</strong>
                                { ": Reveals source code after 1 second, along with all available details." }
                            </li>
                        </ol>
                    </div>
                </div>
                <Dropdown<3>
                    current_index={context.docs as usize}
                    callback={hover_docs_callback.clone()}
                    name={"hover_attr"}
                    iterator={[
                        "None",
                        "Partial",
                        "Full",
                    ]}
                />
            </div>
        </div>
    }
}
