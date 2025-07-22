use std::rc::Rc;
use yew::{
    Children, ContextProvider, Html, Properties, Reducible, UseReducerHandle, function_component,
    html, use_effect_with, use_reducer,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum HoverDocs {
    #[default]
    Full,
    Partial,
    None,
}

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

    // {
    //     let settings = settings.clone();
    //     use_effect_with(settings, move |settings| {
    //         web_sys::console::log_1(&format!("{:?}", (*settings).docs).into());
    //     })
    // }

    html! {
        <ContextProvider<SettingsContext> context={settings.clone()}>
            { for props.children.iter() }
        </ContextProvider<SettingsContext>>
    }
}
