use std::{collections::HashSet, mem::MaybeUninit};

use crate::{
    components::{Image, ImageType},
    components_v2::base_table::MakeThead,
    model_v2::{Enemy, L_SIML, RangeDamage, Realtime, TypeMetadata},
    url,
    utils::{ToStaticStr, api},
};
use tutorlolv2_imports::ItemId;
use yew::{platform::spawn_local, prelude::*};

const BYTES: &[u8] = include_bytes!("../../../src-tauri/example.json");

async fn get_data() -> Realtime {
    api::decode_bytes(
        gloo_net::http::Request::post(url!("/api/games/realtime"))
            .body(BYTES.to_vec())
            .unwrap()
            .send()
            .await
            .unwrap(),
    )
    .await
    .unwrap()
}

fn find_best_5(arr: &[RangeDamage]) -> [usize; 5] {
    let mut top: [(i64, usize); 5] = [(i64::MIN, usize::MAX); 5];

    for (idx, rd) in arr.iter().enumerate() {
        let sum = rd.minimum_damage as i64 + rd.maximum_damage as i64;
        if sum <= 0 {
            continue;
        }
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
    let mut uninit_slice = MaybeUninit::<[RangeDamage; L_SIML]>::zeroed();
    let base_ptr = uninit_slice.as_mut_ptr().cast::<RangeDamage>();
    let add = |index: usize, items: &[RangeDamage]| {
        assert!(index < L_SIML);
        for rd in items {
            unsafe {
                let p = base_ptr.add(index);
                (*p).minimum_damage = (*p).minimum_damage.saturating_add(rd.minimum_damage);
                (*p).maximum_damage = (*p).maximum_damage.saturating_add(rd.maximum_damage);
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
                        {siml_damages[i].minimum_damage}
                        {(siml_damages[i].maximum_damage != 0).then_some(
                            format!(" - {}", siml_damages[i].maximum_damage)
                        )}
                    </div>
                    <Image class={classes!("w-6", "h-6")} source={ImageType::Item(metadata[i].kind)}/>
                </div>
            }
        })
        .collect()
}

fn make_cell<T: Copy>(metadata: &[TypeMetadata<T>], range_damage: &[RangeDamage]) -> Html {
    html! {
        for range_damage.iter().enumerate().map(|(i, range_damage)| {
            html! {
                <td class={classes!("text-center", "text-sm", metadata[i].damage_type.as_static_str())}>
                    {range_damage.minimum_damage}
                    {(range_damage.maximum_damage != 0).then_some(
                        format!(" - {}", range_damage.maximum_damage)
                    )}
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
                data.set(Some(get_data().await));
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
                        <div>
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
                            <div class={classes!("absolute", "right-0", "top-0", "space-y-1")}>
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
