use crate::{
    color,
    components::{Image, ImageType},
    hooks::mouseout::use_mouseout,
    models::shared::ChampionId,
    svg,
};
use generated_code::CHAMPION_ID_TO_NAME;
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html,
    use_callback, use_memo, use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct ChampionSelectorProps {
    pub callback: Callback<ChampionId>,
    pub current_champion: ChampionId,
}

#[derive(Clone, PartialEq)]
struct ChampionItem {
    name: &'static str,
    html: Html,
}

#[function_component(ChampionSelector)]
pub fn champion_selector(props: &ChampionSelectorProps) -> Html {
    let is_open = use_state(|| false);
    let search_query = use_state(|| String::new());
    let callback = {
        let original_callback = props.callback.clone();
        use_callback(original_callback, |v, original_callback| {
            original_callback.emit(v);
        })
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
        CHAMPION_ID_TO_NAME
            .into_iter()
            .enumerate()
            .map(|(index, name)| {
                let html = html! {
                    <ChampionOptions
                        key={index}
                        callback={callback}
                        champion_id={ChampionId::unsafe_cast(index as u8)}
                    />
                };

                ChampionItem { name, html }
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
                    "bg-[#1f1f25]", color!(hover:bg-950),
                    "flex", "items-center", "gap-2", "h-10",
                    color!(text-200), "pl-10", "pr-4",
                    "relative", "rounded-md"
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
                    placeholder={*CHAMPION_ID_TO_NAME.get(props.current_champion as usize).unwrap_or(&"Unknown")}
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
    pub callback: Callback<ChampionId>,
    pub champion_id: ChampionId,
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
                let champion_id = props.champion_id;
                props.callback.reform(move |_| champion_id)
            }}
        >
            <Image class={classes!("w-5", "h-5")} source={ImageType::Champions(props.champion_id)} />
            <span>{CHAMPION_ID_TO_NAME.get(props.champion_id as usize).unwrap_or(&"Unknown")}</span>
        </button>
    }
}
