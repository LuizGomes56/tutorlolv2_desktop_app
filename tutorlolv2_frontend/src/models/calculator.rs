use super::base::{
    AbilityLevels, AdaptativeType, Attacks, BasicStats, DamageLike, InstanceDamage, Stats,
};
use crate::components::tables::cells::DisplayDamage;
use bincode::{Decode, Encode};
use generated_code::{
    AbilityLike, CHAMPION_ID_TO_NAME, ChampionId, ItemId, RECOMMENDED_ITEMS, RuneId,
};
use web_sys::js_sys::Math;
use yew::{Html, html};

#[derive(Debug, Decode)]
pub struct MonsterExpr {
    pub attacks: Attacks,
    pub abilities: Vec<InstanceDamage>,
    pub items: Vec<InstanceDamage>,
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
    pub tower_damage: [f32; 6],
    pub current_player: OutputCurrentPlayer,
    pub enemies: Vec<(ChampionId, OutputEnemy)>,
}

#[derive(Debug, Decode)]
pub struct OutputCurrentPlayer {
    pub champion_id: ChampionId,
    pub damaging_items: Vec<ItemId>,
    pub damaging_runes: Vec<RuneId>,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub stats: Stats,
}

#[derive(Debug, Decode)]
pub struct CalculatorDamages {
    pub attacks: Attacks,
    pub abilities: Vec<(AbilityLike, InstanceDamage)>,
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
    pub real_armor: f32,
    pub real_magic_resist: f32,
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
}

#[derive(Clone, Debug, Encode)]
pub struct InputGame<'a> {
    pub active_player: &'a InputCurrentPlayer,
    pub enemy_players: &'a [std::rc::Rc<InputEnemyPlayer>],
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct InputDragons {
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
}

#[inline]
pub fn rand_num_limited(limit: f64) -> f64 {
    Math::floor(Math::random() * limit)
}

impl InputCurrentPlayer {
    #[inline]
    pub fn new() -> Self {
        let (champion_id, items) = unsafe {
            let random_number = rand_num_limited(CHAMPION_ID_TO_NAME.len() as f64);
            (
                std::mem::transmute::<_, ChampionId>(random_number as u8),
                RECOMMENDED_ITEMS
                    .get_unchecked(random_number as usize)
                    .get_unchecked(rand_num_limited(5.0) as usize)
                    .to_vec(),
            )
        };
        Self {
            champion_id,
            stats: Default::default(),
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            level: 18,
            infer_stats: true,
            items,
            runes: Default::default(),
            stacks: Default::default(),
        }
    }
}

impl InputEnemyPlayer {
    #[inline]
    pub fn new() -> Self {
        let (champion_id, items) = unsafe {
            let random_number = rand_num_limited(CHAMPION_ID_TO_NAME.len() as f64);
            (
                std::mem::transmute::<_, ChampionId>(random_number as u8),
                RECOMMENDED_ITEMS
                    .get_unchecked(random_number as usize)
                    .get_unchecked(rand_num_limited(5.0) as usize)
                    .to_vec(),
            )
        };
        Self {
            champion_id,
            level: 18,
            infer_stats: true,
            items,
            stats: Default::default(),
        }
    }
}
