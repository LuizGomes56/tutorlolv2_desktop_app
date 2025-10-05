use bincode::{Decode, Encode};
use tutorlolv2_imports::{AbilityLike, ChampionId, ItemId, Position, RuneId};

pub const L_MSTR: usize = 7;
pub const L_TWRD: usize = 6;
pub const L_SIML: usize = 118;

#[derive(Decode)]
pub enum Attrs {
    None,
    Onhit,
    OnhitMin,
    OnhitMax,
    Area,
    AreaOnhit,
    AreaOnhitMin,
    AreaOnhitMax,
}

#[derive(Decode)]
pub enum AdaptativeType {
    Physical,
    Magic,
}

#[derive(Decode)]
pub enum GameMap {
    SummonersRift,
    Tutorial,
    TwistedTreeline,
    Dominion,
    Aram,
    DarkStar,
    Invasion,
    Project,
    StarGuardian,
    Odyssey,
    NexusBlitz,
    Tft,
    Arena,
    Urf,
}

#[derive(Decode)]
pub enum Team {
    Blue,
    Red,
}

#[derive(Decode)]
pub struct RangeDamageI32 {
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

#[derive(Decode)]
pub struct BasicStatsI32 {
    pub armor: i32,
    pub health: i32,
    pub attack_damage: i32,
    pub magic_resist: i32,
    pub mana: i32,
}

#[derive(Decode)]
pub struct Attacks {
    pub basic_attack: RangeDamageI32,
    pub critical_strike: RangeDamageI32,
    pub onhit_damage: RangeDamageI32,
}

#[derive(Decode)]
pub struct TypeMetadata<T> {
    pub level: u8,
    pub kind: T,
    pub meta: Meta,
}

#[derive(Decode)]
pub struct Meta(pub u8);

impl Meta {
    pub const fn damage_type(&self) -> DamageType {
        unsafe { std::mem::transmute((self.0 >> 5) & 0b0000_0111) }
    }
    pub const fn attributes(&self) -> Attrs {
        unsafe { std::mem::transmute(self.0 & 0b0001_1111) }
    }
}

#[derive(Decode)]
pub struct ConstItemMetadata {
    pub kind: ItemId,
    pub meta: Meta,
}

#[derive(Decode)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: Box<[Enemy]>,
    pub scoreboard: Box<[Scoreboard]>,
    pub abilities_meta: Box<[TypeMetadata<AbilityLike>]>,
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
    pub siml_meta: [ConstItemMetadata; L_SIML],
    pub game_time: u32,
    pub ability_levels: AbilityLevels,
}

#[derive(Decode)]
pub struct Scoreboard {
    pub riot_id: String,
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub champion_id: ChampionId,
    pub position: Position,
    pub team: Team,
}

#[derive(Decode)]
pub struct StatsI32 {
    pub ability_power: i32,
    pub armor: i32,
    pub armor_penetration_flat: i32,
    pub armor_penetration_percent: i32,
    pub attack_damage: i32,
    pub attack_range: i32,
    pub attack_speed: i32,
    pub crit_chance: i32,
    pub crit_damage: i32,
    pub current_health: i32,
    pub magic_penetration_flat: i32,
    pub magic_penetration_percent: i32,
    pub magic_resist: i32,
    pub health: i32,
    pub mana: i32,
    pub current_mana: i32,
}

#[derive(Decode)]
pub struct CurrentPlayer {
    pub riot_id: String,
    pub base_stats: BasicStatsI32,
    pub bonus_stats: BasicStatsI32,
    pub current_stats: StatsI32,
    pub level: u8,
    pub team: Team,
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub game_map: GameMap,
}

#[derive(Decode)]
pub struct SimpleStatsI32 {
    pub armor: i32,
    pub health: i32,
    pub magic_resist: i32,
}

#[derive(Decode)]
pub struct Enemy {
    pub riot_id: String,
    pub damages: Damages,
    pub siml_items: [Damages; L_SIML],
    pub base_stats: SimpleStatsI32,
    pub bonus_stats: SimpleStatsI32,
    pub current_stats: SimpleStatsI32,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
    pub team: Team,
    pub position: Position,
}

#[derive(Decode)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: Box<[RangeDamageI32]>,
    pub items: Box<[RangeDamageI32]>,
    pub runes: Box<[RangeDamageI32]>,
}

#[derive(Decode)]
pub struct OutputEnemy {
    pub damages: Damages,
    pub base_stats: SimpleStatsI32,
    pub bonus_stats: SimpleStatsI32,
    pub current_stats: SimpleStatsI32,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
}

#[derive(Decode)]
pub struct OutputCurrentPlayer {
    pub current_stats: StatsI32,
    pub base_stats: BasicStatsI32,
    pub bonus_stats: BasicStatsI32,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub champion_id: ChampionId,
}

#[derive(Decode)]
pub struct MonsterDamage {
    pub attacks: Attacks,
    pub abilities: Box<[RangeDamageI32]>,
    pub items: Box<[RangeDamageI32]>,
}

#[derive(Decode)]
pub struct OutputGame {
    pub monster_damages: [MonsterDamage; L_MSTR],
    pub current_player: OutputCurrentPlayer,
    pub enemies: Box<[OutputEnemy]>,
    pub tower_damages: [i32; L_TWRD],
    pub abilities_meta: Box<[TypeMetadata<AbilityLike>]>,
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
}

#[derive(Encode)]
pub struct StackExceptionKind<T> {
    pub kind: T,
    pub stacks: u16,
    pub offset: u8,
}

#[derive(Encode)]
pub enum StackException {
    Item(StackExceptionKind<ItemId>),
    Rune(StackExceptionKind<RuneId>),
    Champion(StackExceptionKind<ChampionId>),
}

#[derive(Encode)]
pub struct InputGame<'a> {
    pub active_player: InputActivePlayer<'a>,
    pub enemy_players: &'a [InputMinData<'a, SimpleStatsF32>],
    pub stack_exceptions: &'a [StackException],
    pub ally_dragons: Dragons,
    pub enemy_earth_dragons: u8,
}

#[derive(Encode)]
pub struct InputActivePlayer<'a> {
    pub runes: &'a [RuneId],
    pub abilities: AbilityLevels,
    pub data: InputMinData<'a, StatsF32>,
}

#[derive(Encode)]
pub struct InputMinData<'a, T> {
    pub stats: T,
    pub items: &'a [ItemId],
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

#[derive(Encode)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    Unknown,
}

#[derive(Encode)]
pub struct Dragons {
    pub earth: u8,
    pub fire: u8,
}

#[derive(Encode)]
pub struct StatsF32 {
    pub ability_power: f32,
    pub armor: f32,
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub current_health: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub magic_resist: f32,
    pub health: f32,
    pub mana: f32,
    pub current_mana: f32,
}

#[derive(Encode)]
pub struct SimpleStatsF32 {
    pub armor: f32,
    pub health: f32,
    pub magic_resist: f32,
}

#[derive(Encode, Decode)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}
