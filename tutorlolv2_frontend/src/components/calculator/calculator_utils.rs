use crate::models::{
    base::{AbilityLevels, BasicStats, Stats},
    calculator::InputGame,
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
    ($name:ident, $( $stat:ident ),*) => {
        paste! {
            pub enum [<Change $name Action>] {
                Replace($name),
                $(
                    [<Set $stat:camel>](f32),
                )*
            }

            fn [<change_ $name:snake>](stats: &mut $name, action: [<Change $name Action>]) {
                match action {
                    [<Change $name Action>]::Replace(value) => {
                        *stats = value;
                    }
                    $(
                        [<Change $name Action>]::[<Set $stat:camel>](value) => {
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
                    [<Set $ability:upper>](u8),
                )*
            }

            fn change_ability_levels(ability_levels: &mut AbilityLevels, action: $name) {
                match action {
                    $(
                        $name::[<Set $ability:upper>](value) => {
                            ability_levels.$ability = value;
                        }
                    )*
                }
            }
        }
    };
}

ability_level_reducer!(ChangeAbilityLevelsAction, q, w, e, r);

pub enum InputGameAction {
    SetCurrentPlayerChampionId(ChampionId),
    SetCurrentPlayerLevel(u8),
    SetCurrentPlayerInferStats(bool),
    SetCurrentPlayerStacks(u32),
    SetCurrentPlayerStats(ChangeStatsAction),
    SetCurrentPlayerAttackForm(bool),
    InsertCurrentPlayerItem(ItemId),
    RemoveCurrentPlayerItem(usize),
    ClearCurrentPlayerItems,
    InsertCurrentPlayerRune(RuneId),
    RemoveCurrentPlayerRune(usize),
    ClearCurrentPlayerRunes,
    SetAbilityLevels(ChangeAbilityLevelsAction),
    SetEnemyPlayerChampionId(usize, ChampionId),
    SetEnemyPlayerStats(usize, ChangeBasicStatsAction),
    SetEnemyPlayerInferStats(usize, bool),
    SetEnemyPlayerAttackForm(usize, bool),
    InsertEnemyPlayerItem(usize, ItemId),
    RemoveEnemyPlayerItem(usize, usize),
    ClearEnemyPlayerItems(usize),
    SetEnemyPlayerLevel(usize, u8),
    SetAllyFireDragons(u8),
    SetAllyEarthDragons(u8),
    SetEnemyEarthDragons(u8),
}

impl Reducible for InputGame {
    type Action = InputGameAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            InputGameAction::SetCurrentPlayerLevel(value) => {
                new_state.active_player.level = value;
            }
            InputGameAction::InsertCurrentPlayerItem(value) => {
                new_state.active_player.items.push(value);
            }
            InputGameAction::RemoveCurrentPlayerItem(item) => {
                new_state.active_player.items.remove(item);
            }
            InputGameAction::ClearCurrentPlayerItems => {
                new_state.active_player.items.clear();
            }
            InputGameAction::InsertCurrentPlayerRune(value) => {
                new_state.active_player.runes.push(value);
            }
            InputGameAction::RemoveCurrentPlayerRune(rune) => {
                new_state.active_player.runes.remove(rune);
            }
            InputGameAction::ClearCurrentPlayerRunes => {
                new_state.active_player.runes.clear();
            }
            InputGameAction::SetCurrentPlayerStacks(value) => {
                new_state.active_player.stacks = value;
            }
            InputGameAction::SetCurrentPlayerChampionId(value) => {
                new_state.active_player.champion_id = value;
            }
            InputGameAction::SetCurrentPlayerInferStats(value) => {
                new_state.active_player.infer_stats = value;
            }
            InputGameAction::SetCurrentPlayerStats(action) => {
                change_stats(&mut new_state.active_player.champion_stats, action);
            }
            InputGameAction::SetCurrentPlayerAttackForm(value) => {
                // new_state
                //     .stack_exceptions
                //     .insert(u32::MAX - 1, u32::from(value));
            }
            InputGameAction::SetEnemyPlayerAttackForm(index, value) => {
                // new_state
                //     .stack_exceptions
                //     .insert(u32::MAX - 1 - index as u32, u32::from(value));
            }
            InputGameAction::SetEnemyPlayerStats(index, action) => {
                if let Some(enemy) = new_state.enemy_players.get_mut(index) {
                    change_basic_stats(&mut enemy.stats, action);
                }
            }
            InputGameAction::SetEnemyPlayerInferStats(index, value) => {
                if let Some(enemy) = new_state.enemy_players.get_mut(index) {
                    enemy.infer_stats = value;
                }
            }
            InputGameAction::InsertEnemyPlayerItem(index, value) => {
                if let Some(enemy) = new_state.enemy_players.get_mut(index) {
                    enemy.items.push(value);
                }
            }
            InputGameAction::RemoveEnemyPlayerItem(index, item) => {
                if let Some(enemy) = new_state.enemy_players.get_mut(index) {
                    enemy.items.remove(item);
                }
            }
            InputGameAction::ClearEnemyPlayerItems(index) => {
                if let Some(enemy) = new_state.enemy_players.get_mut(index) {
                    enemy.items.clear();
                }
            }
            InputGameAction::SetEnemyPlayerLevel(index, value) => {
                if let Some(enemy) = new_state.enemy_players.get_mut(index) {
                    enemy.level = value;
                }
            }
            InputGameAction::SetEnemyPlayerChampionId(index, value) => {
                if let Some(enemy) = new_state.enemy_players.get_mut(index) {
                    enemy.champion_id = value;
                }
            }
            InputGameAction::SetAbilityLevels(action) => {
                change_ability_levels(&mut new_state.active_player.abilities, action);
            }
            InputGameAction::SetAllyFireDragons(value) => {
                new_state.ally_fire_dragons = value;
            }
            InputGameAction::SetAllyEarthDragons(value) => {
                new_state.ally_earth_dragons = value;
            }
            InputGameAction::SetEnemyEarthDragons(value) => {
                new_state.enemy_earth_dragons = value;
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
    Remove(usize),
}

#[derive(Clone, PartialEq, Default)]
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
                new_state.remove(index);
            }
        }
        Rc::new(new_state)
    }
}
