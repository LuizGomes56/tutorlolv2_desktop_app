use crate::{
    components::{
        Image, ImageType,
        calculator::*,
        tables::{
            BaseTable,
            cells::{DisplayDamage, ImageCell, Instances},
        },
    },
    external::api::{decode_bytes, send_bytes},
    models::{
        base::DamageType,
        calculator::{InputCurrentPlayer, InputDragons, InputEnemyPlayer, InputGame, OutputGame},
    },
    url,
};
use generated_code::ChampionId;
use web_sys::AbortController;
use yew::{
    AttrValue, Html, classes, function_component, html, platform::spawn_local, use_callback,
    use_effect_with, use_mut_ref, use_reducer, use_state,
};

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let input_current_player = use_reducer(InputCurrentPlayer::new);
    let input_enemy_players = use_reducer(InputEnemies::new);
    let input_dragons = use_reducer(InputDragons::default);
    let cp_infer_flag = use_mut_ref(|| false);

    let output_game = use_state(|| None::<OutputGame>);
    let abort_controller = use_state(|| None::<AbortController>);
    let damage_stack = use_reducer(Stack::default);

    let current_player_champion_id = (*input_current_player).champion_id;

    let set_current_player_champion_id = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::ChampionId(v));
        })
    };
    let insert_current_player_items = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::InsertItem(v));
        })
    };
    let remove_current_player_items = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::RemoveItem(v));
        })
    };
    let insert_current_player_runes = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::InsertRune(v));
        })
    };
    let remove_current_player_runes = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::RemoveRune(v));
        })
    };
    let change_ability_level = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::AbilityLevels(v));
        })
    };
    let set_current_player_stats = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::Stats(v));
        })
    };
    let set_current_player_attack_form = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::AttackForm(v));
        })
    };
    let set_current_player_level = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::Level(v));
        })
    };
    let set_current_player_infer_stats = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::InferStats(v));
        })
    };
    let set_current_player_stacks = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(CurrentPlayerAction::Stacks(v));
        })
    };
    let set_ally_fire_dragons = {
        let input_dragons = input_dragons.clone();
        use_callback((), move |v, _| {
            input_dragons.dispatch(DragonAction::AllyFireDragons(v));
        })
    };
    let set_ally_earth_dragons = {
        let input_dragons = input_dragons.clone();
        use_callback((), move |v, _| {
            input_dragons.dispatch(DragonAction::AllyEarthDragons(v));
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
        let input_current_player = input_current_player.clone();
        let input_enemy_players = input_enemy_players.clone();
        let input_dragons = input_dragons.clone();
        use_effect_with(
            (
                input_current_player.clone(),
                input_enemy_players.clone(),
                input_dragons.clone(),
            ),
            move |_| {
                if *(*cp_infer_flag).borrow() {
                    *(*cp_infer_flag).borrow_mut() = false;
                    return;
                }
                if let Some(controller) = &*abort_controller {
                    controller.abort();
                }

                let new_controller = AbortController::new().ok();
                let signal = new_controller.as_ref().map(|c| c.signal());
                abort_controller.set(new_controller);
                spawn_local(async move {
                    let input_game = InputGame {
                        active_player: &*input_current_player,
                        enemy_players: (*input_enemy_players).as_slice(),
                        ally_earth_dragons: input_dragons.ally_earth_dragons,
                        ally_fire_dragons: input_dragons.ally_fire_dragons,
                        enemy_earth_dragons: input_dragons.enemy_earth_dragons,
                    };

                    web_sys::console::log_1(&format!("{:#?}", input_game).into());

                    let response =
                        send_bytes(url!("/api/games/calculator"), &input_game, signal).await;

                    if let Ok(res) = response {
                        match decode_bytes::<OutputGame>(res).await {
                            Ok(data) => {
                                if input_current_player.infer_stats {
                                    if input_current_player.stats != data.current_player.stats {
                                        *(*cp_infer_flag).borrow_mut() = true;
                                    }
                                    input_current_player.dispatch(CurrentPlayerAction::Stats(
                                        ChangeStatsAction::Replace(
                                            &data.current_player.stats as *const _,
                                        ),
                                    ));
                                }
                                // web_sys::console::log_1(&format!("{:#?}", data).into());
                                output_game.set(Some(data));
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("{:#?}", e).into());
                            }
                        }
                    }
                });
            },
        );
    }

    let make_td = |text: f32, damage_type: DamageType| -> Html {
        html! {
            <td class={classes!{
                "text-center", "text-sm", "px-2", "h-10",
                "max-w-24", "truncate", damage_type.get_color(),
            }}>
                {text.round()}
            </td>
        }
    };

    html! {
        <div>
            <div class={classes!(
                "oxanium", "gap-4", "grid", "grid-cols-[auto_1fr]",
            )}>
                <div class={classes!(
                    "flex", "flex-col", "gap-4", "w-60", "bg-[#141417]"
                )}>
                    <ChampionBanner
                        champion_id={current_player_champion_id}
                    />
                    <div class={classes!(
                        "grid", "grid-cols-2", "gap-2", "px-4"
                    )}>
                        <AbilitySelector
                            ability_levels={input_current_player.abilities}
                            callback={change_ability_level}
                            current_player_champion_id={current_player_champion_id}
                        />
                        <ExceptionSelector
                            current_player_champion_id={current_player_champion_id}
                            attack_form={false}
                            infer_stats={input_current_player.infer_stats}
                            set_ally_fire_dragons={set_ally_fire_dragons}
                            set_ally_earth_dragons={set_ally_earth_dragons}
                            set_current_player_stacks={set_current_player_stacks}
                            set_current_player_infer_stats={set_current_player_infer_stats}
                            set_current_player_attack_form={set_current_player_attack_form}
                        />
                    </div>
                    <StatsSelector
                        champion_stats={input_current_player.stats}
                        infer_stats={input_current_player.infer_stats}
                        set_stats_callback={set_current_player_stats}
                        set_level_callback={set_current_player_level}
                        level={input_current_player.level}
                    />
                </div>
                <div>
                    {
                        if let Some(output_game) = &*output_game {
                            html! {
                                <div class={classes!(
                                    "flex", "flex-col", "gap-6", "py-2"
                                )}>
                                    <BaseTable
                                        damaging_items={output_game.current_player.damaging_items.clone()}
                                        damaging_runes={output_game.current_player.damaging_runes.clone()}
                                        champion_id={current_player_champion_id}
                                        damages={
                                            output_game.enemies
                                                .iter()
                                                .map(|(enemy_champion_id, enemy)| {
                                                    html! {
                                                        <tr>
                                                            <td class={classes!("w-10", "h-10")}>
                                                                <ImageCell instance={Instances::Champions(*enemy_champion_id)} />
                                                            </td>
                                                            {enemy.damages.attacks.display_damage()}
                                                            {enemy.damages.abilities.display_damage()}
                                                            {enemy.damages.items.display_damage()}
                                                            {enemy.damages.runes.display_damage()}
                                                        </tr>
                                                    }
                                                })
                                                .collect::<Html>()
                                        }
                                    />
                                    <DamageStackSelector
                                        items={output_game.current_player.damaging_items.clone()}
                                        runes={output_game.current_player.damaging_runes.clone()}
                                        champion_id={current_player_champion_id}
                                        stack={(*damage_stack).get_owned()}
                                        push_callback={push_stack_callback}
                                        remove_callback={remove_stack_callback}
                                        damages={
                                            output_game.enemies
                                                .iter()
                                                .map(|(enemy_champion_id, enemy)| {
                                                    let mut total_damage = 0.0;
                                                    for value in (*damage_stack).get_ref() {
                                                        match value {
                                                            StackValue::Ability(ability) => {
                                                                if let Some((_, instance_damage)) = enemy.damages.abilities.iter().find(|(a, _)| a == ability) {
                                                                    total_damage += instance_damage.minimum_damage;
                                                                }
                                                            },
                                                            StackValue::BasicAttack => {
                                                                let len = enemy.damages.abilities.len();
                                                                if let Some((_, instance_damage)) = enemy.damages.abilities.get(len - 3) {
                                                                    total_damage += instance_damage.minimum_damage;
                                                                }
                                                            }
                                                            StackValue::CriticalStrike => {
                                                                let len = enemy.damages.abilities.len();
                                                                if let Some((_, instance_damage)) = enemy.damages.abilities.get(len - 2) {
                                                                    total_damage += instance_damage.minimum_damage;
                                                                }
                                                            },
                                                            StackValue::Onhit => {
                                                                let len = enemy.damages.abilities.len();
                                                                if let Some((_, instance_damage)) = enemy.damages.abilities.get(len - 1) {
                                                                    total_damage += instance_damage.minimum_damage;
                                                                }
                                                            },
                                                            StackValue::Item(item_id) => {
                                                                if let Ok(index) = enemy.damages.items.binary_search_by_key(item_id, |(key, _)| *key) {
                                                                    let instance_damage = &enemy.damages.items[index].1;
                                                                    total_damage += instance_damage.minimum_damage;
                                                                }
                                                            },
                                                            StackValue::Rune(rune_id) => {
                                                                if let Ok(index) = enemy.damages.runes.binary_search_by_key(rune_id, |(key, _)| *key) {
                                                                    let instance_damage = &enemy.damages.runes[index].1;
                                                                    total_damage += instance_damage.minimum_damage;
                                                                }
                                                            },
                                                            _ => {},
                                                        }
                                                    }
                                                    html! {
                                                        <tr>
                                                            <td class={classes!("w-10", "h-10")}>
                                                                <ImageCell instance={Instances::Champions(*enemy_champion_id)} />
                                                            </td>
                                                            {make_td(total_damage.round(), DamageType::Mixed)}
                                                            {make_td((enemy.current_stats.health - total_damage).round(), DamageType::Unknown)}
                                                            {make_td((total_damage / enemy.current_stats.health * 100.0).round(), DamageType::True)}
                                                        </tr>
                                                    }
                                                })
                                                .collect::<Html>()
                                        }
                                    />
                                    <TurretTable
                                        damages={
                                            (0..6).into_iter().map(|i| {
                                                html! {
                                                    <td class={classes!(
                                                        "text-center", "text-sm",
                                                        "px-2", output_game.current_player.adaptative_type.get_color(),
                                                        "max-w-24", "truncate", "h-10"
                                                    )}>
                                                        {output_game.tower_damage[i].round()}
                                                    </td>
                                                }
                                            })
                                            .collect::<Html>()
                                        }
                                    />
                                    <BaseTable
                                        empty_headers={4}
                                        damaging_items={output_game.current_player.damaging_items.clone()}
                                        damaging_runes={output_game.current_player.damaging_runes.clone()}
                                        champion_id={current_player_champion_id}
                                        damages={
                                            ([
                                                &[
                                                    url!("/img/other/voidgrubs.avif"),
                                                    url!("/img/other/melee_minion.avif"),
                                                    url!("/img/other/ranged_minion.avif"),
                                                    url!("/img/other/cannon.avif"),
                                                ],
                                                &[url!("/img/other/super_minion.avif")],
                                                &[
                                                    url!("/img/other/elder_dragon.avif"),
                                                    url!("/img/other/fire_dragon.avif"),
                                                    url!("/img/other/ocean_dragon.avif"),
                                                    url!("/img/other/earth_dragon.avif"),
                                                ],
                                                &[url!("/img/other/baron.avif")],
                                                &[url!("/img/other/atakhan.avif")],
                                                &[
                                                    url!("/img/other/red_buff.avif"),
                                                    url!("/img/other/blue_buff.avif"),
                                                    url!("/img/other/gromp.avif"),
                                                    url!("/img/other/wolves.avif"),
                                                ],
                                                &[
                                                    url!("/img/other/krug.avif"),
                                                    url!("/img/other/raptor.avif"),
                                                ],
                                            ] as [&[&'static str]; 7])
                                            .iter()
                                            .enumerate()
                                            .map(|(index, urls)| {
                                                html! {
                                                    <tr>
                                                        {
                                                            for (0..4).map(|i| {
                                                                html!{
                                                                    <td class={classes!("min-w-10","h-10","justify-items-center")}>
                                                                        {
                                                                            if let Some(&icon_url) = urls.get(i) {
                                                                                html! {
                                                                                    <Image
                                                                                        class={classes!("w-8","h-8")}
                                                                                        source={ImageType::Other(AttrValue::Static(icon_url))}
                                                                                    />
                                                                                }
                                                                            } else {
                                                                                html!{}
                                                                            }
                                                                        }
                                                                    </td>
                                                                }
                                                            })
                                                        }
                                                        { output_game.monster_damages.join_td_index(index) }
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
            // <MainSelector
            //     set_current_player_champion_id_callback={set_current_player_champion_id}
            //     insert_item_callback={insert_current_player_items}
            //     remove_item_callback={remove_current_player_items}
            //     insert_rune_callback={insert_current_player_runes}
            //     remove_rune_callback={remove_current_player_runes}
            //     items_iterator={input_current_player.items.clone()}
            //     runes_iterator={input_current_player.runes.clone()}
            //     current_player_champion_id={current_player_champion_id}
            // />
        </div>
    }
}
