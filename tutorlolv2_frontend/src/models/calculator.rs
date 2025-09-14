use super::base::{
    AbilityLevels, AdaptativeType, Attacks, BasicStats, DamageLike, InstanceDamage, Stats,
};
use crate::{components::tables::cells::DisplayDamage, utils::RandomInput};
use bincode::{Decode, Encode};
use tutorlolv2_imports::{AbilityLike, ChampionId, ItemId, RuneId};
use std::rc::Rc;
use yew::{Html, html};

#[derive(Debug, Decode)]
pub struct MonsterExpr {
    pub attacks: Attacks,
    pub abilities: Box<[InstanceDamage]>,
    pub items: Box<[InstanceDamage]>,
}

#[derive(Debug, Decode)]
pub struct MonsterDamages([MonsterExpr; 7]);

impl MonsterDamages {
    pub fn join_td_index(&self, index: usize) -> Html {
        let Some(monster_expr) = self.0.get(index) else {
            return html! {};
        };

        html! {
            <>
                {monster_expr.attacks.display_damage()}
                {monster_expr.abilities.display_damage()}
                {monster_expr.items.display_damage()}
            </>
        }
    }
}

#[derive(Debug, Decode)]
pub struct OutputGame {
    pub monster_damages: MonsterDamages,
    pub tower_damage: [i32; 6],
    pub current_player: OutputCurrentPlayer,
    pub enemies: Box<[(ChampionId, OutputEnemy)]>,
}

#[derive(Debug, Decode)]
pub struct OutputCurrentPlayer {
    pub champion_id: ChampionId,
    pub damaging_items: Box<[ItemId]>,
    pub damaging_runes: Box<[RuneId]>,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub stats: Stats,
}

#[derive(Debug, Decode)]
pub struct CalculatorDamages {
    pub attacks: Attacks,
    pub abilities: Box<[(AbilityLike, InstanceDamage)]>,
    pub items: DamageLike<ItemId>,
    pub runes: DamageLike<RuneId>,
}

#[derive(Debug, Decode)]
pub struct OutputEnemy {
    pub level: u8,
    pub damages: CalculatorDamages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: i32,
    pub real_magic_resist: i32,
}

#[derive(Clone, Debug, PartialEq, Encode)]
pub struct InputCurrentPlayer {
    pub champion_id: ChampionId,
    pub stats: Stats,
    pub abilities: AbilityLevels,
    pub items: Vec<ItemId>,
    pub runes: Vec<RuneId>,
    pub level: u8,
    pub stacks: u32,
    pub infer_stats: bool,
}

#[derive(Clone, Debug, PartialEq, Encode)]
pub struct InputEnemyPlayer {
    pub champion_id: ChampionId,
    pub items: Vec<ItemId>,
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
    pub stacks: u32,
}

#[derive(Clone, Debug, Encode)]
pub struct InputGame<'a> {
    pub active_player: &'a InputCurrentPlayer,
    pub enemy_players: &'a [Rc<InputEnemyPlayer>],
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    pub stack_exceptions: Vec<(u8, u16, u8)>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct InputDragons {
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
}

impl InputCurrentPlayer {
    #[inline]
    pub fn create(&self, champion_id: ChampionId) -> Self {
        Self {
            champion_id,
            items: RandomInput::recommended_items(champion_id),
            runes: self.runes.clone(),
            ..*self
        }
    }
    #[inline]
    pub fn new() -> Self {
        Self::create(&Self::default(), RandomInput::champion_id())
    }
}

impl Default for InputCurrentPlayer {
    fn default() -> Self {
        Self {
            champion_id: RandomInput::champion_id(),
            items: Default::default(),
            level: 18,
            stats: Default::default(),
            infer_stats: true,
            stacks: 0,
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            runes: Default::default(),
        }
    }
}

impl Default for InputEnemyPlayer {
    fn default() -> Self {
        Self {
            champion_id: RandomInput::champion_id(),
            items: Default::default(),
            level: 18,
            stats: Default::default(),
            infer_stats: true,
            stacks: 0,
        }
    }
}

impl InputEnemyPlayer {
    #[inline]
    pub fn create(&self, champion_id: ChampionId) -> Self {
        Self {
            champion_id,
            items: RandomInput::recommended_items(champion_id),
            ..*self
        }
    }
    #[inline]
    pub fn new() -> Self {
        Self::create(&Self::default(), RandomInput::champion_id())
    }
}
