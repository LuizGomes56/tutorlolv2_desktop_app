use crate::{
    MAX_FAILURES, REFRESH_RATE, RETRY_INTERVAL,
    components::tables::{
        BaseTable,
        cells::{ImageCell, Instances, damage_cells},
    },
    external::api::{decode_bytes, send_bytes},
    loop_flag,
    models::realtime::{CurrentPlayer, Enemy, Realtime, ReqRealtime},
    url,
};
use rustc_hash::FxHashSet;
use serde::Serialize;
use std::{collections::BTreeMap, rc::Rc};
use web_sys::{HtmlInputElement, console};
use yew::{
    AttrValue, Html, InputEvent, TargetCast, classes, function_component, html,
    platform::spawn_local, use_effect_with, use_state,
};

/*
{
    for enemies.iter().map(|(enemy_champion_id, enemy)| {
        STATIC_COMPARED_ITEMS
            .get()
            .and_then(|compared_items| {
                compared_items.iter().map(|(siml_item_id, compared_item)| {
                    let mut sum = 0.0;
                    for (ability_id, ability) in enemy.damages.abilities.iter() {
                        sum += enemy.damages.compared_items
                            .get(&siml_item_id)
                            .and_then(|siml_items| {
                                siml_items.abilities.get(ability_id)
                            })
                            .map(|siml| {
                                siml.minimum_damage
                                    + siml.maximum_damage
                                    - ability.minimum_damage
                                    - ability.maximum_damage
                            })
                            .unwrap_or(0.0);
                    }
                    Some(
                        html! {
                            <div class={classes!(
                                "flex", "gap-2", "items-center",
                                "text-white",
                            )}>
                                <div class={classes!(
                                    "flex", "gap-2", "items-center",
                                )}>
                                    <img
                                        loading={"lazy"}
                                        class={classes!("h-7", "w-7")}
                                        src={url!("/img/champions/{}.avif", enemy_champion_id)}
                                        alt={""}
                                    />
                                    <span>
                                        { enemy_champion_id }
                                    </span>
                                </div>
                                <div class={classes!(
                                    "flex", "gap-2", "items-center",
                                )}>
                                    <img
                                        loading={"lazy"}
                                        class={classes!("h-7", "w-7")}
                                        src={url!("/img/items/{}.avif", siml_item_id)}
                                        alt={""}
                                    />
                                    <span>
                                        { &compared_item.name }
                                    </span>
                                </div>
                                { sum }
                            </div>
                        }
                    )
                })
                .collect::<Option<Html>>()
            })
            .unwrap_or_else(|| html! {
                <div>
                    {"Erro ao carregar itens comparados."}
                </div>
            })
    })
}
*/

