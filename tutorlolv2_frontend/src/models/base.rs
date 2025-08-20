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
    pub minimum_damage: f32,
    pub maximum_damage: f32,
    pub damage_type: DamageType,
}

#[derive(Debug, Encode, Clone, Copy, Decode, PartialEq, Default)]
pub struct Stats {
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
    pub max_health: f32,
    pub max_mana: f32,
    pub current_mana: f32,
}

pub type DamageLike<T> = Vec<(T, InstanceDamage)>;

#[derive(Debug, Encode, Decode, Copy, Clone, PartialEq, Default)]
pub struct BasicStats {
    pub armor: f32,
    pub health: f32,
    pub attack_damage: f32,
    pub magic_resist: f32,
    pub mana: f32,
}

#[derive(Debug, Decode)]
pub struct DamageValue {
    pub minimum_damage: f32,
    pub maximum_damage: f32,
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
    pub message: Box<str>,
}
