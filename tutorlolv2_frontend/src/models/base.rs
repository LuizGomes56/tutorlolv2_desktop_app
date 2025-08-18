use bincode::{Decode, Encode};

#[derive(Debug, Copy, Clone, Decode, Default)]
pub enum DamageType {
    Physical,
    Magic,
    Mixed,
    True,
    Adaptative,
    #[default]
    Unknown,
}

#[derive(Debug, Decode, Copy, Clone)]
pub enum AdaptativeType {
    Physical,
    Magic,
}

impl AdaptativeType {
    pub const fn get_color(&self) -> &'static str {
        match self {
            Self::Magic => DamageType::get_color(&DamageType::Magic),
            Self::Physical => DamageType::get_color(&DamageType::Physical),
        }
    }
}

impl DamageType {
    pub const fn get_color(&self) -> &'static str {
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

#[derive(Debug, Decode)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: DamageType,
}

#[derive(Debug, Encode, Clone, Copy, Decode, PartialEq, Default)]
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

#[derive(Debug, Encode, Decode, Copy, Clone, PartialEq, Default)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Debug, Decode)]
pub struct DamageValue {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
}

#[derive(Debug, Decode)]
pub struct Attacks {
    pub basic_attack: DamageValue,
    pub critical_strike: DamageValue,
    pub onhit_damage: DamageValue,
}

#[derive(Debug, Copy, Clone, Encode, Decode, PartialEq)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

#[derive(Debug, Decode)]
pub struct ApiError {
    pub message: String,
}
