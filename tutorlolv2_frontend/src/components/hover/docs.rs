use crate::color;
use yew::{AttrValue, Html, classes, html, virtual_dom::VNode};

pub fn hover_docs(formula: AttrValue) -> Html {
    html! {
        <div class={
            classes!(
                "flex", "flex-col", "absolute", "max-w-md",
                "max-h-96", "overflow-auto", "p-2", "leading-6",
                "text-base", "z-50", "hover-docs",
                "translate-x-[calc(50%-16px)]",
                "translate-y-[calc(50%+16px)]",
                "border", color!(border-800), color!(bg-900),
            )
        }>
            {
                html! {
                    <code class={classes!(
                        "text-[#D4D4D4]", "font-normal",
                        "text-left", "text-wrap"
                    )}>
                        { VNode::from_html_unchecked(formula) }
                    </code>
                }
            }
        </div>
    }
}
