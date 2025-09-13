use crate::{
    MAX_FAILURES, REFRESH_RATE, RETRY_INTERVAL,
    components::{Image, ImageType},
    external::invoke::take_live_game,
};
use generated_code::ChampionId;
use yew::{
    Html, classes, function_component, html, platform::spawn_local, use_effect_with, use_state,
};

#[function_component(Process2)]
pub fn process2() -> Html {
    let overlay_data = use_state(|| None::<Vec<ChampionId>>);

    {
        let overlay_data = overlay_data.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let mut failures = 0usize;

                loop {
                    if let Some(data) = take_live_game() {
                        overlay_data.set(Some(
                            data.enemies.iter().map(|(x, _)| *x).collect::<Vec<_>>(),
                        ));
                    } else {
                        failures += 1;
                    }
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
                        <div class={classes!("flex", "gap-2")}>
                            {
                                data.iter().map(|x| {
                                    html! {
                                        <Image
                                            class={classes!("w-8", "h-8")}
                                            source={ImageType::Champions(*x)}
                                        />
                                    }
                                })
                                .collect::<Html>()
                            }
                        </div>
                    }
                } else {
                    html!()
                }
            }
        </div>
    }
}
