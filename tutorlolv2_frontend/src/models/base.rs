use std::collections::BTreeMap;

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Stats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub armor_penetration_percent: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub max_mana: f64,
    pub current_mana: f64,
}

pub type DamageLike<T> = BTreeMap<T, InstanceDamage>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ComparedItem {
    pub name: String,
    pub gold_cost: usize,
    pub prettified_stats: FxHashMap<String, f64>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SimulatedDamages {
    pub abilities: DamageLike<String>,
    pub items: DamageLike<usize>,
    pub runes: DamageLike<usize>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Damages {
    pub abilities: DamageLike<String>,
    pub items: DamageLike<usize>,
    pub runes: DamageLike<usize>,
    pub compared_items: BTreeMap<usize, SimulatedDamages>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DragonMultipliers {
    pub earth: f64,
    pub fire: f64,
    pub chemtech: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
}
