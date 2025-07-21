#[macro_export]
macro_rules! svg {
    ($path:literal, $size:expr) => {{
        let content = include_str!(concat!($path, ".svg"));
        yew::virtual_dom::VNode::from_html_unchecked(content.replace("{size}", $size).into())
    }};
}

#[macro_export]
macro_rules! color {
    (@inner) => { "zinc" };
    ($property:ident-$weight:literal) => {
        concat!(stringify!($property), "-", color!(@inner), "-", $weight)
    };
    ($property:ident-$alignment:ident-$weight:literal) => {
        concat!(
            stringify!($property),
            "-",
            stringify!($alignment),
            "-",
            color!(@inner),
            "-",
            $weight
        )
    };
    ($modifier:ident:$property:ident-$weight:literal) => {
        concat!(
            stringify!($modifier),
            ":",
            stringify!($property),
            "-",
            color!(@inner),
            "-",
            $weight
        )
    };
}

#[macro_export]
macro_rules! url {
    (@inner) => { "http://localhost:8082" };
    ($path:literal) => {
        concat!(url!(@inner), $path)
    };
    ($fmt:literal $(, $vars:expr)*) => {
        format!(
            concat!(url!(@inner), $fmt)
            $(, $vars)*
        )
    };
}

pub static STATS_URL: phf::Map<&'static str, &'static str> = phf::phf_map!(
    "Health" => url!("/img/stats/health.svg"),
    "Ability Haste" => url!("/img/stats/ability_haste.svg"),
    "Move Speed" => url!("/img/stats/move_speed.svg"),
    "Life Steal" => url!("/img/stats/life_steal.svg"),
    "Omnivamp" => url!("/img/stats/omnivamp.svg"),
    "Armor Penetration" => url!("/img/stats/armor_penetration.svg"),
    "Lethality" => url!("/img/stats/lethality.svg"),
    "Magic Penetration" => url!("/img/stats/magic_penetration.svg"),
    "Gold Per 10 Seconds" => url!("/img/stats/gold.svg"),
    "Tenacity" => url!("/img/stats/tenacity.svg"),
    "Attack Damage" => url!("/img/stats/attack_damage.svg"),
    "Attack Speed" => url!("/img/stats/attack_speed.svg"),
    "Critical Strike Chance" => url!("/img/stats/crit_chance.svg"),
    "Critical Strike Damage" => url!("/img/stats/crit_damage.svg"),
    "Heal And Shield Power" => url!("/img/stats/heal_and_shield_power.svg"),
    "Mana" => url!("/img/stats/mana.svg"),
    "Armor" => url!("/img/stats/armor.svg"),
    "Base Mana Regen" => url!("/img/stats/mana_regeneration.svg"),
    "Base Health Regen" => url!("/img/stats/health_regeneration.svg"),
    "Magic Resist" => url!("/img/stats/magic_resist.svg"),
    "Ability Power" => url!("/img/stats/ability_power.svg"),
);
