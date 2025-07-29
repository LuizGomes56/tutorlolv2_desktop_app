use crate::build_imports::CHAMPION_FORMULAS;
use yew::{AttrValue, Html, Properties, classes, function_component, html, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct SourceCodeProps {
    pub champion_id: &'static str,
}

#[function_component(SourceCode)]
pub fn source_code(props: &SourceCodeProps) -> Html {
    let code = CHAMPION_FORMULAS
        .get(&props.champion_id)
        .unwrap_or(&"Failed to fetch code");

    html! {
        <code class={classes!(
            "text-[#D4D4D4]", "text-left",
            "text-wrap", "break-all"
        )}>
            { VNode::from_html_unchecked(AttrValue::Static(code)) }
        </code>
    }
}
