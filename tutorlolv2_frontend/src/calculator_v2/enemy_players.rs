use std::{cell::RefCell, rc::Rc};

use crate::{
    calculator_v2::{
        DragonAction, EnemyAction, InputDataAction, InputEnemies, page::ActionTracker,
    },
    components::calculator::{
        ChampionBanner, Exception, ExceptionSelector, NumericField, OpenTray, Tray,
    },
    model_v2::*,
    url,
};
use tutorlolv2_imports::ItemId;
use yew::{
    Html, Properties, UseReducerHandle, classes, function_component, html, use_callback, use_state,
};

#[derive(Properties)]
pub struct EnemyPlayersDataProps {
    pub input_enemy_players: UseReducerHandle<InputEnemies<SimpleStats>>,
    pub input_dragons: UseReducerHandle<Dragons>,
    pub action_tracker: Rc<RefCell<ActionTracker>>,
}

impl PartialEq for EnemyPlayersDataProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[function_component(EnemyPlayersData)]
pub fn enemy_players_data(props: &EnemyPlayersDataProps) -> Html {
    let input_enemy_players = props.input_enemy_players.clone();
    let input_dragons = props.input_dragons.clone();
    let action_tracker = props.action_tracker.clone();
    let input_enemy_index = use_state(|| 0);

    let set_enemy_earth_dragons = {
        let input_dragons = input_dragons.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_dragons.dispatch(DragonAction::EnemyEarth(v));
        })
    };
    let set_enemy_champion_id = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemyAction::Edit(
                input_enemy_index,
                InputDataAction::ChampionId(v),
            ));
        })
    };
    let set_enemy_stacks = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemyAction::Edit(
                input_enemy_index,
                InputDataAction::Stacks(v),
            ));
        })
    };
    let set_enemy_infer_stats = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemyAction::Edit(
                input_enemy_index,
                InputDataAction::InferStats(v),
            ));
        })
    };
    let set_enemy_stats = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemyAction::Edit(
                input_enemy_index,
                InputDataAction::Stats(v),
            ));
        })
    };
    let set_enemy_level = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemyAction::Edit(
                input_enemy_index,
                InputDataAction::Level(v),
            ));
        })
    };
    let insert_enemy_player_items = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemyAction::Edit(
                input_enemy_index,
                InputDataAction::InsertItem(v),
            ));
        })
    };
    let remove_enemy_player_items = {
        let input_enemy_players = input_enemy_players.clone();
        let input_enemy_index = *input_enemy_index;
        let action_tracker = action_tracker.clone();
        use_callback(input_enemy_index, move |v, _| {
            action_tracker.replace(ActionTracker::EnemyPlayer(input_enemy_index));
            input_enemy_players.dispatch(EnemyAction::Edit(
                input_enemy_index,
                InputDataAction::RemoveItem(v),
            ));
        })
    };

    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-2", "w-60", "bg-[#141417]",
            "h-screen",
        )}>
            {(*input_enemy_players).as_ref().get(*input_enemy_index).map(|input_enemy| html! {
                <>
                    <ChampionBanner
                        callback={set_enemy_champion_id}
                        champion_id={input_enemy.champion_id}
                        translate_left={true}
                    />
                    <div class={classes!("grid", "grid-cols-4", "gap-2", "px-4")}>
                        <NumericField<u16>
                            title={"Number of enemy earth dragons"}
                            source={Exception::Image}
                            img_url={url!("/img/other/earth_soul.avif")}
                            callback={set_enemy_earth_dragons}
                        />
                        <ExceptionSelector
                            champion_id={input_enemy.champion_id}
                            infer_stats={input_enemy.infer_stats}
                            stack_callback={set_enemy_stacks}
                            infer_stats_callback={set_enemy_infer_stats}
                        />
                    </div>
                    // <BasicStatsSelector
                    //     champion_stats={input_enemy.stats}
                    //     infer_stats={input_enemy.infer_stats}
                    //     set_stats_callback={set_enemy_stats}
                    //     set_level_callback={set_enemy_level}
                    //     level={input_enemy.level}
                    // />
                    <OpenTray<ItemId>
                        insert_callback={insert_enemy_player_items}
                        title={"Search items"}
                    />
                    <Tray<ItemId>
                        array={input_enemy.items.clone()}
                        remove_callback={remove_enemy_player_items}
                        translate_left={true}
                    />
                </>
            })}
        </div>
    }
}
