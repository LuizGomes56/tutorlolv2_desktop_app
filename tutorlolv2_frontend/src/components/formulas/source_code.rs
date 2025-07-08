use crate::{STATIC_FORMULAS, external::highliter};
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use yew::{Html, Properties, classes, function_component, html, use_memo, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct SourceCodeProps {
    pub champion_id: String,
}

thread_local! {
    static FORMULA_CODE_CACHE: RefCell<FxHashMap<String, Html>> =
        RefCell::new(FxHashMap::default());
}

#[function_component(SourceCode)]
pub fn source_code(props: &SourceCodeProps) -> Html {
    let code = STATIC_FORMULAS
        .get()
        .and_then(|map| map.get(&props.champion_id))
        .map(String::as_str)
        .unwrap_or("Failed to fetch code");

    let highlighted_code = use_memo(props.champion_id.clone(), |champion_id| {
        if let Some(cached) =
            FORMULA_CODE_CACHE.with(|cache| cache.borrow().get(champion_id).cloned())
        {
            return cached;
        }

        let raw_html = highliter::highlight(&code);
        let html_result = html! {
            <code class={classes!("text-[#D4D4D4]")}>
                { VNode::from_html_unchecked(raw_html.clone().into()) }
            </code>
        };

        FORMULA_CODE_CACHE.with(|cache| {
            cache
                .borrow_mut()
                .insert(champion_id.clone(), html_result.clone());
        });

        html_result
    });

    html! {(*highlighted_code).clone()}
}
