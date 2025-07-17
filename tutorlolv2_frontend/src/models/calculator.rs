use super::base::{AbilityLevels, BasicStats, Damages, Stats};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Deserialize)]
pub struct OutputCurrentPlayer {
    pub champion_id: String,
    pub damaging_abilities: BTreeSet<String>,
    pub damaging_items: BTreeSet<usize>,
    pub damaging_runes: BTreeSet<usize>,
    pub level: usize,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Debug, Deserialize)]
pub struct OutputEnemy {
    pub champion_name: String,
    pub level: usize,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Debug, Deserialize)]
pub struct OutputGame {
    pub current_player: OutputCurrentPlayer,
    pub enemies: BTreeMap<String, OutputEnemy>,
    pub recommended_items: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputActivePlayer {
    pub champion_id: String,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: Vec<usize>,
    pub runes: Vec<usize>,
    pub level: u8,
    pub stacks: usize,
    pub infer_stats: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputEnemyPlayers {
    pub champion_name: String,
    pub items: Vec<usize>,
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
    pub stack_exceptions: FxHashMap<usize, u8>,
}

impl Default for InputGame {
    fn default() -> Self {
        Self {
            active_player: InputActivePlayer {
                champion_id: "Neeko".into(),
                champion_stats: Default::default(),
                abilities: AbilityLevels {
                    q: 5,
                    w: 5,
                    e: 5,
                    r: 3,
                },
                level: 15,
                infer_stats: true,
                items: Default::default(),
                runes: Default::default(),
                stacks: Default::default(),
            },
            enemy_players: Vec::from_iter([InputEnemyPlayers {
                champion_name: "Gwen".into(),
                level: 15,
                infer_stats: true,
                items: Default::default(),
                stats: Default::default(),
            }]),
            ally_earth_dragons: 0,
            ally_fire_dragons: 0,
            enemy_earth_dragons: 0,
            stack_exceptions: Default::default(),
        }
    }
}
