use crate::{impl_unsafe_cast, utils::ToStaticStr};
use std::rc::Rc;
use yew::{
    Children, ContextProvider, Html, Properties, Reducible, UseReducerHandle, function_component,
    html, use_reducer,
};

#[derive(Copy, Clone, PartialEq, Default)]
#[repr(u8)]
pub enum HoverDocs {
    None,
    Partial,
    #[default]
    Full,
}

impl ToStaticStr for HoverDocs {
    #[inline]
    fn as_static_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Partial => "Partial",
            Self::Full => "Full",
        }
    }
}

impl HoverDocs {
    #[inline]
    pub fn to_array() -> [&'static str; 3] {
        [
            Self::None.as_static_str(),
            Self::Partial.as_static_str(),
            Self::Full.as_static_str(),
        ]
    }
}

impl_unsafe_cast!(HoverDocs, u8);

#[derive(PartialEq, Default, Copy, Clone)]
pub struct GlobalContext {
    pub docs: HoverDocs,
}

pub enum GlobalContextActions {
    SetHoverDocs(HoverDocs),
}

impl Reducible for GlobalContext {
    type Action = GlobalContextActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = *self;
        match action {
            GlobalContextActions::SetHoverDocs(docs) => {
                new_state.docs = docs;
            }
        }
        Rc::new(new_state)
    }
}

pub type SettingsContext = UseReducerHandle<GlobalContext>;

#[derive(Properties, PartialEq)]
pub struct SettingsProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(SettingsProvider)]
pub fn settings_provider(props: &SettingsProviderProps) -> Html {
    let settings = use_reducer(GlobalContext::default);

    html! {
        <ContextProvider<SettingsContext> context={settings.clone()}>
            { for props.children.iter() }
        </ContextProvider<SettingsContext>>
    }
}
