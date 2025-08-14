use bincode::{Decode, Encode};

pub enum StatName {
    AbilityHaste(f64),
    AbilityPower(f64),
    Armor(f64),
    ArmorPenetration(f64),
    MagicPenetration(f64),
    AttackDamage(f64),
    AttackSpeed(f64),
    GoldPer10Seconds(f64),
    AdaptiveForce(f64),
    CriticalStrikeChance(f64),
    CriticalStrikeDamage(f64),
    Health(f64),
    LifeSteal(f64),
    MagicResist(f64),
    Mana(f64),
    MoveSpeed(f64),
    Omnivamp(f64),
    BaseHealthRegen(f64),
    BaseManaRegen(f64),
    Tenacity(f64),
    HealAndShieldPower(f64),
}

impl StatName {
    pub fn info(&self) -> (&'static str, &'static str, f64) {
        match self {
            Self::AbilityPower(value) => ("/img/stats/ability_power.svg", "Ability Power", *value),
            Self::Health(value) => ("/img/stats/health.svg", "Health", *value),
            Self::AbilityHaste(value) => ("/img/stats/ability_haste.svg", "Ability Haste", *value),
            Self::MoveSpeed(value) => ("/img/stats/move_speed.svg", "Move Speed", *value),
            Self::LifeSteal(value) => ("/img/stats/life_steal.svg", "Life Steal", *value),
            Self::Omnivamp(value) => ("/img/stats/omnivamp.svg", "Omnivamp", *value),
            Self::ArmorPenetration(value) => (
                "/img/stats/armor_penetration.svg",
                "Armor Penetration",
                *value,
            ),
            Self::MagicPenetration(value) => (
                "/img/stats/magic_penetration.svg",
                "Magic Penetration",
                *value,
            ),
            Self::GoldPer10Seconds(value) => ("/img/stats/gold.svg", "Gold Per 10 Seconds", *value),
            Self::Tenacity(value) => ("/img/stats/tenacity.svg", "Tenacity", *value),
            Self::AttackDamage(value) => ("/img/stats/attack_damage.svg", "Attack Damage", *value),
            Self::AttackSpeed(value) => ("/img/stats/attack_speed.svg", "Attack Speed", *value),
            Self::CriticalStrikeChance(value) => (
                "/img/stats/crit_chance.svg",
                "Critical Strike Chance",
                *value,
            ),
            Self::CriticalStrikeDamage(value) => (
                "/img/stats/crit_damage.svg",
                "Critical Strike Damage",
                *value,
            ),
            Self::HealAndShieldPower(value) => (
                "/img/stats/heal_and_shield_power.svg",
                "Heal And Shield Power",
                *value,
            ),
            Self::Mana(value) => ("/img/stats/mana.svg", "Mana", *value),
            Self::Armor(value) => ("/img/stats/armor.svg", "Armor", *value),
            Self::BaseManaRegen(value) => (
                "/img/stats/mana_regeneration.svg",
                "Base Mana Regen",
                *value,
            ),
            Self::BaseHealthRegen(value) => (
                "/img/stats/health_regeneration.svg",
                "Base Health Regen",
                *value,
            ),
            Self::MagicResist(value) => ("/img/stats/magic_resist.svg", "Magic Resist", *value),
            Self::AdaptiveForce(value) => {
                ("/img/stats/adaptive_force.svg", "Adaptive Force", *value)
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
    A,
    C,
    O,
}

impl AbilityLike {
    pub fn as_char(&self) -> char {
        match self {
            Self::P(_) => 'P',
            Self::Q(_) => 'Q',
            Self::W(_) => 'W',
            Self::E(_) => 'E',
            Self::R(_) => 'R',
            Self::A => 'A',
            Self::C => 'C',
            Self::O => 'O',
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Encode, Decode)]
pub enum AbilityName {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    Mega,
    Max,
    Min,
    Minion,
    Minion1,
    Minion2,
    Minion3,
    MinionMax,
    Monster,
    Monster1,
    Monster2,
    Monster3,
    Monster4,
    MonsterMax,
    Void,
    _1Max,
    _2Max,
    _3Max,
    _4Max,
    _5Max,
    _6Max,
    _7Max,
    _8Max,
    _1Min,
    _2Min,
    _3Min,
    _4Min,
    _5Min,
    _6Min,
    _7Min,
    _8Min,
}

pub struct ItemDescription {
    pub name: &'static str,
    pub prettified_stats: &'static [StatName],
    pub gold_cost: u16,
}
