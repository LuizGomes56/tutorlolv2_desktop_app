use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::{Image, ImageType},
    model_v2::{RangeDamage, Realtime, TypeMetadata},
    url,
    utils::{ToStaticStr, api},
};

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

fn make_header<T: Copy>(metadata: &[TypeMetadata<T>], closure: impl Fn(T) -> ImageType) -> Html {
    html! {
        for metadata.iter().map(|meta| {
            html! {
                <th class={classes!("justify-items-center")}>
                    <Image
                        class={classes!("w-6", "h-6")}
                        source={closure(meta.kind)}
                    />
                </th>
            }
        })
    }
}

fn make_cell<T: Copy>(metadata: &[TypeMetadata<T>], range_damage: &[RangeDamage]) -> Html {
    html! {
        for range_damage.iter().enumerate().map(|(i, range_damage)| {
            html! {
                <td class={classes!("text-center", metadata[i].damage_type.as_static_str())}>
                    {range_damage.minimum_damage}{" - "}{range_damage.maximum_damage}
                </td>
            }
        })
    }
}

#[function_component(RealtimeOverlay)]
pub fn realtime_overlay() -> Html {
    let data = use_state(|| None::<Realtime>);
    let display_indexes = use_state(Vec::<usize>::new);

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
                        <div class={classes!("flex")}>
                            <table>
                                <thead>
                                    <th></th>
                                    {make_header(&game_data.abilities_meta, |v| ImageType::Ability(game_data.current_player.champion_id, v))}
                                    {make_header(&game_data.items_meta, |v| ImageType::Item(v))}
                                    {make_header(&game_data.runes_meta, |v| ImageType::Rune(v))}
                                </thead>
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
