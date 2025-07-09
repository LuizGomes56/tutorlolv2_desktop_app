use super::base::{AbilityLevels, BasicStats, ComparedItem, Damages, Stats};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct OutputCurrentPlayer {
    pub champion_id: String,
    pub damaging_abilities: BTreeMap<String, String>,
    pub damaging_items: BTreeMap<usize, String>,
    pub damaging_runes: BTreeMap<usize, String>,
    pub level: usize,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Deserialize)]
pub struct OutputEnemy {
    pub champion_name: String,
    pub champion_id: String,
    pub level: usize,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Deserialize)]
pub struct OutputGame {
    pub current_player: OutputCurrentPlayer,
    pub enemies: Vec<OutputEnemy>,
    pub recommended_items: Vec<usize>,
    pub compared_items: BTreeMap<usize, ComparedItem>,
}

#[derive(Serialize)]
pub struct InputActivePlayer {
    pub champion_id: String,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: Vec<usize>,
    pub runes: Vec<usize>,
    pub level: usize,
    pub stacks: usize,
    pub infer_stats: bool,
}

#[derive(Serialize)]
pub struct InputEnemyPlayers {
    pub champion_name: String,
    pub items: Vec<usize>,
    pub level: usize,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Serialize)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: Vec<InputEnemyPlayers>,
    pub ally_earth_dragons: usize,
    pub ally_fire_dragons: usize,
    pub enemy_earth_dragons: usize,
    pub stack_exceptions: FxHashMap<usize, usize>,
}
