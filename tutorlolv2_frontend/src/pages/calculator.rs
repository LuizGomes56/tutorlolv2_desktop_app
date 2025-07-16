use crate::{
    components::calculator::*,
    external::api::{decode_bytes, send_bytes},
    models::calculator::{InputActivePlayer, InputEnemyPlayers, InputGame, OutputGame},
    url,
};
use std::{cell::RefCell, rc::Rc};
use yew::{
    Html, classes, function_component, html, platform::spawn_local, use_effect_with, use_state,
};

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let current_player_volatile_attrs = use_state(CurrentPlayerVolatileAttrs::default);
    let enemy_players_volatile_attrs = use_state(Vec::<EnemyPlayerVolatileAttrs>::new);
    let outer_volatile_attrs = use_state(OuterVolatileAttrs::default);
    let dangerous_attrs = use_state(|| (Rc::new(RefCell::new(DangerousAttrs::default())), 0));
    let output_game = use_state(|| None::<Rc<OutputGame>>);

    {
        let current_player_volatile_attrs = current_player_volatile_attrs.clone();
        let enemy_players_volatile_attrs = enemy_players_volatile_attrs.clone();
        let outer_volatile_attrs = outer_volatile_attrs.clone();
        let dangerous_attrs = dangerous_attrs.clone();
        let output_game = output_game.clone();
        use_effect_with(
            (
                current_player_volatile_attrs,
                enemy_players_volatile_attrs,
                outer_volatile_attrs,
                dangerous_attrs,
            ),
            move |(
                current_player_volatile_attrs,
                enemy_players_volatile_attrs,
                outer_volatile_attrs,
                dangerous_attrs,
            )| {
                let dangerous_values = dangerous_attrs.get();
                let input_game = InputGame {
                    active_player: InputActivePlayer {
                        champion_id: dangerous_values.current_player_champion_id.clone(),
                        abilities: current_player_volatile_attrs.abilities,
                        level: current_player_volatile_attrs.level,
                        stacks: current_player_volatile_attrs.stacks,
                        infer_stats: current_player_volatile_attrs.infer_stats,
                        champion_stats: current_player_volatile_attrs.champion_stats,
                        items: dangerous_values.current_player_items.clone(),
                        runes: dangerous_values.current_player_runes.clone(),
                    },
                    enemy_players: enemy_players_volatile_attrs
                        .iter()
                        .enumerate()
                        .map(|(i, enemy_attr)| InputEnemyPlayers {
                            champion_name: dangerous_values
                                .enemy_champion_names
                                .get(i)
                                .unwrap_or(&String::new())
                                .clone(),
                            items: dangerous_values
                                .enemy_items
                                .get(i)
                                .unwrap_or(&Vec::new())
                                .clone(),
                            infer_stats: enemy_attr.infer_stats,
                            level: enemy_attr.level,
                            stats: enemy_attr.stats,
                        })
                        .collect(),
                    ally_earth_dragons: outer_volatile_attrs.ally_earth_dragons,
                    enemy_earth_dragons: outer_volatile_attrs.enemy_earth_dragons,
                    ally_fire_dragons: outer_volatile_attrs.ally_fire_dragons,
                    stack_exceptions: Default::default(),
                };

                spawn_local(async move {
                    let response = send_bytes(url!("/api/games/calculator"), &input_game).await;

                    if let Ok(res) = response {
                        match decode_bytes::<OutputGame>(res).await {
                            Ok(data) => {
                                output_game.set(Some(Rc::new(data)));
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

    html! {
        <div class={classes!(
            "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            // <div class={classes!(
            //     "flex", "flex-col", "gap-4", "w-56"
            // )}>
            //     <img
            //         loading={"lazy"}
            //         class={classes!("w-full", "img-clipped", "h-16")}
            //         src={url!("/img/centered/{}_0.avif", input_game.get().active_player.champion_id)}
            //         alt={""}
            //     />
            //     <div class={classes!(
            //         "grid", "grid-cols-2", "gap-x-2",
            //     )}>
            //         <AbilitySelector input_game={input_game.clone()} />
            //         <ExceptionSelector input_game={input_game.clone()} />
            //     </div>
            //     <ItemSelector input_game={input_game.clone()} />
            //     <RuneSelector input_game={input_game.clone()} />
            //     <StatsSelector input_game={input_game.clone()} />
            // </div>
            // <div>
            //     {
            //         if let Some(output_game) = &*output_game {
            //             html! {
            //                 <div class={classes!(
            //                     "text-white", "text-xl"
            //                 )}>
            //                     {output_game.current_player.current_stats.armor}
            //                 </div>
            //             }
            //         } else {
            //             html! {}
            //         }
            //     }
            // </div>
        </div>
    }
}
