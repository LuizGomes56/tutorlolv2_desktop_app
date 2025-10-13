use crate::{
    calculator_v2::{
        AbilitySelector, DragonAction, InputActivePlayerAction, InputDataAction,
        page::{ActionTracker, use_dragon_editor},
    },
    components::calculator::{
        ChampionBanner, Exception, ExceptionSelector, NumericField, OpenTray, Tray,
    },
    model_v2::{Dragons, OwnedActivePlayer},
    url,
};
use std::{cell::RefCell, rc::Rc};
use tutorlolv2_imports::{ItemId, RuneId};
use yew::{
    Callback, Html, Properties, UseReducerHandle, classes, function_component, hook, html,
    use_callback,
};

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

#[derive(Clone)]
struct EditorProps {
    input_current_player: UseReducerHandle<OwnedActivePlayer>,
    action_tracker: Rc<RefCell<ActionTracker>>,
}

#[hook]
fn use_editor<T: 'static>(
    props: EditorProps,
    closure: fn(T) -> InputActivePlayerAction,
) -> Callback<T> {
    let EditorProps {
        input_current_player,
        action_tracker,
    } = props;
    use_callback((), move |v, _| {
        action_tracker.replace(ActionTracker::CurrentPlayer);
        input_current_player.dispatch(closure(v));
    })
}

#[function_component(ActivePlayerData)]
pub fn active_player_data(props: &ActivePlayerDataProps) -> Html {
    let input_current_player = props.input_current_player.clone();
    let action_tracker = props.action_tracker.clone();
    let input_dragons = props.input_dragons.clone();
    let current_player_champion_id = input_current_player.data.champion_id;

    let editor_props = EditorProps {
        input_current_player: input_current_player.clone(),
        action_tracker: action_tracker.clone(),
    };

    let cb_champion_id = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::Data(InputDataAction::ChampionId(v))
    });
    let cb_insert_item = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::Data(InputDataAction::InsertItem(v))
    });
    let cb_remove_item = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::Data(InputDataAction::RemoveItem(v))
    });
    let cb_insert_rune = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::InsertRune(v)
    });
    let cb_remove_rune = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::RemoveRune(v)
    });
    let cb_level = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::Data(InputDataAction::Level(v))
    });
    let cb_ability_levels = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::AbilityLevels(v)
    });
    let cb_infer_stats = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::Data(InputDataAction::InferStats(v))
    });
    let cb_stacks = use_editor(editor_props.clone(), |v| {
        InputActivePlayerAction::Data(InputDataAction::Stacks(v))
    });
    let cb_fire_dragons = use_dragon_editor(
        action_tracker.clone(),
        ActionTracker::CurrentPlayer,
        input_dragons.clone(),
        DragonAction::AllyFire,
    );
    let cb_earth_dragons = use_dragon_editor(
        action_tracker.clone(),
        ActionTracker::CurrentPlayer,
        input_dragons.clone(),
        DragonAction::AllyEarth,
    );

    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-2", "w-60", "bg-[#141417]",
            "h-screen"
        )}>
            <ChampionBanner
                callback={cb_champion_id}
                champion_id={current_player_champion_id}
            />
            <div class={classes!("grid", "grid-cols-4", "gap-2", "px-4")}>
                <AbilitySelector
                    ability_levels={input_current_player.abilities}
                    callback={cb_ability_levels}
                    current_player_champion_id={current_player_champion_id}
                />
                <NumericField<u16>
                    title={"Number of ally fire dragons"}
                    source={Exception::Image}
                    img_url={url!("/img/other/fire_soul.avif")}
                    callback={cb_fire_dragons}
                />
                <NumericField<u16>
                    title={"Number of ally earth dragons"}
                    source={Exception::Image}
                    img_url={url!("/img/other/earth_soul.avif")}
                    callback={cb_earth_dragons}
                />
                <ExceptionSelector
                    champion_id={current_player_champion_id}
                    infer_stats={input_current_player.data.infer_stats}
                    stack_callback={cb_stacks}
                    infer_stats_callback={cb_infer_stats}
                />
            </div>
            <OpenTray<ItemId>
                insert_callback={cb_insert_item}
                title={"Search items"}
            />
            <Tray<ItemId>
                array={input_current_player.data.items.clone()}
                remove_callback={cb_remove_item}
            />
            <OpenTray<RuneId>
                insert_callback={cb_insert_rune}
                title={"Search runes"}
            />
            <Tray<RuneId>
                array={input_current_player.runes.clone()}
                remove_callback={cb_remove_rune}
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
