use crate::{
    STATIC_CHAMPIONS, color, components::formulas::source_code::SourceCode,
    hooks::mouseout::use_mouseout, svg, url,
};
use std::borrow::Cow;
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html,
    use_memo, use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct ChampionSelectorProps {
    pub callback: Callback<String>,
    pub current_champion: String,
}

#[derive(Clone, PartialEq)]
struct ChampionItem<'a> {
    name: Cow<'a, str>,
    html: Html,
}

#[function_component(ChampionSelector)]
fn champion_selector(props: &ChampionSelectorProps) -> Html {
    let is_open = use_state(|| false);

    let dropdown_ref = use_node_ref();
    let search_query = use_state(|| String::new());

    let label_ref = {
        let is_open = is_open.clone();
        use_mouseout(
            Callback::from(move |_| is_open.set(false)),
            [dropdown_ref.clone()],
        )
    };

    let all_champions = use_memo(props.callback.clone(), |callback| {
        STATIC_CHAMPIONS
            .get()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, (champion_id, champion_name))| {
                let html = html! {
                    <ChampionOptions
                        key={index}
                        callback={callback.clone()}
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
        <div class={classes!("relative", "max-w-48")}>
            <label
                ref={label_ref}
                class={classes!(
                    color!(bg-900), color!(border-600), color!(hover:bg-950),
                    "flex", "items-center", "gap-2", "py-2",
                    "text-zinc-200", "rounded", "border", "pl-12", "pr-4",
                    "relative"
                )}
            >
                <span class={classes!("absolute", "left-4", "text-zinc-400")}>
                    {svg!("../../public/svgs/search", "16")}
                </span>
                <input
                    type="text"
                    class={classes!(
                        "text-white",
                        "focus:outline-none",
                        "w-full"
                    )}
                    placeholder={{
                        STATIC_CHAMPIONS.get().unwrap()
                            .iter()
                            .find(|(id, _)| *id == &props.current_champion)
                            .map(|(_, name)| name.as_str())
                            .unwrap_or("Select Champion")
                    }}
                    value={(*search_query).clone()}
                    onfocus={{
                        let is_open = is_open.clone();
                        Callback::from(move |_| is_open.set(true))
                    }}
                    oninput={{
                        let search_query = search_query.clone();
                        Callback::from(move |e: InputEvent| {
                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            search_query.set(input);
                        })
                    }}
                />
            </label>

            <div
                ref={dropdown_ref}
                class={classes!(
                    "absolute", "left-0", "w-full", "mt-2",
                    "rounded", "flex-col", color!(bg-900),
                    "max-h-64", "overflow-y-auto", "z-10",
                    if *is_open { "flex" } else { "hidden" }
                )
            }>
                {visible_champions.iter().map(|item| item.html.clone()).collect::<Html>()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChampionOptionsProps {
    pub callback: Callback<String>,
    pub champion_id: String,
    pub champion_name: String,
}

#[function_component(ChampionOptions)]
fn champion_options(props: &ChampionOptionsProps) -> Html {
    html! {
        <div
            class={classes!(
                "p-1", "cursor-pointer", color!(hover:bg-800),
                "flex", "items-center", "gap-2", "text-zinc-200",
                "text-sm"
            )}
            onclick={{
                let champion_id = props.champion_id.clone();
                props.callback.reform(move |_| champion_id.to_string())
            }}
        >
            <img
                src={url!("/cdn/champions/{}.png", props.champion_id)}
                alt=""
                class={classes!("w-6", "h-6")}
            />
            <span>{&props.champion_name}</span>
        </div>
    }
}

#[function_component(Formulas)]
pub fn formulas() -> Html {
    let current_champion = use_state(|| String::from("Aatrox"));

    html! {
        // <FormulaSidebar />
        <div class={classes!(
            "p-6", "flex-1", "h-screen", "overflow-y-auto",
            "flex", "flex-col", "gap-4",
        )}>
            <div class={classes!("flex", "flex-wrap", "gap-4", "items-center")}>
                <h1 class={classes!(
                    "font-semibold", "text-2xl", "text-white"
                )}>
                    {"Formulas and Generator Code"}
                </h1>
                <ChampionSelector
                    callback={
                        let current_champion = current_champion.clone();
                        Callback::from(move |champion_id: String| {
                            current_champion.set(champion_id);
                        })
                    }
                    current_champion={(*current_champion).clone()}
                />
            </div>
            <SourceCode champion_id={(*current_champion).clone()} />
        </div>
    }
}
