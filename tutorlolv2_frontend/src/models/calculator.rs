use super::base::{AbilityLevels, BasicStats, DamageLike, InstanceDamage, Stats};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MonsterDamages {
    pub tower: f64,
    pub dragon: f64,
    pub baron: f64,
    pub atakhan: f64,
    pub voidgrubs: f64,
    pub melee_minion: f64,
    pub ranged_minion: f64,
    pub super_minion: f64,
    pub red_buff: f64,
    pub blue_buff: f64,
    pub gromp: f64,
    pub krug: f64,
    pub wolves: f64,
    pub raptor: f64,
}

#[derive(Debug, Deserialize)]
pub struct OutputGame {
    pub monster_damages: MonsterDamages,
    pub current_player: OutputCurrentPlayer,
    pub enemies: Vec<(String, OutputEnemy)>,
    pub recommended_items: Vec<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OutputCurrentPlayer {
    pub champion_id: String,
    pub damaging_items: Vec<u32>,
    pub damaging_runes: Vec<u32>,
    pub level: u8,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Debug, Deserialize)]
pub struct CalculatorDamages {
    pub abilities: Vec<(String, InstanceDamage)>,
    pub items: DamageLike<u32>,
    pub runes: DamageLike<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OutputEnemy {
    pub champion_name: String,
    pub level: u8,
    pub damages: CalculatorDamages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputActivePlayer {
    pub champion_id: &'static str,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: Vec<u32>,
    pub runes: Vec<u32>,
    pub level: u8,
    pub stacks: u32,
    pub infer_stats: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputEnemyPlayers {
    pub champion_name: &'static str,
    pub items: Vec<u32>,
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: Vec<InputEnemyPlayers>,
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    // pub stack_exceptions: FxHashMap<u32, u32>,
}

impl Default for InputGame {
    fn default() -> Self {
        Self {
            active_player: InputActivePlayer {
                champion_id: "Vex",
                champion_stats: Default::default(),
                abilities: AbilityLevels {
                    q: 5,
                    w: 5,
                    e: 5,
                    r: 3,
                },
                level: 15,
                infer_stats: false,
                items: vec![3115, 3153, 3100],
                runes: Default::default(),
                stacks: Default::default(),
            },
            enemy_players: Vec::from_iter([InputEnemyPlayers {
                champion_name: "Gwen",
                level: 15,
                infer_stats: true,
                items: Default::default(),
                stats: Default::default(),
            }]),
            ally_earth_dragons: 0,
            ally_fire_dragons: 0,
            enemy_earth_dragons: 0,
            // stack_exceptions: Default::default(),
        }
    }
}
