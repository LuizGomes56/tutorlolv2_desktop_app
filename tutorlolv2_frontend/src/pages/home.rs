use crate::{
    color,
    components::{formulas::formula_sidebar::FormulaSidebar, sidebar::Sidebar},
    external::highliter,
};
use yew::{Html, classes, function_component, html, virtual_dom::VNode};

#[function_component(Home)]
pub fn home() -> Html {
    let paragraph = |text: &str| -> Html {
        html! {
            <p class={classes!("text-zinc-200")}>{text}</p>
        }
    };

    html! {
        <div class={classes!(
            "flex", "w-full"
        )}>
            <Sidebar />
            <div class={classes!(
                "flex", "flex-1", "overflow-y-auto",
                color!(bg-900)
            )}>
                <FormulaSidebar />
                <div class={classes!(
                    "flex", "flex-col", "gap-4", "p-6", "flex-1",
                    "max-w-3xl", "overflow-y-auto"
                )}>
                    <h1 class={classes!(
                        "font-semibold", "text-2xl", "text-white"
                    )}>
                        {"Automated damage evaluation"}
                    </h1>
                    {paragraph(
                        "The statistics and damage formulas for items, champions,
                        and runes are evaluated at compile-time and automatically
                        updated with each patch. This approach significantly boosts 
                        the application's performance but also complicates the process
                        of adding new features and detecting incorrect formulas. 
                        This section aims to provide transparency by sharing the data 
                        currently used by the application, allowing you to review it
                        and report any potential errors."
                    )}
                    {paragraph(
                        "If you're a developer, you
                        can review the data, submit a pull request on GitHub, or reach 
                        out to me on Discord, and I'll assess whether I can address the 
                        reported issue. If you prefer to handle it yourself, the documentation 
                        for the internal functions is available for you to verify the 
                        implementation's accuracy."
                    )}
                    <code class={classes!("text-[#D4D4D4]")}>
                        {VNode::from_html_unchecked(highliter::highlight().into())}
                    </code>
                </div>
            </div>
        </div>
    }
}
