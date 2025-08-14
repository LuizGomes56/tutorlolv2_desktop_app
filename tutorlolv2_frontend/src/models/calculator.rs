use super::base::{AbilityLevels, BasicStats, DamageLike, InstanceDamage, Stats};
use bincode::{Decode, Encode};
use generated_code::{AbilityLike, ChampionId, ItemId, RuneId};
use yew::{AttrValue, Html, html};

#[derive(Debug, Decode)]
pub struct DamageValue {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
}

#[derive(Debug, Decode)]
pub struct MonsterExpr {
    pub abilities: Vec<DamageValue>,
    pub items: Vec<DamageValue>,
}

#[derive(Debug, Decode)]
pub struct MonsterDamages([MonsterExpr; 7]);

impl MonsterDamages {
    pub fn join_td_index(&self, index: usize) -> Html {
        let Some(monster_expr) = self.0.get(index) else {
            return html! {};
        };

        monster_expr
            .abilities
            .iter()
            .chain(monster_expr.items.iter())
            .map(|damage_value| {
                let text = if damage_value.maximum_damage != 0.0 {
                    let mut s = damage_value.minimum_damage.round().to_string();
                    s.push_str(" - ");
                    s.push_str(&damage_value.maximum_damage.round().to_string());
                    AttrValue::from(s)
                } else {
                    AttrValue::from(damage_value.minimum_damage.round().to_string())
                };
                html! {
                    <td
                        title={text.clone()}
                        class="text-center text-sm px-2 text-violet-500 max-w-24 truncate"
                    >
                        { text }
                    </td>
                }
            })
            .collect::<Html>()
    }
}

#[derive(Debug, Decode)]
pub struct OutputGame {
    pub monster_damages: MonsterDamages,
    pub tower_damage: [f64; 6],
    pub current_player: OutputCurrentPlayer,
    pub enemies: Vec<(ChampionId, OutputEnemy)>,
    pub recommended_items: Vec<ItemId>,
}

#[derive(Debug, Decode)]
pub struct OutputCurrentPlayer {
    pub champion_id: ChampionId,
    pub damaging_items: Vec<ItemId>,
    pub damaging_runes: Vec<RuneId>,
    pub level: u8,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Debug, Decode)]
pub struct CalculatorDamages {
    pub abilities: Vec<(AbilityLike, InstanceDamage)>,
    pub items: DamageLike<ItemId>,
    pub runes: DamageLike<RuneId>,
}

#[derive(Debug, Decode)]
pub struct OutputEnemy {
    pub level: u8,
    pub damages: CalculatorDamages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f64,
    pub real_magic_resist: f64,
}

#[derive(Clone, Debug, PartialEq, Encode)]
pub struct InputActivePlayer {
    pub champion_id: ChampionId,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: Vec<ItemId>,
    pub runes: Vec<RuneId>,
    pub level: u8,
    pub stacks: u32,
    pub infer_stats: bool,
}

#[derive(Clone, Debug, PartialEq, Encode)]
pub struct InputEnemyPlayers {
    pub champion_id: ChampionId,
    pub items: Vec<ItemId>,
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
}

#[derive(Clone, Debug, PartialEq, Encode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: Vec<InputEnemyPlayers>,
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    // pub stack_exceptions: hashbrown::HashMap<u32, u32>,
}

impl Default for InputGame {
    fn default() -> Self {
        Self {
            active_player: InputActivePlayer {
                champion_id: ChampionId::Vex,
                champion_stats: Default::default(),
                abilities: AbilityLevels {
                    q: 5,
                    w: 5,
                    e: 5,
                    r: 3,
                },
                level: 15,
                infer_stats: true,
                items: vec![
                    ItemId::NashorsTooth,
                    ItemId::BladeoftheRuinedKing,
                    ItemId::LichBane,
                ],
                runes: Default::default(),
                stacks: Default::default(),
            },
            enemy_players: Vec::from_iter([InputEnemyPlayers {
                champion_id: ChampionId::Gwen,
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