#[function_component(History)]
pub fn history() -> Html {
    // let game_code = use_state(|| String::with_capacity(6));
    let game_code = use_state(|| String::from("113680"));
    let game_data = use_state(|| Rc::new(None::<Realtime>));

    {
        let game_data = game_data.clone();
        let game_code = game_code.clone();
        use_effect_with(game_code.clone(), move |_| {
            loop_flag!(history true);
            if (*game_code).len() != 6 {
                return;
            }

            loop_flag!(history false);

            spawn_local(async move {
                let mut failures = 0usize;

                loop {
                    if loop_flag!(history) {
                        break;
                    }

                    #[derive(Serialize)]
                    struct GetByCodeBody<'a> {
                        game_code: &'a str,
                    }

                    let response = send_bytes(
                        url!("/api/games/get_by_code"),
                        &GetByCodeBody {
                            game_code: &*game_code,
                        },
                        None,
                    )
                    .await;

                    if let Ok(data) = response {
                        match decode_bytes::<ReqRealtime>(data).await {
                            Ok(req_realtime) => {
                                game_data.set(Rc::new(Some(Realtime {
                                    current_player: CurrentPlayer {
                                        damaging_abilities: req_realtime
                                            .current_player
                                            .damaging_abilities
                                            .into_iter()
                                            .collect(),
                                        damaging_items: req_realtime
                                            .current_player
                                            .damaging_items
                                            .into_iter()
                                            .collect(),

                                        damaging_runes: req_realtime
                                            .current_player
                                            .damaging_runes
                                            .into_iter()
                                            .collect(),
                                        riot_id: req_realtime.current_player.riot_id,
                                        level: req_realtime.current_player.level,
                                        team: req_realtime.current_player.team,
                                        champion_id: req_realtime.current_player.champion_id,
                                        champion_name: req_realtime.current_player.champion_name,
                                        current_stats: req_realtime.current_player.current_stats,
                                        base_stats: req_realtime.current_player.base_stats,
                                        bonus_stats: req_realtime.current_player.bonus_stats,
                                        position: req_realtime.current_player.position,
                                    },
                                    enemies: req_realtime
                                        .enemies
                                        .into_iter()
                                        .map(|(enemy_id, enemy)| {
                                            (
                                                AttrValue::from(enemy_id),
                                                Enemy {
                                                    riot_id: enemy.riot_id,
                                                    level: enemy.level,
                                                    team: enemy.team,
                                                    champion_name: enemy.champion_name,
                                                    current_stats: enemy.current_stats,
                                                    base_stats: enemy.base_stats,
                                                    bonus_stats: enemy.bonus_stats,
                                                    position: enemy.position,
                                                    real_armor: enemy.real_armor,
                                                    real_magic_resist: enemy.real_magic_resist,
                                                    damages: Rc::new(enemy.damages),
                                                },
                                            )
                                        })
                                        .collect(),
                                    game_information: req_realtime.game_information,
                                    scoreboard: req_realtime.scoreboard,
                                    ally_dragon_multipliers: req_realtime.ally_dragon_multipliers,
                                    enemy_dragon_multipliers: req_realtime.enemy_dragon_multipliers,
                                    recommended_items: req_realtime.recommended_items,
                                })));
                                failures = 0;
                            }
                            Err(e) => {
                                console::log_1(&e.to_string().into());
                                failures += 1;
                            }
                        }
                    };

                    let delay = if failures > MAX_FAILURES {
                        std::time::Duration::from_secs(RETRY_INTERVAL)
                    } else {
                        std::time::Duration::from_millis(REFRESH_RATE)
                    };

                    gloo_timers::future::sleep(delay).await;
                }
            });
        });
    }

    html! {
        <div class={classes!(
            "p-6", "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <h1 class={classes!(
                "font-semibold", "text-2xl", "text-white"
            )}>
                { "History" }
            </h1>
            <div class={classes!(
                "flex", "flex-col", "gap-2",
                "text-white"
            )}>
                <span>{
                    "If you are willing to see your friend's game information, or a past one of yours,
                    enter the game code in the box below."
                }</span>
                <input
                    class={classes!(
                        "bg-zinc-800", "py-2", "px-4", "rounded-lg",
                        "w-48"
                    )}
                    type="text"
                    placeholder="ABC123"
                    oninput={{
                        let game_code = game_code.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let value = input.value();
                            game_code.set(value);
                        }
                    }}
                />
            </div>
            {
                if let Some(ref data) = **game_data {
                    let hidden_set = FxHashSet::from_iter([AttrValue::from("Neeko")]);

                    let enemies = data
                        .enemies
                        .iter()
                        .filter(|(keyname, _)| !hidden_set.contains(*keyname))
                        .map(|(key, val)| (key, val))
                        .collect::<BTreeMap<_, _>>();

                    html! {
                        <div>
                            <BaseTable
                                damaging_abilities={data.current_player.damaging_abilities.clone()}
                                damaging_items={data.current_player.damaging_items.clone()}
                                damaging_runes={data.current_player.damaging_runes.clone()}
                                champion_id={data.current_player.champion_id.clone()}
                                damages={
                                    enemies
                                        .iter()
                                        .map(|(enemy_champion_id, enemy)| {
                                            html! {
                                                <tr class={classes!(
                                                    // color!(odd:bg-900), color!(even:bg-800)
                                                )}>
                                                    <td class={classes!("w-10", "h-10")}>
                                                        <ImageCell
                                                            instance={
                                                                Instances::Champions(
                                                                    (*enemy_champion_id).clone(),
                                                                )
                                                            }
                                                        />
                                                    </td>
                                                    {damage_cells(&enemy.damages.abilities)}
                                                    {damage_cells(&enemy.damages.items)}
                                                    {damage_cells(&enemy.damages.runes)}
                                                </tr>
                                            }
                                        })
                                        .collect::<Html>()
                                }
                            />
                        </div>
                    }
                } else {
                    html!()
                }
            }
        </div>
    }
}
