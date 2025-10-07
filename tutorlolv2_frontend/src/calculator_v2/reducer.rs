use crate::model_v2::*;
use bincode::Encode;
use std::rc::Rc;
use tutorlolv2_imports::{ChampionId, ItemId, RuneId};
use yew::Reducible;

pub enum InputDataAction<T> {
    ChampionId(ChampionId),
    InsertItem(ItemId),
    RemoveItem(usize),
    InferStats(bool),
    IsMegaGnar(bool),
    Stats(*const T),
    Stacks(u32),
    Level(u8),
}

pub enum InputActivePlayerAction {
    InsertRune(RuneId),
    RemoveRune(usize),
    AbilityLevels(AbilityLevels),
    Data(InputDataAction<StatsF32>),
}

impl<T: Copy + Default> OwnedMinData<T> {
    pub fn apply_reducer(&mut self, action: InputDataAction<T>) {
        match action {
            InputDataAction::ChampionId(v) => *self = Self::new(v, T::default()),
            InputDataAction::InsertItem(v) => self.items.push(v),
            InputDataAction::RemoveItem(v) => {
                self.items.swap_remove(v);
            }
            InputDataAction::InferStats(v) => self.infer_stats = v,
            InputDataAction::IsMegaGnar(v) => self.is_mega_gnar = v,
            InputDataAction::Stats(v) => self.stats = unsafe { *v },
            InputDataAction::Stacks(v) => self.stacks = v,
            InputDataAction::Level(v) => self.level = v,
        }
    }
}

impl<T: Clone + Copy + Default> Reducible for OwnedMinData<T> {
    type Action = InputDataAction<T>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        new_state.apply_reducer(action);
        Rc::new(new_state)
    }
}

impl Reducible for OwnedActivePlayer {
    type Action = InputActivePlayerAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            Self::Action::InsertRune(v) => new_state.runes.push(v),
            Self::Action::RemoveRune(v) => {
                new_state.runes.swap_remove(v);
            }
            Self::Action::AbilityLevels(v) => new_state.abilities = v,
            Self::Action::Data(v) => new_state.data.apply_reducer(v),
        };

        Rc::new(new_state)
    }
}

#[derive(Encode, Clone, PartialEq)]
#[repr(transparent)]
pub struct InputEnemies<T>(Vec<Rc<OwnedMinData<T>>>);

impl<T: Default> InputEnemies<T> {
    pub fn new() -> Self {
        Self(vec![Rc::new(OwnedMinData::<T>::default())])
    }
}

pub enum EnemyAction<T> {
    Push,
    Remove(usize),
    Edit(usize, InputDataAction<T>),
}

impl<T: Clone + Copy + Default> Reducible for InputEnemies<T> {
    type Action = EnemyAction<T>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            EnemyAction::Push => {
                new_state.0.push(Rc::new(OwnedMinData::<T>::default()));
            }
            EnemyAction::Remove(v) => {
                new_state.0.swap_remove(v);
            }
            EnemyAction::Edit(v, enemy_action) => {
                new_state.0[v] = new_state.0[v].clone().reduce(enemy_action);
            }
        }
        Rc::new(new_state)
    }
}

impl<T> AsRef<[Rc<OwnedMinData<T>>]> for InputEnemies<T> {
    fn as_ref(&self) -> &[Rc<OwnedMinData<T>>] {
        &self.0
    }
}
