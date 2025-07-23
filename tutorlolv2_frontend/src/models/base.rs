use rustc_hash::FxHashMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;

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

pub type DamageLike<T> = BTreeMap<T, InstanceDamage>;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Default)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Debug, Deserialize)]
pub struct ItemDef {
    pub name: String,
    pub gold_cost: u16,
    pub prettified_stats: BTreeMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct SimulatedDamages {
    #[serde(deserialize_with = "ord_abilities_map")]
    pub abilities: Vec<(String, InstanceDamage)>,
    pub items: DamageLike<u32>,
    pub runes: DamageLike<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Damages {
    #[serde(deserialize_with = "ord_abilities_map")]
    pub abilities: Vec<(String, InstanceDamage)>,
    pub items: DamageLike<u32>,
    pub runes: DamageLike<u32>,
    pub compared_items: BTreeMap<u32, SimulatedDamages>,
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

const ABILITY_ORDER: [char; 7] = ['A', 'C', 'P', 'Q', 'W', 'E', 'R'];

fn build_priority_map() -> FxHashMap<char, usize> {
    ABILITY_ORDER
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect()
}

pub(super) fn ord_abilities_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut items: Vec<String> = Vec::deserialize(deserializer)?;
    let priority = build_priority_map();

    items.sort_by(|a, b| {
        let pa = a
            .chars()
            .next()
            .and_then(|c| priority.get(&c).copied())
            .unwrap_or(usize::MAX);
        let pb = b
            .chars()
            .next()
            .and_then(|c| priority.get(&c).copied())
            .unwrap_or(usize::MAX);
        pa.cmp(&pb).then_with(|| a.cmp(b))
    });

    Ok(items)
}

pub(super) fn ord_abilities_map<'de, D>(
    deserializer: D,
) -> Result<Vec<(String, InstanceDamage)>, D::Error>
where
    D: Deserializer<'de>,
{
    let priority = build_priority_map();
    let raw_map: FxHashMap<String, InstanceDamage> = FxHashMap::deserialize(deserializer)?;
    let mut items: Vec<_> = raw_map.into_iter().collect();

    items.sort_by(|(ka, _), (kb, _)| {
        let pa = ka
            .chars()
            .next()
            .and_then(|c| priority.get(&c).copied())
            .unwrap_or(usize::MAX);
        let pb = kb
            .chars()
            .next()
            .and_then(|c| priority.get(&c).copied())
            .unwrap_or(usize::MAX);

        pa.cmp(&pb).then_with(|| ka.cmp(kb))
    });

    Ok(items)
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct SpriteInner {
    pub f: u8,
    pub w: u32,
    pub h: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpriteMap {
    pub abilities: FxHashMap<String, SpriteInner>,
    pub champions: FxHashMap<String, SpriteInner>,
    pub items: FxHashMap<u32, SpriteInner>,
}
