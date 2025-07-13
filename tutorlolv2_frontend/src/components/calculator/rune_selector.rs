use crate::{
    models::calculator::InputGame, pages::calculator::CalculatorState, url, utils::to_pascal_case,
};
use std::{cell::RefCell, rc::Rc};
use yew::{Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct RuneSelectorProps {
    pub data: CalculatorState,
}

#[function_component(RuneSelector)]
pub fn rune_selector(props: &RuneSelectorProps) -> Html {
    html! {}
}
