use crate::{
    calculator_v2::{
        AbilitySelector, DragonAction, InputActivePlayerAction, InputDataAction,
        page::ActionTracker,
    },
    components::calculator::{
        ChampionBanner, Exception, ExceptionSelector, NumericField, OpenTray, Tray,
    },
    model_v2::{Dragons, OwnedActivePlayer},
    url,
};
use std::{cell::RefCell, rc::Rc};
use tutorlolv2_imports::{ItemId, RuneId};
use yew::{Html, Properties, UseReducerHandle, classes, function_component, html, use_callback};

#[derive(Properties)]
pub struct ActivePlayerDataProps {
    pub input_current_player: UseReducerHandle<OwnedActivePlayer>,
    pub action_tracker: Rc<RefCell<ActionTracker>>,
    pub input_dragons: UseReducerHandle<Dragons>,
}

impl PartialEq for ActivePlayerDataProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[function_component(ActivePlayerData)]
pub fn active_player_data(props: &ActivePlayerDataProps) -> Html {
    let input_current_player = props.input_current_player.clone();
    let action_tracker = props.action_tracker.clone();
    let input_dragons = props.input_dragons.clone();
    let current_player_champion_id = input_current_player.data.champion_id;

    let set_current_player_champion_id = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(InputActivePlayerAction::Data(
                InputDataAction::ChampionId(v),
            ));
        })
    };
    let insert_current_player_items = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(InputActivePlayerAction::Data(
                InputDataAction::InsertItem(v),
            ));
        })
    };
    let remove_current_player_items = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(InputActivePlayerAction::Data(
                InputDataAction::RemoveItem(v),
            ));
        })
    };
    let insert_current_player_runes = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(InputActivePlayerAction::InsertRune(v));
        })
    };
    let remove_current_player_runes = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(InputActivePlayerAction::RemoveRune(v));
        })
    };
    let change_ability_level = {
        let input_current_player = input_current_player.clone();
        use_callback((), move |v, _| {
            input_current_player.dispatch(InputActivePlayerAction::AbilityLevels(v));
        })
    };
    let set_current_player_infer_stats = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player.dispatch(InputActivePlayerAction::Data(
                InputDataAction::InferStats(v),
            ));
        })
    };
    let set_current_player_stacks = {
        let input_current_player = input_current_player.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_current_player
                .dispatch(InputActivePlayerAction::Data(InputDataAction::Stacks(v)));
        })
    };
    let set_ally_fire_dragons = {
        let input_dragons = input_dragons.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_dragons.dispatch(DragonAction::AllyFire(v));
        })
    };
    let set_ally_earth_dragons = {
        let input_dragons = input_dragons.clone();
        let action_tracker = action_tracker.clone();
        use_callback((), move |v, _| {
            action_tracker.replace(ActionTracker::CurrentPlayer);
            input_dragons.dispatch(DragonAction::AllyEarth(v));
        })
    };

    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-2", "w-60", "bg-[#141417]",
            "h-screen"
        )}>
            <ChampionBanner
                callback={set_current_player_champion_id}
                champion_id={current_player_champion_id}
            />
            <div class={classes!("grid", "grid-cols-4", "gap-2", "px-4")}>
                <AbilitySelector
                    ability_levels={input_current_player.abilities}
                    callback={change_ability_level}
                    current_player_champion_id={current_player_champion_id}
                />
                <NumericField<u16>
                    title={"Number of ally fire dragons"}
                    source={Exception::Image}
                    img_url={url!("/img/other/fire_soul.avif")}
                    callback={set_ally_fire_dragons}
                />
                <NumericField<u16>
                    title={"Number of ally earth dragons"}
                    source={Exception::Image}
                    img_url={url!("/img/other/earth_soul.avif")}
                    callback={set_ally_earth_dragons}
                />
                <ExceptionSelector
                    champion_id={current_player_champion_id}
                    infer_stats={input_current_player.data.infer_stats}
                    stack_callback={set_current_player_stacks}
                    infer_stats_callback={set_current_player_infer_stats}
                />
            </div>
            <OpenTray<ItemId>
                insert_callback={insert_current_player_items}
                title={"Search items"}
            />
            <Tray<ItemId>
                array={input_current_player.data.items.clone()}
                remove_callback={remove_current_player_items}
            />
            <OpenTray<RuneId>
                insert_callback={insert_current_player_runes}
                title={"Search runes"}
            />
            <Tray<RuneId>
                array={input_current_player.runes.clone()}
                remove_callback={remove_current_player_runes}
            />
            // <StatsSelector
            //     champion_stats={input_current_player.data.stats}
            //     infer_stats={input_current_player.data.infer_stats}
            //     set_stats_callback={set_current_player_stats}
            //     set_level_callback={set_current_player_level}
            //     level={input_current_player.data.level}
            // />
        </div>
    }
}
