use crate::utils::{RandomInput, ToStaticStr};
use bincode::{Decode, Encode};
use std::{ops::Deref, rc::Rc};
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

#[derive(Decode, Clone, Copy)]
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

#[derive(Decode, Clone, Copy)]
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
    pub active_player: BorrowedActivePlayer<'a>,
    pub enemy_players: &'a [Rc<OwnedMinData<SimpleStatsF32>>],
    pub stack_exceptions: &'a [StackException],
    pub ally_dragons: Dragons,
    pub enemy_earth_dragons: u8,
}

#[derive(Encode, PartialEq, Clone)]
pub struct ActivePlayer<
    T: Clone,
    R: PartialEq + Deref<Target = [RuneId]>,
    I: PartialEq + Deref<Target = [ItemId]>,
> {
    pub runes: R,
    pub abilities: AbilityLevels,
    pub data: MinData<T, I>,
}

#[derive(Encode, Clone, PartialEq)]
pub struct MinData<T, I: PartialEq + Deref<Target = [ItemId]>> {
    pub stats: T,
    pub items: I,
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

pub type OwnedActivePlayer = ActivePlayer<StatsF32, Vec<RuneId>, Vec<ItemId>>;
pub type BorrowedActivePlayer<'a> = ActivePlayer<StatsF32, &'a [RuneId], &'a [ItemId]>;
pub type OwnedMinData<T> = MinData<T, Vec<ItemId>>;
pub type BorrowedMinData<'a, T> = MinData<T, &'a [ItemId]>;

#[derive(Encode)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    Unknown,
}

#[derive(Encode, Default)]
pub struct Dragons {
    pub earth: u8,
    pub fire: u8,
}

#[derive(Encode, Clone, Copy, PartialEq)]
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

#[derive(Encode, Clone, Copy, PartialEq)]
pub struct SimpleStatsF32 {
    pub armor: f32,
    pub health: f32,
    pub magic_resist: f32,
}

#[derive(Encode, Decode, Clone, Copy, PartialEq)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

impl<T: Default> Default for OwnedMinData<T> {
    fn default() -> Self {
        Self::new(RandomInput::champion_id(), T::default())
    }
}

impl<'a, T: Copy> From<&'a OwnedMinData<T>> for BorrowedMinData<'a, T> {
    fn from(value: &'a OwnedMinData<T>) -> Self {
        Self {
            stats: value.stats,
            items: &value.items,
            stacks: value.stacks,
            level: value.level,
            infer_stats: value.infer_stats,
            is_mega_gnar: value.is_mega_gnar,
            champion_id: value.champion_id,
        }
    }
}

impl<'a> From<&'a OwnedActivePlayer> for BorrowedActivePlayer<'a> {
    fn from(value: &'a OwnedActivePlayer) -> Self {
        Self {
            runes: &value.runes,
            abilities: value.abilities,
            data: (&value.data).into(),
        }
    }
}

impl<T> OwnedMinData<T> {
    pub fn new(champion_id: ChampionId, stats: T) -> Self {
        Self {
            stats,
            items: RandomInput::recommended_items(champion_id).to_vec(),
            stacks: 0,
            level: 1,
            infer_stats: true,
            is_mega_gnar: false,
            champion_id,
        }
    }
}

impl Default for OwnedActivePlayer {
    fn default() -> Self {
        Self::new(RandomInput::champion_id())
    }
}

impl Default for StatsF32 {
    fn default() -> Self {
        Self {
            ability_power: 0.0,
            armor: 50.0,
            armor_penetration_flat: 0.0,
            armor_penetration_percent: 0.0,
            attack_damage: 90.0,
            attack_range: 0.0,
            attack_speed: 0.7,
            crit_chance: 0.0,
            crit_damage: 175.0,
            current_health: 1000.0,
            magic_penetration_flat: 0.0,
            magic_penetration_percent: 0.0,
            magic_resist: 50.0,
            health: 1000.0,
            mana: 500.0,
            current_mana: 500.0,
        }
    }
}

impl Default for SimpleStatsF32 {
    fn default() -> Self {
        Self {
            armor: 50.0,
            health: 1000.0,
            magic_resist: 50.0,
        }
    }
}

impl OwnedActivePlayer {
    pub fn new(champion_id: ChampionId) -> Self {
        Self {
            runes: RandomInput::recommended_runes(champion_id).to_vec(),
            abilities: AbilityLevels {
                q: 1,
                w: 1,
                e: 1,
                r: 1,
            },
            data: OwnedMinData::new(
                champion_id,
                StatsF32 {
                    ability_power: 0.0,
                    armor: 50.0,
                    armor_penetration_flat: 0.0,
                    armor_penetration_percent: 0.0,
                    attack_damage: 90.0,
                    attack_range: 0.0,
                    attack_speed: 0.7,
                    crit_chance: 0.0,
                    crit_damage: 175.0,
                    current_health: 1000.0,
                    magic_penetration_flat: 0.0,
                    magic_penetration_percent: 0.0,
                    magic_resist: 50.0,
                    health: 1000.0,
                    mana: 500.0,
                    current_mana: 500.0,
                },
            ),
        }
    }
}

impl From<StatsI32> for StatsF32 {
    fn from(value: StatsI32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<SimpleStatsI32> for SimpleStatsF32 {
    fn from(value: SimpleStatsI32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl ToStaticStr for DamageType {
    fn as_static_str(&self) -> &'static str {
        match self {
            Self::Physical => "text-orange-500",
            Self::Magic => "text-sky-500",
            Self::True => "text-white",
            Self::Adaptative => "text-pink-500",
            Self::Mixed => "text-violet-500",
            Self::Unknown => "text-emerald-500",
        }
    }
}
