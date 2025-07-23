use crate::{
    color,
    components::calculator::{StaticIterator, StaticSelector},
};
use yew::{
    Callback, Html, Properties, classes, function_component, html, use_callback, use_memo,
    use_state,
};

#[derive(PartialEq, Properties)]
pub struct MainSelectorProps {
    pub set_current_player_champion_callback: Callback<String>,
    pub insert_item_callback: Callback<u32>,
    pub remove_item_callback: Callback<usize>,
    pub insert_rune_callback: Callback<u32>,
    pub remove_rune_callback: Callback<usize>,
}

#[derive(PartialEq)]
enum SelectedTab {
    Champions,
    Items,
    Runes,
}

#[function_component(MainSelector)]
pub fn main_selector(props: &MainSelectorProps) -> Html {
    let selected_tab = use_state(|| SelectedTab::Items);

    let set_champions_tab = {
        let selected_tab = selected_tab.clone();
        use_callback((), move |_, _| selected_tab.set(SelectedTab::Champions))
    };

    let set_items_tab = {
        let selected_tab = selected_tab.clone();
        use_callback((), move |_, _| selected_tab.set(SelectedTab::Items))
    };

    let set_runes_tab = {
        let selected_tab = selected_tab.clone();
        use_callback((), move |_, _| selected_tab.set(SelectedTab::Runes))
    };

    let tab_buttons = use_memo(
        (
            set_champions_tab.clone(),
            set_items_tab.clone(),
            set_runes_tab.clone(),
        ),
        move |(set_champions_tab, set_items_tab, set_runes_tab)| {
            html! {
                <div class={classes!(
                    "grid", "grid-cols-3", "gap-4", "text-center",
                    "text-white", "text-lg"
                )}>
                    <button onclick={set_champions_tab}>{ "Champions" }</button>
                    <button onclick={set_items_tab}>{ "Items" }</button>
                    <button onclick={set_runes_tab}>{ "Runes" }</button>
                </div>
            }
        },
    );

    html! {
        <div class={classes!(
            // "hidden",
            "absolute", "top-1/2", "left-1/2",
            "-translate-x-1/2", "-translate-y-1/2",
            "max-w-2xl", "w-full", "h-2/3",
            "overflow-y-auto", color!(bg-900),
            "grid", "grid-cols-[auto_1fr]"
        )}>
            <div class={classes!("p-4", "flex", "flex-col", "gap-4")}>
                {(*tab_buttons).clone()}
                <div class={classes!(
                    if *selected_tab == SelectedTab::Champions {
                        "block"
                    } else {
                        "hidden"
                    })}>
                    { "Coming soon" }
                </div>
                <div class={classes!(
                    if *selected_tab == SelectedTab::Items {
                        "block"
                    } else {
                        "hidden"
                    })}>
                    <StaticSelector
                        static_iter={StaticIterator::Items}
                        insert_callback={props.insert_item_callback.clone()}
                    />
                </div>
                <div class={classes!(
                    if *selected_tab == SelectedTab::Runes {
                        "block"
                    } else {
                        "hidden"
                    })}>
                    <StaticSelector
                        static_iter={StaticIterator::Runes}
                        insert_callback={props.insert_rune_callback.clone()}
                    />
                </div>
            </div>
            <div>{ "..." }</div>
            // <StaticSelector
            //     static_iter={StaticIterator::Items}
            //     insert_callback={props.insert_item_callback.clone()}
            //  ?   // iterator={input_game.active_player.items.clone()}
            //  ?  // remove_callback={remove_current_player_items}
            // />
            // <StaticSelector
            //     static_iter={StaticIterator::Runes}
            //     iterator={input_game.active_player.runes.clone()}
            //     insert_callback={insert_current_player_runes}
            //     remove_callback={remove_current_player_runes}
            // />
        </div>
    }
}
