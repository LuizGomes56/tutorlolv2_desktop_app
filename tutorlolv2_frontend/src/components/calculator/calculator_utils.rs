use crate::models::{
    base::{AbilityLevels, BasicStats, Stats},
    calculator::{InputCurrentPlayer, InputDragons, InputEnemyPlayer},
};
use generated_code::{AbilityLike, ChampionId, ItemId, RuneId};
use paste::paste;
use std::{rc::Rc, u32};
use yew::Reducible;

#[derive(Clone, Copy, PartialEq)]
pub enum StaticIterator {
    Runes,
    Items,
    Champions,
}

macro_rules! stats_reducer {
    ($name:ident, $($stat:ident),*) => {
        paste! {
            pub enum [<Change $name Action>] {
                Replace(*const $name),
                $(
                    [<$stat:camel>](f32),
                )*
            }

            #[inline]
            fn [<change_ $name:snake>](stats: &mut $name, action: [<Change $name Action>]) {
                match action {
                    [<Change $name Action>]::Replace(value) => {
                        unsafe {
                            *stats = *value;
                        }
                    },
                    $(
                        [<Change $name Action>]::[<$stat:camel>](value) => {
                            stats.$stat = value;
                        }
                    )*
                }
            }
        }
    };
}

stats_reducer!(
    Stats,
    ability_power,
    armor,
    armor_penetration_flat,
    armor_penetration_percent,
    attack_damage,
    attack_speed,
    crit_chance,
    crit_damage,
    current_health,
    magic_penetration_flat,
    magic_penetration_percent,
    magic_resist,
    max_health,
    max_mana,
    current_mana
);

stats_reducer!(BasicStats, armor, health, attack_damage, magic_resist, mana);

macro_rules! ability_level_reducer {
    ($name:ident, $( $ability:ident ),*) => {
        paste! {
            pub enum $name {
                $(
                    [<$ability:upper>](u8),
                )*
            }

            fn change_ability_levels(ability_levels: &mut AbilityLevels, action: $name) {
                match action {
                    $(
                        $name::[<$ability:upper>](value) => {
                            ability_levels.$ability = value;
                        }
                    )*
                }
            }
        }
    };
}

ability_level_reducer!(ChangeAbilityLevelsAction, q, w, e, r);

pub enum CurrentPlayerAction {
    ChampionId(ChampionId),
    Level(u8),
    InferStats(bool),
    Stacks(u32),
    Stats(ChangeStatsAction),
    AttackForm(bool),
    InsertItem(ItemId),
    RemoveItem(usize),
    ClearItems,
    InsertRune(RuneId),
    RemoveRune(usize),
    ClearRunes,
    AbilityLevels(ChangeAbilityLevelsAction),
}

pub enum DragonAction {
    AllyFireDragons(u8),
    AllyEarthDragons(u8),
    EnemyEarthDragons(u8),
}

pub enum InputEnemyAction {
    ChampionId(ChampionId),
    Stats(ChangeBasicStatsAction),
    InferStats(bool),
    AttackForm(bool),
    InsertItem(ItemId),
    RemoveItem(usize),
    ClearItems,
    Level(u8),
}

impl Reducible for InputCurrentPlayer {
    type Action = CurrentPlayerAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            Self::Action::ChampionId(v) => {
                new_state.champion_id = v;
            }
            Self::Action::Level(v) => {
                new_state.level = v;
            }
            Self::Action::InferStats(v) => {
                new_state.infer_stats = v;
            }
            Self::Action::Stats(v) => {
                change_stats(&mut new_state.stats, v);
            }
            Self::Action::AbilityLevels(v) => {
                change_ability_levels(&mut new_state.abilities, v);
            }
            Self::Action::AttackForm(v) => {}
            Self::Action::InsertItem(v) => {
                new_state.items.push(v);
            }
            Self::Action::RemoveItem(v) => {
                new_state.items.swap_remove(v);
            }
            Self::Action::ClearItems => {
                new_state.items.clear();
            }
            Self::Action::InsertRune(v) => {
                new_state.runes.push(v);
            }
            Self::Action::RemoveRune(v) => {
                new_state.runes.remove(v);
            }
            Self::Action::ClearRunes => {
                new_state.runes.clear();
            }
            Self::Action::Stacks(v) => {
                new_state.stacks = v;
            }
        }
        Rc::new(new_state)
    }
}

#[derive(bincode::Encode, PartialEq, Clone, Debug)]
#[repr(transparent)]
pub struct InputEnemies(Vec<Rc<InputEnemyPlayer>>);

impl InputEnemies {
    pub fn new() -> Self {
        Self(vec![Rc::new(InputEnemyPlayer::new())])
    }

    pub fn as_slice(&self) -> &[Rc<InputEnemyPlayer>] {
        &self.0
    }
}

pub enum EnemiesAction {
    Push,
    Remove(usize),
    Edit(usize, InputEnemyAction),
}

impl Reducible for InputEnemies {
    type Action = EnemiesAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            Self::Action::Push => {
                new_state.0.push(Rc::new(InputEnemyPlayer::new()));
            }
            Self::Action::Remove(v) => {
                new_state.0.swap_remove(v);
            }
            Self::Action::Edit(v, enemy_action) => {
                new_state.0[v] = new_state.0[v].clone().reduce(enemy_action);
            }
        }
        Rc::new(new_state)
    }
}

impl Reducible for InputEnemyPlayer {
    type Action = InputEnemyAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            Self::Action::ChampionId(v) => {
                new_state.champion_id = v;
            }
            Self::Action::Stats(v) => {
                change_basic_stats(&mut new_state.stats, v);
            }
            Self::Action::InferStats(v) => {
                new_state.infer_stats = v;
            }
            Self::Action::AttackForm(v) => {}
            Self::Action::InsertItem(v) => {
                new_state.items.push(v);
            }
            Self::Action::RemoveItem(v) => {
                new_state.items.swap_remove(v as usize);
            }
            Self::Action::ClearItems => {
                new_state.items.clear();
            }
            Self::Action::Level(v) => {
                new_state.level = v;
            }
        }
        Rc::new(new_state)
    }
}

impl Reducible for InputDragons {
    type Action = DragonAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            Self::Action::AllyFireDragons(v) => {
                new_state.ally_fire_dragons = v;
            }
            Self::Action::AllyEarthDragons(v) => {
                new_state.ally_earth_dragons = v;
            }
            Self::Action::EnemyEarthDragons(v) => {
                new_state.enemy_earth_dragons = v;
            }
        }
        Rc::new(new_state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StackValue {
    Ability(AbilityLike),
    Item(ItemId),
    Rune(RuneId),
    BasicAttack,
    CriticalStrike,
    Onhit,
    Ignite,
}

pub enum StackAction {
    Push(StackValue),
    Remove(u16),
}

#[derive(Clone, PartialEq, Default)]
#[repr(transparent)]
pub struct Stack(Vec<StackValue>);

impl Stack {
    pub fn get_owned(&self) -> Vec<StackValue> {
        self.0.clone()
    }
    pub fn get_ref(&self) -> &[StackValue] {
        &self.0
    }
    pub fn push(&mut self, value: StackValue) {
        self.0.push(value);
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

impl Reducible for Stack {
    type Action = StackAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            StackAction::Push(value) => {
                new_state.push(value);
            }
            StackAction::Remove(index) => {
                new_state.remove(index as usize);
            }
        }
        Rc::new(new_state)
    }
}
