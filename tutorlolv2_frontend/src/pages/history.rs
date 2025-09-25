use crate::{
    MAX_FAILURES, REFRESH_RATE, RETRY_INTERVAL,
    components::tables::{
        BaseTable,
        cells::{DisplayDamage, ImageCell, Instances},
    },
    external::api::{decode_bytes, send_bytes},
    global_bool,
    models::realtime::Realtime,
    url,
};
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::{
    Html, InputEvent, TargetCast, classes, function_component, html, platform::spawn_local,
    use_effect_with, use_state,
};

#[function_component(History)]
pub fn history() -> Html {
    // let game_code = use_state(|| String::with_capacity(6));
    let game_code = use_state(|| String::from("113680"));
    let game_data = use_state(|| Rc::new(None::<Realtime>));

    {
        let game_data = game_data.clone();
        let game_code = game_code.clone();
        use_effect_with(game_code.clone(), move |_| {
            global_bool!(set HISTORY_LOOP_FLAG, true);
            if (*game_code).len() != 6 {
                return;
            }

            global_bool!(set HISTORY_LOOP_FLAG, false);

            spawn_local(async move {
                let mut failures = 0usize;

                loop {
                    if global_bool!(get HISTORY_LOOP_FLAG) {
                        break;
                    }

                    let response =
                        send_bytes(url!("/api/games/get_by_code"), &*game_code, None).await;

                    if let Some(data) = response {
                        if let Some(req_realtime) = decode_bytes::<Realtime>(data).await {
                            game_data.set(Rc::new(Some(req_realtime)));
                            failures = 0;
                        } else {
                            failures += 1;
                        }
                    };

                    let delay = std::time::Duration::from_millis(if failures > MAX_FAILURES {
                        RETRY_INTERVAL
                    } else {
                        REFRESH_RATE
                    });

                    gloo_timers::future::sleep(delay).await;
                }
            });
        });
    }

    html! {
        <div class={classes!(
            "p-6", "flex-1", "flex", "flex-col", "gap-4",
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
                        "_bg-800", "py-2", "px-4", "rounded-lg",
                        "w-48"
                    )}
                    type={"text"}
                    placeholder={"ABC123"}
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
                    html! {
                        <div>
                            <BaseTable
                                damaging_items={data.current_player.damaging_items.clone()}
                                damaging_runes={data.current_player.damaging_runes.clone()}
                                champion_id={data.current_player.champion_id}
                                damages={
                                    data.enemies
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
                        </div>
                    }
                } else {
                    html!()
                }
            }
        </div>
    }
}
