use super::base::{AdaptativeType, Attacks, BasicStats, DamageLike, Stats};
use bincode::Decode;
use generated_code::{AbilityLike, ChampionId, ItemId, Position, RuneId};

#[derive(Debug, Decode)]
pub struct GameInformation {
    pub game_time: i32,
    pub map_number: u8,
}

#[derive(Debug, Decode)]
pub struct Scoreboard {
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub riot_id: Box<str>,
    pub champion_id: ChampionId,
    pub position: Position,
}

#[derive(Decode, Debug)]
pub struct CurrentPlayer {
    pub damaging_items: Box<[ItemId]>,
    pub damaging_runes: Box<[RuneId]>,
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

#[derive(Debug, Decode)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: DamageLike<AbilityLike>,
    pub items: DamageLike<ItemId>,
    pub runes: DamageLike<RuneId>,
    pub compared_items: Box<[(ItemId, SimulatedDamages)]>,
}

#[derive(Debug, Decode)]
pub struct DragonMultipliers {
    pub earth: i32,
    pub fire: i32,
    pub chemtech: i32,
}

#[derive(Debug, Decode)]
pub struct SimulatedDamages {
    pub attacks: Attacks,
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
    pub real_armor: i32,
    pub real_magic_resist: i32,
}

#[derive(Decode, Debug)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: Box<[(ChampionId, Enemy)]>,
    pub game_information: GameInformation,
    pub scoreboard: Scoreboard,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}
