use crate::{
    MAX_FAILURES, REFRESH_RATE, RETRY_INTERVAL,
    components::tables::{
        BaseTable,
        cells::{DisplayDamage, ImageCell, Instances},
    },
    external::invoke::{invoke_get_live_game, take_live_game},
    global_bool,
    models::realtime::Realtime as RealtimeData,
};
use web_sys::console;
use yew::{
    Html, classes, function_component, html, platform::spawn_local, use_effect_with, use_state,
};

#[function_component(Realtime)]
pub fn realtime() -> Html {
    let game_data = use_state(|| None::<RealtimeData>);

    if !global_bool!(get IS_DEKTOP_PLATFORM) {
        return html! {
            <div class={classes!(
                "p-6", "flex-1", "flex", "flex-col", "gap-4",
            )}>
                <h1 class={classes!(
                    "font-semibold", "text-2xl", "text-white"
                )}>
                    { "Download the desktop application to use this feature" }
                </h1>
            </div>
        };
    }

    {
        let game_data = game_data.clone();
        use_effect_with((), |_| {
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
                        game_data.set(Some(data));
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
        <div class={classes!(
            "p-6", "flex-1", "flex", "flex-col", "gap-4",
        )}>
            {
                if let Some(ref data) = *game_data {
                    html! {
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
                    }
                } else {
                    html! { "No data" }
                }
            }
        </div>
    }
}
