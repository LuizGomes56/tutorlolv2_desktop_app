use crate::{
    REFRESH_RATE, RETRY_INTERVAL,
    components::{Image, ImageType},
    external::invoke::RT_ENEMY_IDS,
};
use gloo_timers::callback::{Interval, Timeout};
use tutorlolv2_imports::ChampionId;
use yew::{Html, TearDown, classes, function_component, html, use_effect_with, use_state_eq};

#[function_component(Process2)]
pub fn process2() -> Html {
    let overlay_data = use_state_eq(|| None::<Vec<ChampionId>>);

    {
        let overlay_data = overlay_data.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(REFRESH_RATE as u32, move || {
                let data = RT_ENEMY_IDS.take();
                if data.is_none() {
                    let _ = Timeout::new(RETRY_INTERVAL as u32, || {});
                } else {
                    overlay_data.set(data);
                }
            });
            TearDown::tear_down(move || {
                interval.cancel();
            })
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
                                            source={ImageType::Champion(*x)}
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
