use crate::{
    STATIC_CHAMPIONS, color,
    components::{Sprite, SpriteType},
    hooks::mouseout::use_mouseout,
    svg,
};
use std::borrow::Cow;
use yew::{
    AttrValue, Callback, Html, InputEvent, Properties, TargetCast, classes, function_component,
    html, use_callback, use_memo, use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct ChampionSelectorProps {
    pub callback: Callback<String>,
    pub current_champion: AttrValue,
}

#[derive(Clone, PartialEq)]
struct ChampionItem<'a> {
    name: Cow<'a, str>,
    html: Html,
}

#[function_component(ChampionSelector)]
pub fn champion_selector(props: &ChampionSelectorProps) -> Html {
    let is_open = use_state(|| false);
    let search_query = use_state(|| String::new());
    let name_selected = use_state(|| props.current_champion.to_string());
    let callback = {
        let original_callback = props.callback.clone();
        let name_selected = name_selected.clone();
        use_callback(
            (original_callback, name_selected),
            |v: String, (original_callback, name_selected)| {
                name_selected.set(v.clone());
                original_callback.emit(v);
            },
        )
    };

    let dropdown_ref = use_node_ref();
    let label_ref = {
        let is_open = is_open.clone();
        use_mouseout(
            Callback::from(move |_| is_open.set(false)),
            [dropdown_ref.clone()],
        )
    };

    let onfocus = {
        let is_open = is_open.clone();
        use_callback((), move |_, _| is_open.set(true))
    };

    let oninput = {
        let search_query = search_query.clone();
        use_callback((), move |e: InputEvent, _| {
            let input = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            search_query.set(input);
        })
    };

    let all_champions = use_memo(callback, |callback| {
        STATIC_CHAMPIONS
            .get()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, (champion_id, champion_name))| {
                let html = html! {
                    <ChampionOptions
                        key={index}
                        callback={callback}
                        champion_id={champion_id.clone()}
                        champion_name={champion_name.clone()}
                    />
                };

                ChampionItem {
                    name: Cow::Borrowed(champion_name),
                    html,
                }
            })
            .collect::<Vec<_>>()
    });

    let visible_champions = all_champions
        .iter()
        .filter(|item| {
            item.name
                .to_lowercase()
                .contains(&search_query.to_lowercase())
        })
        .collect::<Vec<_>>();

    html! {
        <div class={classes!("relative")}>
            <label
                ref={label_ref}
                class={classes!(
                    "bg-[#121214]", color!(hover:bg-950),
                    "flex", "items-center", "gap-2", "h-10",
                    color!(text-200), "pl-10", "pr-4",
                    "relative"
                )}
            >
                <span class={classes!("absolute", "left-4", color!(text-400))}>
                    {svg!("../../public/svgs/search", "14")}
                </span>
                <input
                    type={"text"}
                    class={classes!(
                        "text-white", "focus:outline-none", "w-full", "ml-1"
                    )}
                    value={(*search_query).clone()}
                    placeholder={(*name_selected).clone()}
                    onfocus={onfocus}
                    oninput={oninput}
                />
            </label>
            <div
                ref={dropdown_ref}
                class={classes!(
                    "absolute", "left-0", "w-full",
                    "flex-col", color!(bg-900),
                    "max-h-64", "overflow-y-auto", "z-10",
                    if *is_open { "flex" } else { "hidden" }
                )
            }>
                {for visible_champions.iter().map(|item| item.html.clone())}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChampionOptionsProps {
    pub callback: Callback<String>,
    pub champion_id: AttrValue,
    pub champion_name: AttrValue,
}

#[function_component(ChampionOptions)]
fn champion_options(props: &ChampionOptionsProps) -> Html {
    html! {
        <button
            class={classes!(
                "p-1", "cursor-pointer", color!(hover:bg-800),
                "flex", "items-center", "gap-2", color!(text-200),
                "text-sm", "select-none"
            )}
            onclick={{
                let champion_id = props.champion_id.clone();
                props.callback.reform(move |_| champion_id.to_string())
            }}
        >
            <Sprite size={20} source={SpriteType::Champions(props.champion_id.to_string())} />
            <span>{&props.champion_name}</span>
        </button>
    }
}
