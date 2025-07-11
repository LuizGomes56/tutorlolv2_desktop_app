use crate::{
    components::tables::BaseTable,
    models::{
        base::{ApiError, Damages},
        realtime::{CurrentPlayer, Enemy, Realtime, ReqRealtime},
    },
    url,
};
use rustc_hash::FxHashSet;
use std::{
    collections::BTreeMap,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};
use web_sys::{HtmlInputElement, console};
use yew::{
    Html, InputEvent, TargetCast, classes, function_component, html, platform::spawn_local,
    use_effect_with, use_state, use_state_eq,
};

async fn fetch_game_by_code(game_code: &str) -> Result<ReqRealtime, Box<dyn std::error::Error>> {
    console::log_1(&format!("Fetching game, Code: {}", game_code).into());
    let response = reqwasm::http::Request::post(url!("/api/games/get_by_code"))
        .body(format!("{{\"game_code\":\"{}\"}}", game_code))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let bytes = response.binary().await?;

    match bincode::serde::decode_from_slice::<ReqRealtime, _>(&bytes, bincode::config::standard()) {
        Ok(decoded) => Ok(decoded.0),
        Err(e) => {
            let api_error = bincode::serde::decode_from_slice::<ApiError, _>(
                &bytes,
                bincode::config::standard(),
            )?;

            Err(format!(
                "API returned error: {}, error: {:#?}",
                api_error.0.message, e
            )
            .into())
        }
    }
}

static LOOP_FLAG: AtomicBool = AtomicBool::new(false);

macro_rules! loop_flag {
    ($boolean:literal) => {
        LOOP_FLAG.store($boolean, Ordering::SeqCst);
    };
    () => {
        LOOP_FLAG.load(Ordering::SeqCst)
    };
}

#[function_component(History)]
pub fn history() -> Html {
    // let game_code = use_state(|| String::with_capacity(6));
    let game_code = use_state(|| String::from("113680"));
    let game_data = use_state_eq(|| Rc::new(None::<Realtime>));

    {
        let game_data = game_data.clone();
        let game_code = game_code.clone();
        use_effect_with(game_code.clone(), move |_| {
            loop_flag!(true);
            if (*game_code).len() != 6 {
                return;
            }

            loop_flag!(false);

            spawn_local(async move {
                let mut failures = 0usize;

                loop {
                    if loop_flag!() {
                        break;
                    }

                    match fetch_game_by_code(&(*game_code)).await {
                        Ok(response) => {
                            game_data.set(Rc::new(Some(Realtime {
                                current_player: CurrentPlayer {
                                    damaging_abilities: Rc::new(
                                        response.current_player.damaging_abilities,
                                    ),
                                    damaging_items: Rc::new(response.current_player.damaging_items),
                                    damaging_runes: Rc::new(response.current_player.damaging_runes),
                                    riot_id: response.current_player.riot_id,
                                    level: response.current_player.level,
                                    team: response.current_player.team,
                                    champion_id: response.current_player.champion_id,
                                    champion_name: response.current_player.champion_name,
                                    current_stats: response.current_player.current_stats,
                                    base_stats: response.current_player.base_stats,
                                    bonus_stats: response.current_player.bonus_stats,
                                    position: response.current_player.position,
                                },
                                enemies: response
                                    .enemies
                                    .into_iter()
                                    .map(|(enemy_id, enemy)| {
                                        (
                                            enemy_id,
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
                                game_information: response.game_information,
                                scoreboard: response.scoreboard,
                                ally_dragon_multipliers: response.ally_dragon_multipliers,
                                enemy_dragon_multipliers: response.enemy_dragon_multipliers,
                                recommended_items: response.recommended_items,
                            })));
                            failures = 0;
                        }
                        Err(e) => {
                            console::log_1(&e.to_string().into());
                            failures += 1;
                        }
                    }

                    let delay = if failures > 10 {
                        std::time::Duration::from_secs(60)
                    } else {
                        std::time::Duration::from_secs(1)
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
                    let hidden_set = vec!["Ashe", "Rakan", "Nasus"]
                        .iter()
                        .map(|val| val.to_string())
                        .collect::<FxHashSet<String>>();

                    let enemies = data
                        .enemies
                        .iter()
                        .filter(|(keyname, _)| !hidden_set.contains(*keyname))
                        .map(|(key, val)| (key, val))
                        .collect::<BTreeMap<_, _>>();

                    html! {
                        <BaseTable
                            damaging_abilities={data.current_player.damaging_abilities.clone()}
                            damaging_items={data.current_player.damaging_items.clone()}
                            damaging_runes={data.current_player.damaging_runes.clone()}
                            champion_id={data.current_player.champion_id.clone()}
                            damages={
                                enemies
                                    .iter()
                                    .map(|(enemy_champion_id, enemy)| {
                                        ((*enemy_champion_id).clone(), enemy.damages.clone())
                                    })
                                    .collect::<BTreeMap<String, Rc<Damages>>>()
                            }
                        />
                    }
                } else {
                    html!()
                }
            }
        </div>
    }
}
