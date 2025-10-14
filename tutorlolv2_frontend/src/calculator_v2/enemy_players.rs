use crate::{
    calculator_v2::{
        DragonAction, EnemyAction, InputDataAction, InputEnemies,
        page::{
            ActionTracker, DragonCallbackProps, EnemyCallbackProps, use_dragon_callback,
            use_enemy_callback,
        },
    },
    components::calculator::{
        ChampionBanner, Exception, ExceptionSelector, NumericField, OpenTray, Tray,
    },
    model_v2::*,
    url,
};
use std::{cell::RefCell, rc::Rc};
use tutorlolv2_imports::ItemId;
use yew::{
    Callback, Html, Properties, UseReducerHandle, UseStateHandle, classes, function_component,
    hook, html, use_callback, use_state,
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

    let editor_props = EnemyCallbackProps {
        input_enemy_players: input_enemy_players.clone(),
        input_enemy_index: input_enemy_index.clone(),
        action_tracker: action_tracker.clone(),
    };

    let cb_champion_id = use_enemy_callback(editor_props.clone(), InputDataAction::ChampionId);
    let cb_stacks = use_enemy_callback(editor_props.clone(), InputDataAction::Stacks);
    let cb_infer_stats = use_enemy_callback(editor_props.clone(), InputDataAction::InferStats);
    let cb_stats = use_enemy_callback(editor_props.clone(), InputDataAction::Stats);
    let cb_level = use_enemy_callback(editor_props.clone(), InputDataAction::Level);
    let cb_insert_item = use_enemy_callback(editor_props.clone(), InputDataAction::InsertItem);
    let cb_remove_item = use_enemy_callback(editor_props, InputDataAction::RemoveItem);
    let cb_earth_dragons = use_dragon_callback(
        DragonCallbackProps {
            new_action_tracker: ActionTracker::EnemyPlayer(*input_enemy_index),
            action_tracker,
            input_dragons,
        },
        DragonAction::EnemyEarth,
    );

    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-2", "w-60", "bg-[#141417]",
            "h-screen",
        )}>
            {(*input_enemy_players).as_ref().get(*input_enemy_index).map(|input_enemy| html! {
                <>
                    <ChampionBanner
                        callback={cb_champion_id}
                        champion_id={input_enemy.champion_id}
                        translate_left={true}
                    />
                    <div class={classes!("grid", "grid-cols-4", "gap-2", "px-4")}>
                        <NumericField<u16>
                            title={"Number of enemy earth dragons"}
                            source={Exception::Image}
                            img_url={url!("/img/other/earth_soul.avif")}
                            callback={cb_earth_dragons}
                        />
                        <ExceptionSelector
                            champion_id={input_enemy.champion_id}
                            infer_stats={input_enemy.infer_stats}
                            stack_callback={cb_stacks}
                            infer_stats_callback={cb_infer_stats}
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
                        insert_callback={cb_insert_item}
                        title={"Search items"}
                    />
                    <Tray<ItemId>
                        array={input_enemy.items.clone()}
                        remove_callback={cb_remove_item}
                        translate_left={true}
                    />
                </>
            })}
        </div>
    }
}
