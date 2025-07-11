use super::base::{BasicStats, Damages, DragonMultipliers, Stats};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

#[derive(Deserialize)]
pub struct ReqCurrentPlayer {
    pub damaging_abilities: BTreeSet<String>,
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

#[derive(Deserialize, PartialEq)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: usize,
}

#[derive(Deserialize)]
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

#[derive(Deserialize, PartialEq)]
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

#[derive(Deserialize)]
pub struct ReqRealtime {
    pub current_player: ReqCurrentPlayer,
    pub enemies: FxHashMap<String, ReqEnemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub scoreboard: Scoreboard,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}

#[derive(PartialEq)]
pub struct CurrentPlayer {
    pub damaging_abilities: Rc<BTreeSet<String>>,
    pub damaging_items: Rc<BTreeSet<usize>>,
    pub damaging_runes: Rc<BTreeSet<usize>>,
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

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: BTreeMap<String, Enemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<usize>,
    pub scoreboard: Scoreboard,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
