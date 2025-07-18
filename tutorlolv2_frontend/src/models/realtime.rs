use super::base::{BasicStats, Damages, DragonMultipliers, Stats};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

#[derive(Debug, Deserialize)]
pub struct ReqCurrentPlayer {
    pub damaging_abilities: Vec<String>,
    pub damaging_items: Vec<usize>,
    pub damaging_runes: Vec<usize>,
    pub riot_id: String,
    pub level: usize,
    pub team: String,
    pub position: String,
    pub champion_name: String,
    pub champion_id: String,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Debug, Deserialize)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(Debug, Deserialize)]
pub struct ReqEnemy {
    pub champion_name: String,
    pub riot_id: String,
    pub team: String,
    pub level: usize,
    pub position: String,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Debug, Deserialize)]
pub struct Scoreboard {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub riot_id: String,
    pub champion_id: String,
    pub champion_name: String,
    pub position: String,
}

#[derive(Debug, Deserialize)]
pub struct ReqRealtime {
    pub current_player: ReqCurrentPlayer,
    pub enemies: FxHashMap<String, ReqEnemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub scoreboard: Scoreboard,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}

#[derive(Debug)]
pub struct CurrentPlayer {
    pub damaging_abilities: Rc<BTreeSet<String>>,
    pub damaging_items: BTreeSet<usize>,
    pub damaging_runes: BTreeSet<usize>,
    pub riot_id: String,
    pub level: usize,
    pub team: String,
    pub position: String,
    pub champion_name: String,
    pub champion_id: String,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Debug)]
pub struct Enemy {
    pub champion_name: String,
    pub riot_id: String,
    pub team: String,
    pub level: usize,
    pub position: String,
    pub damages: Rc<Damages>,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Debug)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: BTreeMap<String, Enemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub scoreboard: Scoreboard,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
