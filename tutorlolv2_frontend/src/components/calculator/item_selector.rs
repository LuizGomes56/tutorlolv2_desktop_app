use crate::{models::calculator::InputGame, url, utils::to_pascal_case};
use std::{cell::RefCell, rc::Rc};
use yew::{Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html};

#[derive(PartialEq, Properties)]
pub struct ItemSelectorProps {
    pub data: Rc<RefCell<InputGame>>,
}

#[function_component(ItemSelector)]
pub fn item_selector(props: &ItemSelectorProps) -> Html {
    html! {}
}
