use super::base::{BasicStats, Damages, DragonMultipliers, Stats};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: u8,
}

#[derive(Debug, Deserialize)]
pub struct Scoreboard {
    pub assists: u16,
    pub creep_score: u16,
    pub deaths: u16,
    pub kills: u16,
    pub riot_id: String,
    pub champion_id: String,
    pub champion_name: String,
    pub position: String,
}

#[derive(Deserialize, Debug)]
pub struct CurrentPlayer {
    pub damaging_items: Vec<u32>,
    pub damaging_runes: Vec<u32>,
    pub riot_id: String,
    pub level: u8,
    pub team: String,
    pub position: String,
    pub champion_name: String,
    pub champion_id: String,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Deserialize, Debug)]
pub struct Enemy {
    pub champion_name: String,
    pub riot_id: String,
    pub team: String,
    pub level: u8,
    pub position: String,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Deserialize, Debug)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: BTreeMap<String, Enemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<u32>,
    pub scoreboard: Scoreboard,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
