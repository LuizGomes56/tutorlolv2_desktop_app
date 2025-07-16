use crate::models::base::{AbilityLevels, BasicStats, Stats};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
use yew::UseStateHandle;

#[derive(PartialEq)]
pub struct StackExceptions {}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct OuterVolatileAttrs {
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
}

/// Will be placed inside an Rc<RefCell> and since they don't depend on InputEvent,
/// they can "safely" be mutated without causing a BorrowMut error. However, if the
/// user decides to create a script to trigger a huge amount of events on this list,
/// a BorrowMut error may occur, but in fair use, this is acceptable and performant
#[derive(PartialEq)]
pub struct DangerousAttrs {
    pub current_player_champion_id: String,
    pub current_player_items: Vec<usize>,
    pub current_player_runes: Vec<usize>,
    pub stack_exceptions: StackExceptions,
    pub enemy_champion_names: Vec<String>,
    pub enemy_items: Vec<Vec<usize>>,
}

impl Default for DangerousAttrs {
    fn default() -> Self {
        Self {
            current_player_champion_id: "Neeko".into(),
            current_player_items: Default::default(),
            current_player_runes: Default::default(),
            stack_exceptions: StackExceptions {},
            enemy_champion_names: Default::default(),
            enemy_items: Default::default(),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct CurrentPlayerVolatileAttrs {
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub level: usize,
    pub stacks: usize,
    pub infer_stats: bool,
}

impl Default for CurrentPlayerVolatileAttrs {
    fn default() -> Self {
        Self {
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            level: 15,
            stacks: 0,
            infer_stats: true,
            champion_stats: Default::default(),
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct EnemyPlayerVolatileAttrs {
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

pub trait DangerousMutAttrExt<T> {
    fn force_update(&self);
    fn get(&self) -> Ref<'_, T>;
    fn try_update<F: FnOnce(&mut T)>(&self, f: F) -> Result<(), String>;
}

impl<T> DangerousMutAttrExt<T> for UseStateHandle<(Rc<RefCell<T>>, u64)> {
    fn force_update(&self) {
        let (data, index) = &**self;
        self.set((data.clone(), *index + 1));
    }

    fn get(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    fn try_update<F: FnOnce(&mut T)>(&self, f: F) -> Result<(), String> {
        match self.0.try_borrow_mut() {
            Ok(mut borrowed) => {
                f(&mut *borrowed);
                drop(borrowed);
                self.force_update();
                Ok(())
            }
            Err(_) => {
                let msg = "Unsafe struct update was prevented";
                web_sys::console::log_1(&msg.into());
                Err(msg.to_string())
            }
        }
    }
}
