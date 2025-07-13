use crate::{components::calculator::*, models::calculator::InputGame, url};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use yew::{
    Callback, Html, UseStateHandle, classes, function_component, html, use_effect_with, use_state,
};

pub type CalculatorState = UseStateHandle<(Rc<RefCell<InputGame>>, u64)>;

pub trait CalculatorExt {
    fn force_update(&self);
    fn get_mut(&self) -> RefMut<'_, InputGame>;
    fn get(&self) -> Ref<'_, InputGame>;
    fn update<F: FnOnce(&mut InputGame)>(&self, f: F);
}

impl CalculatorExt for UseStateHandle<(Rc<RefCell<InputGame>>, u64)> {
    fn force_update(&self) {
        let (data, index) = &**self;
        self.set((data.clone(), *index + 1));
    }

    fn get_mut(&self) -> RefMut<'_, InputGame> {
        self.0.borrow_mut()
    }

    fn get(&self) -> Ref<'_, InputGame> {
        self.0.borrow()
    }

    fn update<F: FnOnce(&mut InputGame)>(&self, f: F) {
        f(&mut self.get_mut());
        self.force_update();
    }
}

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let data = use_state(|| (Rc::new(RefCell::new(InputGame::default())), 0));

    {
        let data = data.clone();
        use_effect_with(data.clone(), move |_| {
            web_sys::console::log_1(&format!("{:#?}", (*data).0.borrow()).into());
        });
    }

    html! {
        <div class={classes!(
            "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-4", "w-56"
            )}>
                <img
                    loading={"lazy"}
                    class={classes!("w-full", "img-clipped", "h-16")}
                    src={url!("/img/centered/{}_0.avif", (*data).0.borrow().active_player.champion_id)}
                    alt={""}
                />
                <div class={classes!(
                    "grid", "grid-cols-2", "gap-x-2",
                )}>
                    <AbilitySelector data={data.clone()} />
                    <ExceptionSelector data={data.clone()} />
                </div>
                <ItemSelector data={data.clone()} />
                <RuneSelector data={data.clone()} />
                <StatsSelector data={data.clone()} />
            </div>
            <div onclick={
                Callback::from(move |_| {
                    web_sys::console::log_1(&format!("{:#?}", (*data).0.borrow()).into());
                })}>
                {"Display Items"}
            </div>
        </div>
    }
}
