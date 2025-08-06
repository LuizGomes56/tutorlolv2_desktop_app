use crate::utils::ComptimeCache;
use yew::{AttrValue, Html, Properties, classes, function_component, html, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct SourceCodeProps {
    pub offset: Option<&'static (usize, usize)>,
}

#[function_component(SourceCode)]
pub fn source_code(props: &SourceCodeProps) -> Html {
    if let Some(offset) = props.offset {
        html! {
            <code class={classes!(
                "text-[#D4D4D4]", "text-left",
                "text-wrap", "break-all"
            )}>
                { VNode::from_html_unchecked(AttrValue::Static(offset.as_str())) }
            </code>
        }
    } else {
        html! {}
    }
}
