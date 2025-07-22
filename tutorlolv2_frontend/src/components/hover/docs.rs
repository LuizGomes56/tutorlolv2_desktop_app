use crate::color;
use yew::{AttrValue, Html, classes, html, virtual_dom::VNode};

pub fn hover_docs(formula: AttrValue, single: bool) -> Html {
    html! {
        <div class={
            if single {
                classes!(
                    "hidden", "group-hover:flex", "fixed",
                    "border", color!(bg - 900), "leading-6",
                    "transform", "max-w-md", "p-2",
                    "translate-x-[calc(50%-16px)]",
                    "translate-y-[calc(50%+20px)]",
                    "overflow-auto", "max-h-96", "z-50",
                    "hover-docs", color!(border - 800),
                    "text-base"
                )
            } else {
                classes!("leading-6", "text-base")
            }
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
