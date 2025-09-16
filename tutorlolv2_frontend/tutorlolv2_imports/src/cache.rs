use bincode::Decode;

pub enum StatName {
    AbilityHaste(u16),
    AbilityPower(u16),
    Armor(u16),
    ArmorPenetration(u16),
    MagicPenetration(u16),
    AttackDamage(u16),
    AttackSpeed(u16),
    GoldPer10Seconds(u16),
    AdaptiveForce(u16),
    CriticalStrikeChance(u16),
    CriticalStrikeDamage(u16),
    Health(u16),
    LifeSteal(u16),
    MagicResist(u16),
    Mana(u16),
    MoveSpeed(u16),
    Omnivamp(u16),
    BaseHealthRegen(u16),
    BaseManaRegen(u16),
    Tenacity(u16),
    HealAndShieldPower(u16),
}

impl StatName {
    pub fn info(self) -> (&'static str, &'static str, u16) {
        match self {
            Self::AbilityPower(value) => ("/img/stats/ability_power.svg", "Ability Power", value),
            Self::Health(value) => ("/img/stats/health.svg", "Health", value),
            Self::AbilityHaste(value) => ("/img/stats/ability_haste.svg", "Ability Haste", value),
            Self::MoveSpeed(value) => ("/img/stats/move_speed.svg", "Move Speed", value),
            Self::LifeSteal(value) => ("/img/stats/life_steal.svg", "Life Steal", value),
            Self::Omnivamp(value) => ("/img/stats/omnivamp.svg", "Omnivamp", value),
            Self::ArmorPenetration(value) => (
                "/img/stats/armor_penetration.svg",
                "Armor Penetration",
                value,
            ),
            Self::MagicPenetration(value) => (
                "/img/stats/magic_penetration.svg",
                "Magic Penetration",
                value,
            ),
            Self::GoldPer10Seconds(value) => ("/img/stats/gold.svg", "Gold Per 10 Seconds", value),
            Self::Tenacity(value) => ("/img/stats/tenacity.svg", "Tenacity", value),
            Self::AttackDamage(value) => ("/img/stats/attack_damage.svg", "Attack Damage", value),
            Self::AttackSpeed(value) => ("/img/stats/attack_speed.svg", "Attack Speed", value),
            Self::CriticalStrikeChance(value) => (
                "/img/stats/crit_chance.svg",
                "Critical Strike Chance",
                value,
            ),
            Self::CriticalStrikeDamage(value) => (
                "/img/stats/crit_damage.svg",
                "Critical Strike Damage",
                value,
            ),
            Self::HealAndShieldPower(value) => (
                "/img/stats/heal_and_shield_power.svg",
                "Heal And Shield Power",
                value,
            ),
            Self::Mana(value) => ("/img/stats/mana.svg", "Mana", value),
            Self::Armor(value) => ("/img/stats/armor.svg", "Armor", value),
            Self::BaseManaRegen(value) => {
                ("/img/stats/mana_regeneration.svg", "Base Mana Regen", value)
            }
            Self::BaseHealthRegen(value) => (
                "/img/stats/health_regeneration.svg",
                "Base Health Regen",
                value,
            ),
            Self::MagicResist(value) => ("/img/stats/magic_resist.svg", "Magic Resist", value),
            Self::AdaptiveForce(value) => {
                ("/img/stats/adaptive_force.svg", "Adaptive Force", value)
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Decode)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl AbilityLike {
    pub fn as_char(&self) -> char {
        match self {
            Self::P(_) => 'P',
            Self::Q(_) => 'Q',
            Self::W(_) => 'W',
            Self::E(_) => 'E',
            Self::R(_) => 'R',
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Decode)]
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
