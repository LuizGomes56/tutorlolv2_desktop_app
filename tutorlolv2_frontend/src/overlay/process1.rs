use crate::{
    MAX_FAILURES, REFRESH_RATE, RETRY_INTERVAL,
    components::tables::{
        BaseTable,
        cells::{DisplayDamage, ImageCell, Instances},
    },
    external::invoke::invoke_get_live_game,
    global_bool,
    models::realtime::Realtime,
};
use std::rc::Rc;
use web_sys::console;
use yew::{
    Html, classes, function_component, html, platform::spawn_local, use_effect_with, use_state,
};

#[function_component(Process1)]
pub fn process1() -> Html {
    let overlay_data = use_state(|| Rc::new(None::<Realtime>));

    {
        let overlay_data = overlay_data.clone();
        use_effect_with((), move |_| {
            global_bool!(set REALTIME_LOOP_FLAG, false);

            console::log_1(&"starting realtime loop".into());

            spawn_local(async move {
                let mut failures = 0usize;

                console::log_1(&"starting loop #1".into());

                loop {
                    if global_bool!(get REALTIME_LOOP_FLAG) {
                        break;
                    }

                    console::log_1(&"getting realtime data".into());

                    let response = invoke_get_live_game().await;

                    console::log_1(&"got response".into());

                    if let Ok(data) = response {
                        match bincode::decode_from_slice::<Realtime, _>(
                            &data.to_vec(),
                            bincode::config::standard(),
                        ) {
                            Ok((realtime_data, _)) => {
                                console::log_1(&"got realtime data".into());
                                // print bytes
                                console::log_1(&format!("vec:{:?}", data.to_vec()).into());
                                overlay_data.set(Rc::new(Some(realtime_data)));
                                failures = 0;
                            }
                            Err(e) => {
                                console::log_1(
                                    &format!("Decode Error: {:#?}", e).to_string().into(),
                                );
                                failures += 1;
                            }
                        }
                    } else {
                        console::log_1(&response.unwrap_err().to_string().into());
                        failures += 1;
                    };

                    console::log_1(&"sleeping".into());

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
        <>
            {
                if let Some(ref data) = **overlay_data {
                    html! {
                        <div>
                            <BaseTable
                                damaging_items={data.current_player.damaging_items.clone()}
                                damaging_runes={data.current_player.damaging_runes.clone()}
                                champion_id={data.current_player.champion_id.clone()}
                                // damages={
                                //     data.enemies
                                //         .iter()
                                //         .map(|(enemy_champion_id, enemy)| {
                                //             html! {
                                //                 <tr>
                                //                     <td class={classes!("w-10", "h-10")}>
                                //                         <ImageCell instance={Instances::Champions(*enemy_champion_id)} />
                                //                     </td>
                                //                     {enemy.damages.attacks.display_damage()}
                                //                     {enemy.damages.abilities.display_damage()}
                                //                     {enemy.damages.items.display_damage()}
                                //                     {enemy.damages.runes.display_damage()}
                                //                 </tr>
                                //             }
                                //         })
                                //         .collect::<Html>()
                                // }
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
        </>
    }
}
