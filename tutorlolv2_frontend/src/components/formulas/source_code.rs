use crate::STATIC_CHAMPION_FORMULAS;
use yew::{Html, Properties, classes, function_component, html, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct SourceCodeProps {
    pub champion_id: String,
}

#[function_component(SourceCode)]
pub fn source_code(props: &SourceCodeProps) -> Html {
    let code = STATIC_CHAMPION_FORMULAS
        .get()
        .and_then(|map| map.get(&props.champion_id))
        .map(String::as_str)
        .unwrap_or("Failed to fetch code");

    html! {
        <code class={classes!(
            "text-[#D4D4D4]", "text-left",
            "text-wrap", "break-all"
        )}>
            { VNode::from_html_unchecked(code.into()) }
        </code>
    }
}
