use crate::{
    MAX_FAILURES, REFRESH_RATE, RETRY_INTERVAL,
    components::tables::{
        BaseTable,
        cells::{DisplayDamage, ImageCell, Instances},
    },
    external::invoke::{invoke_get_live_game, take_live_game},
    global_bool,
    models::realtime::Realtime,
};
use web_sys::console;
use yew::{
    Html, classes, function_component, html, platform::spawn_local, use_effect_with, use_state,
};

#[function_component(Process1)]
pub fn process1() -> Html {
    let overlay_data = use_state(|| None::<Realtime>);

    {
        let overlay_data = overlay_data.clone();
        use_effect_with((), move |_| {
            global_bool!(set REALTIME_LOOP_FLAG, false);

            console::log_1(&"starting realtime loop".into());

            spawn_local(async move {
                let mut failures = 0usize;

                console::log_1(&"starting loop #1".into());
                invoke_get_live_game();

                loop {
                    web_sys::console::log_1(&"loop #1".into());
                    if global_bool!(get REALTIME_LOOP_FLAG) {
                        break;
                    }

                    if let Some(data) = take_live_game() {
                        overlay_data.set(Some(data));
                        failures = 0;
                    } else {
                        invoke_get_live_game();
                        web_sys::console::log_1(&"no data".into());
                        failures += 1;
                    };

                    console::log_1(&"sleeping".into());

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
        <div class={classes!("flex", "w-full", "justify-center")}>
            {
                if let Some(ref data) = *overlay_data {
                    html! {
                        <div>
                            <BaseTable
                                damaging_items={data.current_player.damaging_items.clone()}
                                damaging_runes={data.current_player.damaging_runes.clone()}
                                champion_id={data.current_player.champion_id}
                                damages={
                                    [data.enemies.first().unwrap()]
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
