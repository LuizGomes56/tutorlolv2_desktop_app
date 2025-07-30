use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: String,
}

#[derive(Debug, Serialize, Clone, Copy, Deserialize, PartialEq, Default)]
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

pub type DamageLike<T> = Vec<(T, InstanceDamage)>;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Default)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Debug, Deserialize)]
pub struct SimulatedDamages {
    pub abilities: Vec<(String, InstanceDamage)>,
    pub items: DamageLike<u32>,
    pub runes: DamageLike<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Damages {
    pub abilities: Vec<(String, InstanceDamage)>,
    pub items: DamageLike<u32>,
    pub runes: DamageLike<u32>,
    pub compared_items: Vec<(u32, SimulatedDamages)>,
}

#[derive(Debug, Deserialize)]
pub struct DragonMultipliers {
    pub earth: f64,
    pub fire: f64,
    pub chemtech: f64,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct SpriteInner {
    pub f: u8,
    pub w: u32,
    pub h: u32,
    pub x: u32,
    pub y: u32,
}
