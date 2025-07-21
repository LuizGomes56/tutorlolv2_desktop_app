use crate::{
    STATIC_COMPARED_ITEMS,
    components::{
        calculator::*,
        hover::item_stats::ItemStatsHover,
        tables::{
            BaseTable,
            cells::{ImageCell, Instances, damage_cells},
        },
    },
    external::api::{decode_bytes, send_bytes},
    macros::STATS_URL,
    models::calculator::{InputGame, OutputGame},
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
    let set_current_player_attack_form = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetCurrentPlayerAttackForm(v));
        })
    };
    let set_current_player_level = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetCurrentPlayerLevel(v));
        })
    };
    let set_ally_fire_dragons = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetAllyFireDragons(v));
        })
    };
    let set_ally_earth_dragons = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetAllyEarthDragons(v));
        })
    };
    let set_current_player_stacks = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetCurrentPlayerStacks(v));
        })
    };
    let set_current_player_infer_stats = {
        let input_game = input_game.clone();
        use_callback((), move |v, _| {
            input_game.dispatch(InputGameAction::SetCurrentPlayerInferStats(v));
        })
    };

    {
        let output_game = output_game.clone();
        let abort_controller = abort_controller.clone();
        let input_game = input_game.clone();
        use_effect_with(input_game.clone(), move |_| {
            web_sys::console::log_1(&format!("{:#?}", *input_game).into());

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
                    match decode_bytes::<OutputGame>(res).await {
                        Ok(data) => {
                            if input_game.active_player.infer_stats {
                                input_game.dispatch(InputGameAction::SetCurrentPlayerStats(
                                    ChangeStatsAction::Replace(data.current_player.current_stats),
                                ));
                            }
                            output_game.set(Some(Rc::new(data)));
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
                    <ChampionBanner
                        champion_id={&current_player_champion_id}
                        set_callback={set_current_player_champion_id}
                    />
                    <div class={classes!(
                        "grid", "grid-cols-2", "gap-x-2",
                        "px-4"
                    )}>
                        <AbilitySelector
                            ability_levels={input_game.active_player.abilities}
                            callback={change_ability_level}
                            current_player_champion_id={&current_player_champion_id}
                        />
                        <ExceptionSelector
                            set_ally_fire_dragons={set_ally_fire_dragons}
                            set_ally_earth_dragons={set_ally_earth_dragons}
                            set_current_player_stacks={set_current_player_stacks}
                            set_current_player_infer_stats={set_current_player_infer_stats}
                            set_current_player_attack_form={set_current_player_attack_form}
                            current_player_champion_id={&current_player_champion_id}
                        />
                    </div>
                    <StatsSelector
                        champion_stats={input_game.active_player.champion_stats}
                        infer_stats={input_game.active_player.infer_stats}
                        set_stats_callback={set_current_player_stats}
                        set_level_callback={set_current_player_level}
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
                                <div>
                                    <ItemStatsHover item_id={4645} />
                                    <ItemStatsHover item_id={224403} />
                                    <BaseTable
                                        damaging_abilities={output_game.current_player.damaging_abilities.clone()}
                                        damaging_items={output_game.current_player.damaging_items.clone()}
                                        damaging_runes={output_game.current_player.damaging_runes.clone()}
                                        champion_id={&current_player_champion_id}
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
                                                                            AttrValue::from((*enemy_champion_id).clone()),
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
                            html! {}
                        }
                    }
                </div>
            </div>
            <div class={classes!("hidden")}>
                <StaticSelector
                    static_iter={StaticIterator::Items}
                    iterator={input_game.active_player.items.clone()}
                    insert_callback={insert_current_player_items}
                    remove_callback={remove_current_player_items}
                />
            </div>
            <div class={classes!("hidden")}>
                <StaticSelector
                    static_iter={StaticIterator::Runes}
                    iterator={input_game.active_player.runes.clone()}
                    insert_callback={insert_current_player_runes}
                    remove_callback={remove_current_player_runes}
                />
            </div>
        </>
    }
}
