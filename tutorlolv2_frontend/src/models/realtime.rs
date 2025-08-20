use super::base::{AdaptativeType, Attacks, BasicStats, DamageLike, Stats};
use bincode::Decode;
use generated_code::{AbilityLike, ChampionId, ItemId, RuneId};
use std::collections::BTreeMap;

#[derive(Debug, Decode)]
pub struct GameInformation {
    pub game_time: f32,
    pub map_number: u8,
}

#[derive(Debug, Decode)]
pub struct Scoreboard {
    pub assists: u16,
    pub creep_score: u16,
    pub deaths: u16,
    pub kills: u16,
    pub riot_id: Box<str>,
    pub champion_id: ChampionId,
    pub position: Position,
}

#[derive(Decode, Debug)]
pub struct CurrentPlayer {
    pub damaging_items: Vec<ItemId>,
    pub damaging_runes: Vec<RuneId>,
    pub riot_id: Box<str>,
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

#[derive(Debug, Decode)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: DamageLike<AbilityLike>,
    pub items: DamageLike<ItemId>,
    pub runes: DamageLike<RuneId>,
    pub compared_items: Vec<(ItemId, SimulatedDamages)>,
}

#[derive(Debug, Decode)]
pub struct DragonMultipliers {
    pub earth: f32,
    pub fire: f32,
    pub chemtech: f32,
}

#[derive(Debug, Decode)]
pub struct SimulatedDamages {
    pub abilities: DamageLike<AbilityLike>,
    pub items: DamageLike<ItemId>,
    pub runes: DamageLike<RuneId>,
}

#[derive(Decode, Debug)]
pub struct Enemy {
    pub riot_id: Box<str>,
    pub team: Team,
    pub level: u8,
    pub position: Position,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f32,
    pub real_magic_resist: f32,
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
