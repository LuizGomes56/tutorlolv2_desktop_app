use crate::color;
use yew::{AttrValue, Html, classes, html, virtual_dom::VNode};

pub fn hover_docs(formula: AttrValue) -> Html {
    html! {
        <div class={classes!(
            "hidden", "group-hover:flex", "fixed",
            "border", color!(bg-900), "leading-6",
            "transform", "max-w-md",
            "translate-x-[calc(50%-16px)]",
            "translate-y-[calc(50%+20px)]",
            "overflow-auto",
            "max-h-96", "hover-docs",
            color!(border-800), "z-50"
        )}>
            {
                html! {
                    <code class={classes!(
                        "text-[#D4D4D4]", "font-normal",
                        "text-left", "p-2", "text-wrap"
                    )}>
                        { VNode::from_html_unchecked(formula) }
                    </code>
                }
            }
        </div>
    }
}
