use crate::{
    components::{Image, ImageType},
    components_v2::base_table::MakeThead,
    model_v2::{Enemy, L_SIML, Realtime, TypeMetadata},
    url,
    utils::{ToStaticStr, api},
};
use std::{mem::MaybeUninit, time::Duration};
use tutorlolv2_imports::ItemId;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::js_sys::Uint8Array;
use yew::{
    platform::{spawn_local, time::sleep},
    prelude::*,
};

const BYTES: &[u8] = include_bytes!("../../../src-tauri/example.json");

#[wasm_bindgen(module = "/public/invoke.js")]
unsafe extern "C" {
    #[wasm_bindgen(js_name = "invoke_get_live_game", catch)]
    pub async fn get_live_game() -> Result<Uint8Array, JsValue>;
}

async fn get_data() -> Realtime {
    let bytes = get_live_game().await;

    if let Ok(response) = bytes {
        let bytes = response.to_vec();
        web_sys::console::log_1(&JsValue::from(bytes.len()));
        api::decode_bytes(
            gloo_net::http::Request::post(url!("/api/games/realtime"))
                .body(bytes)
                // .body(BYTES.to_vec())
                .unwrap()
                .send()
                .await
                .unwrap(),
        )
        .await
        .unwrap()
    } else {
        web_sys::console::log_1(&JsValue::from("No response"));
        panic!();
    }
}

fn find_best_5(arr: &[i32]) -> [usize; 5] {
    let mut top = [(i32::MIN, usize::MAX); 5];

    for (idx, &sum) in arr.iter().enumerate() {
        let mut pos = None;
        for i in 0..5 {
            if sum > top[i].0 || (sum == top[i].0 && idx < top[i].1) {
                pos = Some(i);
                break;
            }
        }
        if let Some(p) = pos {
            for j in (p + 1..5).rev() {
                top[j] = top[j - 1];
            }
            top[p] = (sum, idx);
        }
    }
    let mut out = [usize::MAX; 5];
    for i in 0..5 {
        out[i] = top[i].1;
    }
    out
}

fn display_siml(enemies: &[Enemy], metadata: &[TypeMetadata<ItemId>]) -> Html {
    let mut uninit_slice = MaybeUninit::<[i32; L_SIML]>::zeroed();
    let base_ptr = uninit_slice.as_mut_ptr().cast::<i32>();
    let add = |index: usize, items: &[i32]| {
        assert!(index < L_SIML);
        for rd in items {
            unsafe {
                let p = base_ptr.add(index);
                (*p) += *rd;
            }
        }
    };
    for enemy in enemies {
        for (index, damage) in enemy.siml_items.iter().enumerate() {
            add(index, &damage.abilities);
            add(index, &damage.items);
            add(index, &damage.runes);
        }
    }
    let siml_damages = unsafe { uninit_slice.assume_init() };
    let indexes = find_best_5(&siml_damages);
    indexes
        .into_iter()
        .map(|i| {
            html! {
                <div class={classes!("flex", "h-6", "gap-1")}>
                    <div class={classes!("content-center", "text-sm", metadata[i].damage_type.as_static_str())}>
                        {siml_damages[i]}
                    </div>
                    <Image class={classes!("w-6", "h-6")} source={ImageType::Item(metadata[i].kind)}/>
                </div>
            }
        })
        .collect()
}

fn make_cell<T: Copy>(metadata: &[TypeMetadata<T>], damages: &[i32]) -> Html {
    html! {
        for damages.iter().enumerate().map(|(i, damage)| {
            html! {
                <td class={classes!("text-center", "text-sm", metadata[i].damage_type.as_static_str())}>
                    {damage}
                </td>
            }
        })
    }
}

#[function_component(RealtimeOverlay)]
pub fn realtime_overlay() -> Html {
    let data = use_state(|| None::<Realtime>);
    let display_indexes = use_state(|| vec![0]);

    let cb_index = {
        let display_indexes = display_indexes.clone();
        use_callback(display_indexes, move |v, display_indexes| {
            let mut new_vec = (**display_indexes).clone();
            if (**display_indexes).contains(&v) {
                new_vec.retain(|&i| i != v);
            } else {
                new_vec.push(v);
            }
            display_indexes.set(new_vec);
        })
    };

    {
        let data = data.clone();
        use_effect_with((), |_| {
            spawn_local(async move {
                loop {
                    data.set(Some(get_data().await));
                    sleep(Duration::from_millis(1000)).await;
                }
            });
        });
    }

    html! {
        <main class={classes!("h-full", "w-full")}>
            {
                if let Some(game_data) = &*data {
                    let enemies = game_data.enemies
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| display_indexes.contains(&index))
                        .map(|(_, enemy)| enemy)
                        .collect::<Vec<_>>();
                    html! {
                        <div class={classes!("ml-[400px]")}>
                            <table>
                                <MakeThead
                                    champion_id={game_data.current_player.champion_id}
                                    abilities_meta={game_data.abilities_meta.clone()}
                                    items_meta={game_data.items_meta.clone()}
                                    runes_meta={game_data.runes_meta.clone()}
                                />
                                <tbody>
                                    {for enemies.iter().map(|enemy| {
                                        html! {
                                            <tr>
                                                <td>
                                                    <Image
                                                        class={classes!("w-6", "h-6")}
                                                        source={ImageType::Champion(enemy.champion_id)}
                                                    />
                                                </td>
                                                {make_cell(&game_data.abilities_meta, &enemy.damages.abilities)}
                                                {make_cell(&game_data.items_meta, &enemy.damages.items)}
                                                {make_cell(&game_data.runes_meta, &enemy.damages.runes)}
                                            </tr>
                                        }
                                    })}
                                </tbody>
                            </table>
                            <div class={classes!("absolute", "right-0", "top-16", "space-y-1")}>
                                {display_siml(&game_data.enemies, &game_data.siml_meta)}
                            </div>
                            <div class={classes!("absolute", "left-0", "bottom-0")}>
                                {for game_data.enemies.iter().enumerate().map(|(index, enemy)| {
                                    html! {
                                        <button onclick={{
                                            let cb_index = cb_index.clone();
                                            move |_| cb_index.emit(index)
                                        }}>
                                            <Image
                                                class={classes!("w-6", "h-6")}
                                                source={ImageType::Champion(enemy.champion_id)}
                                            />
                                        </button>
                                    }
                                })}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <h1>{"Loading..."}</h1>
                    }
                }
            }
        </main>
    }
}
