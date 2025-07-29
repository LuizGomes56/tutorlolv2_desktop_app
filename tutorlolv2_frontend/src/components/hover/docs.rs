use crate::color;
use yew::{AttrValue, Html, classes, html, virtual_dom::VNode};

pub fn hover_docs(formula: AttrValue, single: bool) -> Html {
    html! {
        <div class={
            if single {
                classes!(
                    "opacity-0", "invisible", "pointer-events-none",
                    "group-hover:opacity-100", "group-hover:visible",
                    "group-hover:pointer-events-auto",
                    "transition-[visibility,opacity]",
                    "duration-200", "group-hover:delay-1000",
                    "flex", "flex-col", "fixed", "max-w-md",
                    "max-h-96", "overflow-auto", "p-2", "leading-6",
                    "text-base", "z-50", "hover-docs",
                    "translate-x-[calc(50%-16px)]",
                    "translate-y-[calc(50%+20px)]",
                    "border", color!(border-800), color!(bg-900),
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
