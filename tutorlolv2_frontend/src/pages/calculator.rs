use crate::{
    components::{
        calculator::*,
        tables::{
            BaseTable,
            cells::{ImageCell, Instances, damage_cells},
        },
    },
    external::api::{decode_bytes, send_bytes},
    models::calculator::{InputGame, OutputGame},
    url,
    utils::BytesExt,
};
use generated_code::CHAMPION_ABILITIES;
use rustc_hash::FxHashSet;
use web_sys::AbortController;
use yew::{
    AttrValue, Html, classes, function_component, html, platform::spawn_local, use_callback,
    use_effect_with, use_reducer, use_state,
};

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let input_game = use_reducer(InputGame::default);
    let output_game = use_state(|| None::<OutputGame>);
    let abort_controller = use_state(|| None::<AbortController>);
    let damage_stack = use_reducer(Stack::default);

    let current_player_champion_id = AttrValue::Static((*input_game).active_player.champion_id);

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
    let push_stack_callback = {
        let damage_stack = damage_stack.clone();
        use_callback((), move |v, _| {
            damage_stack.dispatch(StackAction::Push(v));
        })
    };
    let remove_stack_callback = {
        let damage_stack = damage_stack.clone();
        use_callback((), move |v, _| {
            damage_stack.dispatch(StackAction::Remove(v));
        })
    };

    use_effect_with(damage_stack.clone(), move |damage_stack| {
        web_sys::console::log_1(&format!("{:#?}", damage_stack.get_owned()).into());
    });

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
                            output_game.set(Some(data));
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
                "h-screen", "overflow-y-auto", "oxanium",
                "gap-4", "grid", "grid-cols-[auto_1fr]",
            )}>
                <div class={classes!(
                    "flex", "flex-col", "gap-4", "w-56", "bg-[#121214]"
                )}>
                    <ChampionBanner
                        champion_id={&current_player_champion_id}
                    />
                    <div class={classes!(
                        "grid", "grid-cols-2", "gap-x-2", "px-4"
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
                                .filter(|(keyname, _)| !hidden_set.contains(keyname))
                                .map(|(key, val)| (key, val))
                                .collect::<Vec<_>>();

                            html! {
                                <div>
                                    <BaseTable
                                        damaging_items={output_game.current_player.damaging_items.clone()}
                                        damaging_runes={output_game.current_player.damaging_runes.clone()}
                                        champion_id={&current_player_champion_id}
                                        damages={
                                            enemies
                                                .iter()
                                                .map(|(enemy_champion_id, enemy)| {
                                                    html! {
                                                        <tr>
                                                            <td class={classes!("w-10", "h-10")}>
                                                                <ImageCell
                                                                    instance={Instances::Champions(
                                                                        AttrValue::from((*enemy_champion_id).clone()),
                                                                    )}
                                                                />
                                                            </td>
                                                            {damage_cells(&enemy.damages.abilities)}
                                                            {damage_cells(&enemy.damages.items)}
                                                            {damage_cells(&enemy.damages.runes )}
                                                        </tr>
                                                    }
                                                })
                                                .collect::<Html>()
                                        }
                                    />
                                    <DamageStackSelector
                                        items={output_game.current_player.damaging_items.clone()}
                                        runes={output_game.current_player.damaging_runes.clone()}
                                        champion_id={&current_player_champion_id}
                                        stack={(*damage_stack).get_owned()}
                                        push_callback={push_stack_callback}
                                        remove_callback={remove_stack_callback}
                                        damages={
                                            enemies
                                                .iter()
                                                .map(|(enemy_champion_id, enemy)| {
                                                    let mut total_damage = 0.0;
                                                    for value in (*damage_stack).get_ref() {
                                                        match value {
                                                            StackValue::Ability(bytes) => {
                                                                let ability_name = bytes.as_str_unchecked();
                                                                if let Some(abilities_phf) = CHAMPION_ABILITIES.get(&current_player_champion_id) {
                                                                    if let Some(index) = abilities_phf.get_index(ability_name) {
                                                                        if let Some((_, instance_damage)) = enemy.damages.abilities.get(index) {
                                                                            total_damage += instance_damage.minimum_damage
                                                                             + instance_damage.maximum_damage;
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            StackValue::BasicAttack => {
                                                                let len = enemy.damages.abilities.len();
                                                                if let Some((_, instance_damage)) = enemy.damages.abilities.get(len - 2) {
                                                                    total_damage += instance_damage.minimum_damage
                                                                     + instance_damage.maximum_damage;
                                                                }
                                                            }
                                                            StackValue::CriticalStrike => {
                                                                let len = enemy.damages.abilities.len();
                                                                if let Some((_, instance_damage)) = enemy.damages.abilities.get(len - 1) {
                                                                    total_damage += instance_damage.minimum_damage
                                                                     + instance_damage.maximum_damage;
                                                                }
                                                            },
                                                            StackValue::Item(item_id) => {
                                                                if let Ok(index) = enemy.damages.items.binary_search_by_key(item_id, |(key, _)| *key) {
                                                                    let instance_damage = &enemy.damages.items[index].1;
                                                                    total_damage += instance_damage.minimum_damage + instance_damage.maximum_damage;
                                                                }
                                                            },
                                                            StackValue::Rune(rune_id) => {
                                                                if let Ok(index) = enemy.damages.runes.binary_search_by_key(rune_id, |(key, _)| *key) {
                                                                    let instance_damage = &enemy.damages.runes[index].1;
                                                                    total_damage += instance_damage.minimum_damage + instance_damage.maximum_damage;
                                                                }
                                                            },
                                                            _ => {},
                                                        }
                                                    }
                                                    let make_td = |text| -> Html {
                                                        html! {
                                                            <td class={classes!{
                                                                "text-center", "text-sm", "px-2",
                                                                "max-w-24", "truncate", "text-violet-500",
                                                            }}>
                                                                {text}
                                                            </td>
                                                        }
                                                    };
                                                    html! {
                                                        <tr>
                                                            <td class={classes!("w-10", "h-10")}>
                                                                <ImageCell
                                                                    instance={Instances::Champions(
                                                                        AttrValue::from((*enemy_champion_id).clone()),
                                                                    )}
                                                                />
                                                            </td>
                                                            {make_td(total_damage.round())}
                                                            {make_td((enemy.current_stats.health - total_damage).round())}
                                                            {make_td((total_damage / enemy.current_stats.health * 100.0).round())}
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
            <MainSelector
                set_current_player_champion_id_callback={set_current_player_champion_id}
                insert_item_callback={insert_current_player_items}
                remove_item_callback={remove_current_player_items}
                insert_rune_callback={insert_current_player_runes}
                remove_rune_callback={remove_current_player_runes}
                items_iterator={input_game.active_player.items.clone()}
                runes_iterator={input_game.active_player.runes.clone()}
                current_player_champion_id={current_player_champion_id}
            />
        </>
    }
}
