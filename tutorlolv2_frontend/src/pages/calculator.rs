use crate::{
    components::calculator::*,
    external::api::{decode_bytes, send_bytes},
    models::calculator::{InputGame, OutputGame},
    url,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use yew::{
    Callback, Html, UseStateHandle, classes, function_component, html, platform::spawn_local,
    use_effect_with, use_state,
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
    let input_game = use_state(|| (Rc::new(RefCell::new(InputGame::default())), 0));
    let output_game = use_state(|| None::<Rc<OutputGame>>);

    {
        let input_game = input_game.clone();
        let output_game = output_game.clone();
        use_effect_with(input_game.clone(), move |_| {
            web_sys::console::log_1(&format!("{:#?}", input_game.get()).into());

            spawn_local(async move {
                let response = send_bytes(url!("/api/games/calculator"), &*input_game.get()).await;

                if let Ok(res) = response {
                    match decode_bytes::<OutputGame>(res).await {
                        Ok(data) => {
                            output_game.set(Some(Rc::new(data)));
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("{:#?}", e).into());
                        }
                    }
                }
            });
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
                    src={url!("/img/centered/{}_0.avif", input_game.get().active_player.champion_id)}
                    alt={""}
                />
                <div class={classes!(
                    "grid", "grid-cols-2", "gap-x-2",
                )}>
                    <AbilitySelector input_game={input_game.clone()} />
                    <ExceptionSelector input_game={input_game.clone()} />
                </div>
                <ItemSelector input_game={input_game.clone()} />
                <RuneSelector input_game={input_game.clone()} />
                <StatsSelector input_game={input_game.clone()} />
            </div>
            <div>
                {
                    if let Some(output_game) = &*output_game {
                        html! {
                            <div class={classes!(
                                "text-white", "text-xl"
                            )}>
                                {output_game.current_player.current_stats.armor}
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
