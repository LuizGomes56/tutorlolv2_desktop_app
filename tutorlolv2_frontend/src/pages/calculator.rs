use crate::{
    components::{
        calculator::*,
        tables::{
            BaseTable,
            cells::{ImageCell, Instances, damage_cells},
        },
    },
    external::api::{decode_bytes, send_bytes},
    models::calculator::{InputGame, OutputCurrentPlayer, OutputEnemy, OutputGame, ReqOutputGame},
    url,
};
use rustc_hash::FxHashSet;
use std::{collections::BTreeMap, rc::Rc};
use web_sys::AbortController;
use yew::{
    AttrValue, Html, classes, function_component, html, platform::spawn_local, use_callback,
    use_effect_with, use_reducer, use_state,
};

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let input_game = use_reducer(InputGame::default);
    let output_game = use_state(|| None::<Rc<OutputGame>>);
    let abort_controller = use_state(|| None::<AbortController>);

    let current_player_champion_id =
        AttrValue::from((*input_game).active_player.champion_id.clone());

    let set_current_player_champion_id = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetCurrentPlayerChampionId(v));
        })
    };
    let insert_current_player_items = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::InsertCurrentPlayerItem(v));
        })
    };
    let remove_current_player_items = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::RemoveCurrentPlayerItem(v));
        })
    };
    let insert_current_player_runes = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::InsertCurrentPlayerRune(v));
        })
    };
    let remove_current_player_runes = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::RemoveCurrentPlayerRune(v));
        })
    };
    let change_ability_level = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetAbilityLevels(v));
        })
    };
    let set_current_player_stats = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetCurrentPlayerStats(v));
        })
    };
    let set_current_player_level = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetCurrentPlayerLevel(v));
        })
    };

    {
        let output_game = output_game.clone();
        let abort_controller = abort_controller.clone();
        use_effect_with(input_game.clone(), move |input_game| {
            let input_game = input_game.clone();
            // web_sys::console::log_1(&format!("{:#?}", *input_game).into());

            if let Some(controller) = &*abort_controller {
                controller.abort();
            }

            let new_controller = AbortController::new().ok();
            let signal = new_controller.as_ref().map(|c| c.signal());
            abort_controller.set(new_controller);

            spawn_local(async move {
                let response =
                    send_bytes(url!("/api/games/calculator"), &*input_game, signal).await;

                if let Ok(res) = response {
                    match decode_bytes::<ReqOutputGame>(res).await {
                        Ok(data) => {
                            if input_game.active_player.infer_stats {
                                input_game.dispatch(InputGameAction::SetCurrentPlayerStats(
                                    ChangeStatsAction::Replace(data.current_player.current_stats),
                                ));
                            }
                            output_game.set(Some(Rc::new(OutputGame {
                                current_player: OutputCurrentPlayer {
                                    champion_id: data.current_player.champion_id,
                                    damaging_abilities: Rc::new(
                                        data.current_player
                                            .damaging_abilities
                                            .into_iter()
                                            .collect(),
                                    ),
                                    damaging_items: data
                                        .current_player
                                        .damaging_items
                                        .into_iter()
                                        .collect(),

                                    damaging_runes: data
                                        .current_player
                                        .damaging_runes
                                        .into_iter()
                                        .collect(),

                                    level: data.current_player.level,
                                    base_stats: data.current_player.base_stats,
                                    bonus_stats: data.current_player.bonus_stats,
                                    current_stats: data.current_player.current_stats,
                                },
                                enemies: data
                                    .enemies
                                    .into_iter()
                                    .map(|(enemy_id, enemy)| {
                                        (
                                            enemy_id,
                                            OutputEnemy {
                                                level: enemy.level,
                                                champion_name: enemy.champion_name,
                                                current_stats: enemy.current_stats,
                                                base_stats: enemy.base_stats,
                                                bonus_stats: enemy.bonus_stats,
                                                real_armor: enemy.real_armor,
                                                real_magic_resist: enemy.real_magic_resist,
                                                damages: Rc::new(enemy.damages),
                                            },
                                        )
                                    })
                                    .collect(),
                                recommended_items: data.recommended_items,
                            })));
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("{:#?}", e).into());
                        }
                    }
                }
            });
        });
    }

    html! {
        <>
            <div class={classes!(
                "h-screen", "overflow-y-auto",
                "gap-4", "grid", "grid-cols-[auto_1fr]",
            )}>
                <div class={classes!(
                    "flex", "flex-col", "gap-4", "w-56"
                )}>
                    <img
                        loading={"lazy"}
                        class={classes!("w-full", "img-clipped", "h-16")}
                        src={url!("/img/centered/{}_0.avif", current_player_champion_id)}
                        alt={""}
                    />
                    <div class={classes!(
                        "grid", "grid-cols-2", "gap-x-2",
                    )}>
                        <AbilitySelector
                            ability_levels={input_game.active_player.abilities}
                            callback={change_ability_level.clone()}
                            current_player_champion_id={&current_player_champion_id}
                        />
                        <ExceptionSelector
                            current_player_champion_id={&current_player_champion_id}
                            input_game={input_game.clone()}
                        />
                    </div>
                    <StatsSelector
                        champion_stats={input_game.active_player.champion_stats}
                        infer_stats={input_game.active_player.infer_stats}
                        set_stats_callback={set_current_player_stats.clone()}
                        set_level_callback={set_current_player_level.clone()}
                        level={input_game.active_player.level}
                    />
                </div>
                <div>
                    {
                        if let Some(output_game) = &*output_game {
                            let hidden_set = FxHashSet::from_iter(["Neeko".to_string()]);

                            let enemies = output_game
                                .enemies
                                .iter()
                                .filter(|(keyname, _)| !hidden_set.contains(*keyname))
                                .map(|(key, val)| (key, val))
                                .collect::<BTreeMap<_, _>>();

                            html! {
                                <BaseTable
                                    damaging_abilities={output_game.current_player.damaging_abilities.clone()}
                                    damaging_items={output_game.current_player.damaging_items.clone()}
                                    damaging_runes={output_game.current_player.damaging_runes.clone()}
                                    champion_id={output_game.current_player.champion_id.clone()}
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
                                                        {damage_cells(enemy.damages.abilities.values())}
                                                        {damage_cells(enemy.damages.items.values())}
                                                        {damage_cells(enemy.damages.runes.values())}
                                                    </tr>
                                                }
                                            })
                                            .collect::<Html>()
                                    }
                                />
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>
            <div class={classes!("hidden")}>
                <ChampionSelector
                    callback={set_current_player_champion_id}
                />
            </div>
            <div class={classes!("hidden")}>
                <StaticSelector
                    static_iter={StaticIterator::Items}
                    iterator={input_game.active_player.items.clone()}
                    insert_callback={insert_current_player_items.clone()}
                    remove_callback={remove_current_player_items.clone()}
                />
            </div>
            <div class={classes!("hidden")}>
                <StaticSelector
                    static_iter={StaticIterator::Runes}
                    iterator={input_game.active_player.runes.clone()}
                    insert_callback={insert_current_player_runes.clone()}
                    remove_callback={remove_current_player_runes.clone()}
                />
            </div>
        </>
    }
}
