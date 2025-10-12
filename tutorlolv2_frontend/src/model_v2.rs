use crate::utils::{RandomInput, ToStaticStr};
use bincode::{Decode, Encode};
use std::{ops::Deref, rc::Rc};
use tutorlolv2_imports::{
    AbilityLike, ChampionId, ITEM_ID_TO_NAME, ItemId, Position, RUNE_ID_TO_NAME, RuneId,
};

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
pub struct RangeDamage {
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

#[derive(Decode)]
pub struct BasicStats {
    pub armor: i32,
    pub health: i32,
    pub attack_damage: i32,
    pub magic_resist: i32,
    pub mana: i32,
}

#[derive(Decode)]
pub struct Attacks {
    pub basic_attack: RangeDamage,
    pub critical_strike: RangeDamage,
    pub onhit_damage: RangeDamage,
}

#[derive(Decode)]
pub struct TypeMetadata<T> {
    pub kind: T,
    pub damage_type: DamageType,
    pub attributes: Attrs,
}

#[derive(Decode)]
pub struct Realtime {
    pub current_player: CurrentPlayer,
    pub enemies: Box<[Enemy]>,
    pub scoreboard: Box<[Scoreboard]>,
    pub abilities_meta: Box<[TypeMetadata<AbilityLike>]>,
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
    pub siml_meta: [TypeMetadata<ItemId>; L_SIML],
    pub game_time: u32,
    pub ability_levels: AbilityLevels,
    pub dragons: Dragons,
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

#[derive(Encode, Decode, Clone, Copy, Default, PartialEq)]
pub struct Stats {
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
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
    pub level: u8,
    pub team: Team,
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub game_map: GameMap,
}

#[derive(Encode, Decode, Clone, Copy, Default, PartialEq)]
pub struct SimpleStats {
    pub armor: i32,
    pub health: i32,
    pub magic_resist: i32,
}

#[derive(Decode)]
pub struct Enemy {
    pub riot_id: String,
    pub damages: Damages,
    pub siml_items: [Damages; L_SIML],
    pub base_stats: SimpleStats,
    pub bonus_stats: SimpleStats,
    pub current_stats: SimpleStats,
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
    pub abilities: Box<[RangeDamage]>,
    pub items: Box<[RangeDamage]>,
    pub runes: Box<[RangeDamage]>,
}

#[derive(Decode)]
pub struct OutputEnemy {
    pub damages: Damages,
    pub base_stats: SimpleStats,
    pub bonus_stats: SimpleStats,
    pub current_stats: SimpleStats,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
}

#[derive(Decode)]
pub struct OutputCurrentPlayer {
    pub current_stats: Stats,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub champion_id: ChampionId,
}

#[derive(Decode)]
pub struct MonsterDamage {
    pub attacks: Attacks,
    pub abilities: Box<[RangeDamage]>,
    pub items: Box<[RangeDamage]>,
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

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq)]
pub struct Dragons {
    pub ally_fire_dragons: u16,
    pub ally_earth_dragons: u16,
    pub ally_chemtech_dragons: u16,
    pub enemy_earth_dragons: u16,
}

#[derive(Encode)]
pub struct InputGame<'a> {
    pub active_player: BorrowedActivePlayer<'a>,
    pub enemy_players: &'a [Rc<OwnedMinData<SimpleStats>>],
    pub dragons: Dragons,
}

#[derive(Encode, PartialEq, Clone)]
pub struct ActivePlayer<
    Stats: Clone,
    Runes: PartialEq + Deref<Target = [RuneId]>,
    Items: PartialEq + Deref<Target = [ItemId]>,
    Vexcp: PartialEq + Deref<Target = [ValueException]>,
> {
    pub runes: Runes,
    pub rune_exceptions: Vexcp,
    pub abilities: AbilityLevels,
    pub data: MinData<Stats, Items, Vexcp>,
}

#[derive(Encode, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct ValueException(u32);

impl ValueException {
    pub const NUMBER_OF_ITEMS: u32 = ITEM_ID_TO_NAME.len() as u32;
    pub const NUMBER_OF_RUNES: u32 = RUNE_ID_TO_NAME.len() as u32;
    pub const DISC_BITS: u32 =
        Self::find_disc_bits(Self::NUMBER_OF_ITEMS as u32, Self::NUMBER_OF_RUNES as u32);
    pub const VAL_BITS: u32 = 32 - Self::DISC_BITS;
    pub const VAL_MASK: u32 = (1u32 << Self::VAL_BITS) - 1;
    pub const DISC_MASK: u32 = !Self::VAL_MASK;
    pub const DISC_LOW_MASK: u32 = (1u32 << Self::DISC_BITS) - 1;

    const fn find_disc_bits(a: u32, b: u32) -> u32 {
        u32::BITS - if a > b { a } else { b }.leading_zeros()
    }

    const fn truncate_value(v: u32) -> u32 {
        v & Self::VAL_MASK
    }

    pub const fn pack_rune_id(r: RuneId, v: u32) -> Self {
        let disc = (r as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }

    pub const fn pack_item_id(i: ItemId, v: u32) -> Self {
        let disc = (i as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }
}

#[derive(Encode, Clone, PartialEq)]
pub struct MinData<
    Stats,
    Items: PartialEq + Deref<Target = [ItemId]>,
    Vexcp: PartialEq + Deref<Target = [ValueException]>,
> {
    pub stats: Stats,
    pub items: Items,
    pub item_exceptions: Vexcp,
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

pub type OwnedActivePlayer = ActivePlayer<Stats, Vec<RuneId>, Vec<ItemId>, Vec<ValueException>>;
pub type BorrowedActivePlayer<'a> =
    ActivePlayer<Stats, &'a [RuneId], &'a [ItemId], &'a [ValueException]>;
pub type OwnedMinData<T> = MinData<T, Vec<ItemId>, Vec<ValueException>>;
pub type BorrowedMinData<'a, T> = MinData<T, &'a [ItemId], &'a [ValueException]>;

#[derive(Decode)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    Unknown,
}

#[derive(Encode, Decode, Clone, Copy, Default, PartialEq)]
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
            item_exceptions: &value.item_exceptions,
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
            rune_exceptions: &value.rune_exceptions,
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
            item_exceptions: Vec::new(),
            champion_id,
            stacks: 0,
            level: 1,
            infer_stats: true,
            is_mega_gnar: false,
        }
    }
}

impl Default for OwnedActivePlayer {
    fn default() -> Self {
        Self::new(RandomInput::champion_id())
    }
}

impl OwnedActivePlayer {
    pub fn new(champion_id: ChampionId) -> Self {
        Self {
            runes: RandomInput::recommended_runes(champion_id).to_vec(),
            rune_exceptions: Vec::new(),
            abilities: AbilityLevels::default(),
            data: OwnedMinData::new(champion_id, Stats::default()),
        }
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
