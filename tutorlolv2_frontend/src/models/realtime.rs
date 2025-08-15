use super::base::{AdaptativeType, BasicStats, Damages, DragonMultipliers, Stats};
use bincode::Decode;
use generated_code::{ChampionId, ItemId, RuneId};
use std::collections::BTreeMap;

#[derive(Debug, Decode)]
pub struct GameInformation {
    pub game_time: f64,
    pub map_number: u8,
}

#[derive(Debug, Decode)]
pub struct Scoreboard {
    pub assists: u16,
    pub creep_score: u16,
    pub deaths: u16,
    pub kills: u16,
    pub riot_id: String,
    pub champion_id: ChampionId,
    pub position: Position,
}

#[derive(Decode, Debug)]
pub struct CurrentPlayer {
    pub damaging_items: Vec<ItemId>,
    pub damaging_runes: Vec<RuneId>,
    pub riot_id: String,
    pub level: u8,
    pub team: Team,
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Debug, Copy, Clone, Decode)]
pub enum Team {
    Blue,
    Red,
}

#[derive(Debug, Copy, Clone, Decode)]
pub enum Position {
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

#[derive(Decode, Debug)]
pub struct Enemy {
    pub riot_id: String,
    pub team: Team,
    pub level: u8,
    pub position: Position,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Decode, Debug)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: BTreeMap<ChampionId, Enemy>,
    pub game_information: GameInformation,
    pub recommended_items: Vec<ItemId>,
    pub scoreboard: Scoreboard,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
