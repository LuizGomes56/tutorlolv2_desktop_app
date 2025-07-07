use crate::{
    STATIC_CHAMPIONS, STATIC_FORMULAS, color, components::sidebar::Sidebar, external::highliter,
    hooks::mouseout::use_mouseout, url,
};
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html,
    use_node_ref, use_state, virtual_dom::VNode,
};

#[derive(Properties, PartialEq)]
pub struct SourceCodeProps {
    pub champion_id: String,
}

#[function_component(SourceCode)]
fn source_code(props: &SourceCodeProps) -> Html {
    let code = STATIC_FORMULAS
        .get()
        .and_then(|map| map.get(props.champion_id.as_str()))
        .map(String::as_str)
        .unwrap_or("Failed to fetch code");
    html! {
        <code class={classes!("text-[#D4D4D4]")}>
            {VNode::from_html_unchecked(highliter::highlight(code).into())}
        </code>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChampionSelectorProps {
    pub callback: Callback<String>,
    pub current_champion: String,
}

#[function_component(ChampionSelector)]
fn champion_selector(props: &ChampionSelectorProps) -> Html {
    let is_open = use_state(|| false);

    let dropdown_ref = use_node_ref();
    let search_query = use_state(|| String::new());

    let button_ref = {
        let is_open = is_open.clone();
        use_mouseout(
            Callback::from(move |_| is_open.set(false)),
            [dropdown_ref.clone()],
        )
    };

    html! {
        <div class={classes!("relative")}>
            <input
                ref={button_ref}
                type="text"
                class={classes!(
                    color!(bg-900), color!(border-600), color!(hover:bg-700),
                    "text-white", "px-4", "py-2", "rounded",
                    "cursor-pointer", "border", "focus:outline-none",
                    "flex", "items-center", "gap-2",
                )}
                placeholder={props.current_champion.clone()}
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

            // {
            //     STATIC_CHAMPIONS.get().unwrap()
            //         .iter()
            //         .find(|(id, _)| *id == &props.current_champion)
            //         .map(|(_, name)| name.as_str())
            //         .unwrap_or("Select Champion")
            // }

            <div
                ref={dropdown_ref}
                class={classes!(
                    "absolute", "left-0", "w-full",
                    color!(bg-900), "rounded", "flex-col",
                    "max-h-64", "overflow-y-auto", "z-10",
                    if *is_open { "flex" } else { "hidden" }
                )
            }>
                {
                    STATIC_CHAMPIONS.get().unwrap().iter().map(|(champion_id, champion_name)| {
                        html! {
                            <div
                                class={classes!(
                                    "p-1", "cursor-pointer", color!(hover:bg-700),
                                    "flex", "items-center", "gap-2", "text-zinc-200",
                                    "text-sm",
                                    if champion_id == &props.current_champion {
                                        color!(bg-900)
                                    } else { "" }
                                )}
                                onclick={{
                                    let callback = props.callback.clone();
                                    let is_open = is_open.clone();
                                    let champion_id_clone = champion_id.clone();
                                    Callback::from(move |_| {
                                        callback.emit(champion_id_clone.clone());
                                        is_open.set(false);
                                    })
                                }}
                            >
                                <img
                                    src={url!("/cdn/champions/{}.png", champion_id)}
                                    alt=""
                                    class={classes!("w-6", "h-6")}
                                />
                                <span>{champion_name}</span>
                            </div>
                        }
                    })
                    .collect::<Html>()
                }
            </div>
        </div>
    }
}

#[function_component(Formulas)]
pub fn formulas() -> Html {
    let current_champion = use_state(|| String::from("Aatrox"));

    html! {
        <div class={classes!(
            "flex", "w-full"
        )}>
            <Sidebar />
            <div class={classes!(
                "flex", "flex-1",
                color!(bg-900)
            )}>
                // <FormulaSidebar />
                <div class={classes!(
                    "p-6", "flex-1", "h-screen", "overflow-y-auto",
                    "flex", "flex-col", "gap-4",
                )}>
                    <div class={classes!("max-w-3xl", "flex", "flex-col", "gap-4")}>
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
            </div>
        </div>
    }
}
