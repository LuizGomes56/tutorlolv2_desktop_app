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
        calculator::{InputCurrentPlayer, InputDragons, InputGame, OutputGame},
    },
    url,
};
use generated_code::{ItemId, RuneId};
use web_sys::AbortController;
use yew::{
    AttrValue, Html, classes, function_component, html, platform::spawn_local, use_callback,
    use_effect_with, use_mut_ref, use_reducer, use_state,
};

#[derive(PartialEq, Clone, Copy)]
enum ActionTracker {
    Init,
    Any,
    CurrentPlayer,
    EnemyPlayer(usize),
    Replace,
}

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let input_current_player = use_reducer(InputCurrentPlayer::new);
    let input_enemy_players = use_reducer(InputEnemies::new);
    let input_enemy_index = use_state(|| 0);
    let input_dragons = use_reducer(InputDragons::default);

    let output_game = use_state(|| None::<OutputGame>);
    let abort_controller = use_state(|| None::<AbortController>);
    let damage_stack = use_reducer(Stack::default);
    let action_tracker = use_mut_ref(|| ActionTracker::Init);

    let current_player_champion_id = (*input_current_player).champion_id;

    let set_current_player_champion_id = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(CurrentPlayerAction::ChampionId(v));
        })
    };
    let insert_current_player_items = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(CurrentPlayerAction::InsertItem(v));
        })
    };
    let remove_current_player_items = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
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
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(CurrentPlayerAction::Stats(v));
        })
    };
    let set_current_player_attack_form = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(CurrentPlayerAction::AttackForm(v));
        })
    };
    let set_current_player_level = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(CurrentPlayerAction::Level(v));
        })
    };
    let set_current_player_infer_stats = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(CurrentPlayerAction::InferStats(v));
        })
    };
    let set_current_player_stacks = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(CurrentPlayerAction::Stacks(v));
        })
    };
    let set_ally_fire_dragons = {
        let input_dragons = input_dragons.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_dragons.dispatch(DragonAction::AllyFireDragons(v));
        })
    };
    let set_ally_earth_dragons = {
        let input_dragons = input_dragons.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_dragons.dispatch(DragonAction::AllyEarthDragons(v));
        })
    };
    let set_enemy_earth_dragons = {
        let input_dragons = input_dragons.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_dragons.dispatch(DragonAction::EnemyEarthDragons(v));
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
    let set_enemy_champion_id = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::ChampionId(v),
            ));
        })
    };
    let set_enemy_stacks = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::Stacks(v),
            ));
        })
    };
    let set_enemy_infer_stats = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::InferStats(v),
            ));
        })
    };
    let set_enemy_attack_form = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::AttackForm(v),
            ));
        })
    };
    let set_enemy_stats = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::Stats(v),
            ));
        })
    };
    let set_enemy_level = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::Level(v),
            ));
        })
    };
    let insert_enemy_player_items = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::InsertItem(v),
            ));
        })
    };
    let remove_enemy_player_items = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemiesAction::Edit(
                input_enemy_index,
                InputEnemyAction::RemoveItem(v),
            ));
        })
    };

    // use_effect_with(damage_stack.clone(), move |damage_stack| {
    //     web_sys::console::log_1(&format!("{:#?}", damage_stack.clone_inner()).into());
    // });

    {
        let output_game = output_game.clone();
        let abort_controller = abort_controller.clone();
        let input_current_player = input_current_player.clone();
        let input_enemy_players = input_enemy_players.clone();
        let input_dragons = input_dragons.clone();
        let action_tracker = action_tracker.clone();
        use_effect_with(
            (
                input_current_player.clone(),
                input_enemy_players.clone(),
                input_dragons.clone(),
            ),
            move |_| {
                let current_action = *action_tracker.borrow();
                match current_action {
                    ActionTracker::Replace => {
                        action_tracker.replace(ActionTracker::Any);
                        return;
                    }
                    _ => {}
                };

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
                        stack_exceptions: vec![],
                    };

                    // web_sys::console::log_1(&format!("{:#?}", input_game).into());

                    let response =
                        send_bytes(url!("/api/games/calculator"), &input_game, signal).await;

                    if let Some(res) = response {
                        if let Some(data) = decode_bytes::<OutputGame>(res).await {
                            let last_action = *action_tracker.borrow();
                            let mut action_ref_mut = action_tracker.borrow_mut();
                            macro_rules! infer_current_player_stats {
                                () => {
                                    if input_current_player.infer_stats {
                                        *action_ref_mut = ActionTracker::Replace;
                                        input_current_player.dispatch(CurrentPlayerAction::Stats(
                                            ChangeStatsAction::Replace(
                                                &data.current_player.stats as *const _,
                                            ),
                                        ));
                                    }
                                };
                            }
                            macro_rules! infer_enemy_player_stats {
                                ($index:expr) => {
                                    if input_enemy_players.as_slice()[$index].infer_stats {
                                        *action_ref_mut = ActionTracker::Replace;
                                        input_enemy_players.dispatch(EnemiesAction::Edit(
                                            $index,
                                            InputEnemyAction::Stats(
                                                ChangeBasicStatsAction::Replace(
                                                    &data.enemies[$index].1.current_stats
                                                        as *const _,
                                                ),
                                            ),
                                        ));
                                    }
                                };
                            }
                            match last_action {
                                ActionTracker::Init => {
                                    infer_current_player_stats!();
                                    for i in 0..data.enemies.len() {
                                        infer_enemy_player_stats!(i);
                                    }
                                }
                                ActionTracker::CurrentPlayer => {
                                    infer_current_player_stats!();
                                }
                                ActionTracker::EnemyPlayer(index) => {
                                    infer_enemy_player_stats!(index);
                                }
                                _ => {}
                            };
                            // web_sys::console::log_1(&format!("{:#?}", data).into());
                            output_game.set(Some(data));
                        }
                    }
                });
            },
        );
    }

    let make_td = |text: i32, damage_type: DamageType| -> Html {
        html! {
            <td class={classes!{
                "text-center", "text-sm", "px-2", "h-10",
                "max-w-24", "truncate", damage_type.get_color(),
            }}>
                {text}
            </td>
        }
    };

    html! {
        <div class={classes!(
            "oxanium", "gap-4", "grid", "grid-cols-[auto_1fr_auto]", "w-full"
        )}>
            <div class={classes!(
                "flex", "flex-col", "gap-2", "w-60", "bg-[#141417]",
                "h-screen"
            )}>
                <ChampionBanner
                    callback={set_current_player_champion_id}
                    champion_id={current_player_champion_id}
                />
                <div class={classes!("grid", "grid-cols-4", "gap-2", "px-4")}>
                    <AbilitySelector
                        ability_levels={input_current_player.abilities}
                        callback={change_ability_level}
                        current_player_champion_id={current_player_champion_id}
                    />
                    <NumericField<u8>
                        title={"Number of ally fire dragons"}
                        source={Exception::Image}
                        img_url={url!("/img/other/fire_soul.avif")}
                        callback={set_ally_fire_dragons}
                    />
                    <NumericField<u8>
                        title={"Number of ally earth dragons"}
                        source={Exception::Image}
                        img_url={url!("/img/other/earth_soul.avif")}
                        callback={set_ally_earth_dragons}
                    />
                    <ExceptionSelector
                        champion_id={current_player_champion_id}
                        attack_form={false}
                        infer_stats={input_current_player.infer_stats}
                        stack_callback={set_current_player_stacks}
                        infer_stats_callback={set_current_player_infer_stats}
                        attack_form_callback={set_current_player_attack_form}
                    />
                </div>
                <OpenTray<ItemId>
                    insert_callback={insert_current_player_items}
                    title={"Search items"}
                />
                <Tray<ItemId>
                    array={input_current_player.items.clone()}
                    remove_callback={remove_current_player_items}
                />
                <OpenTray<RuneId>
                    insert_callback={insert_current_player_runes}
                    title={"Search runes"}
                />
                <Tray<RuneId>
                    array={input_current_player.runes.clone()}
                    remove_callback={remove_current_player_runes}
                />
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
                                    stack={(*damage_stack).clone_inner()}
                                    push_callback={push_stack_callback}
                                    remove_callback={remove_stack_callback}
                                    damages={
                                        output_game.enemies
                                            .iter()
                                            .map(|(enemy_champion_id, enemy)| {
                                                let mut total_damage = 0;
                                                for value in (*damage_stack).get_ref() {
                                                    match value {
                                                        StackValue::Ability(ability) => {
                                                            if let Some((_, instance_damage)) = enemy.damages.abilities.iter().find(|(a, _)| a == ability) {
                                                                total_damage += instance_damage.minimum_damage;
                                                            }
                                                        },
                                                        StackValue::BasicAttack => {
                                                            total_damage += enemy.damages.attacks.basic_attack.minimum_damage;
                                                        }
                                                        StackValue::CriticalStrike => {
                                                            total_damage += enemy.damages.attacks.critical_strike.minimum_damage;
                                                        },
                                                        StackValue::Onhit => {
                                                            total_damage += enemy.damages.attacks.onhit_damage.minimum_damage;
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
                                                        {make_td(total_damage, DamageType::Mixed)}
                                                        {make_td(enemy.current_stats.health - total_damage, DamageType::Unknown)}
                                                        {make_td(total_damage / enemy.current_stats.health * 100, DamageType::True)}
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
                                                    {output_game.tower_damage[i]}
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
                                                                <td class={classes!("h-10", "justify-items-center")}>
                                                                    {
                                                                        if let Some(&icon_url) = urls.get(i) {
                                                                            html! {
                                                                                <Image
                                                                                    class={classes!("w-8", "h-8")}
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
            <div class={classes!(
                "flex", "flex-col", "gap-2", "w-60", "bg-[#141417]",
                "h-screen",
            )}>
                {input_enemy_players.as_slice().get(*input_enemy_index).and_then(|input_enemy| {
                    Some(html! {
                        <>
                            <ChampionBanner
                                callback={set_enemy_champion_id}
                                champion_id={input_enemy.champion_id}
                                translate_left={true}
                            />
                            <div class={classes!("grid", "grid-cols-4", "gap-2", "px-4")}>
                                <NumericField<u8>
                                    title={"Number of enemy earth dragons"}
                                    source={Exception::Image}
                                    img_url={url!("/img/other/earth_soul.avif")}
                                    callback={set_enemy_earth_dragons}
                                />
                                <ExceptionSelector
                                    champion_id={input_enemy.champion_id}
                                    attack_form={false}
                                    infer_stats={input_enemy.infer_stats}
                                    stack_callback={set_enemy_stacks}
                                    infer_stats_callback={set_enemy_infer_stats}
                                    attack_form_callback={set_enemy_attack_form}
                                />
                            </div>
                            <BasicStatsSelector
                                champion_stats={input_enemy.stats}
                                infer_stats={input_enemy.infer_stats}
                                set_stats_callback={set_enemy_stats}
                                set_level_callback={set_enemy_level}
                                level={input_enemy.level}
                            />
                            <OpenTray<ItemId>
                                insert_callback={insert_enemy_player_items}
                                title={"Search items"}
                            />
                            <Tray<ItemId>
                                array={input_enemy.items.clone()}
                                remove_callback={remove_enemy_player_items}
                                translate_left={true}
                            />
                        </>
                    })
                })}
            </div>
        </div>
    }
}
