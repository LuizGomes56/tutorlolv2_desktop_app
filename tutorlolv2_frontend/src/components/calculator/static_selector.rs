use crate::{
    components::Image,
    svg,
    utils::{ImportedEnum, RandomInput},
};
use yew::{
    Callback, Html, InputEvent, NodeRef, Properties, TargetCast, classes, function_component, html,
    use_callback, use_memo, use_state,
};

#[derive(PartialEq, Properties)]
pub struct StaticEventProps<T: PartialEq + 'static> {
    pub remove_callback: Callback<usize>,
    pub iterator: Vec<T>,
}

#[function_component(StaticEvent)]
pub fn static_event<T: ImportedEnum>(props: &StaticEventProps<T>) -> Html {
    html! {
        <div class={classes!(
            "grid", "gap-4", "grid-cols-8",
            "h-fit", "w-fit"
        )}>
            {
                for props.iterator
                    .iter()
                    .enumerate()
                    .map(|(index, id)| {
                        html! {
                            <button
                                class={classes!(
                                    "items-center", "gap-2", "text-sm",
                                    "select-none", "border", "relative",
                                    "_border-700", "cursor-pointer"
                                )}
                                onclick={{
                                    let remove_callback = props.remove_callback.clone();
                                    Callback::from(move |_| {
                                        remove_callback.emit(index);
                                    })
                                }}
                            >
                                <Image
                                    class={classes!("h-10", "w-10")}
                                    source={id.into_image_type()}
                                />
                            </button>
                        }
                })
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct StaticSelectorProps<T: PartialEq> {
    pub callback: Callback<T>,
    pub node_ref: NodeRef,
}

#[derive(Clone, PartialEq)]
struct StaticSelectorItem {
    index: usize,
    name: &'static str,
    html: Html,
}

#[function_component(StaticSelector)]
pub fn static_selector<T: ImportedEnum>(props: &StaticSelectorProps<T>) -> Html {
    let search_query = use_state(String::new);
    let id_to_name = T::ID_TO_NAME;
    let oninput = {
        let search_query = search_query.clone();
        use_callback((), move |e: InputEvent, _| {
            let input = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            search_query.set(input);
        })
    };

    let all_values = use_memo((), |_| {
        id_to_name
            .iter()
            .enumerate()
            .map(|(index, &name)| {
                let html = html! {
                    <button
                        class={classes!(
                            "items-center", "gap-2", "text-sm",
                            "select-none", "border", "relative",
                            "_border-700", "cursor-pointer"
                        )}
                        onclick={{
                            let callback = props.callback.clone();
                            Callback::from(move |_| {
                                callback.emit(T::from_usize_unchecked(index));
                            })
                        }}
                    >
                        <Image
                            source={T::into_image_type_unchecked(index)}
                            class={classes!("h-10", "w-10")}
                        />
                    </button>
                };
                StaticSelectorItem { index, name, html }
            })
            .collect::<Vec<_>>()
    });

    let visible_values = all_values
        .iter()
        .filter(|value| {
            value
                .name
                .to_lowercase()
                .contains(&search_query.to_lowercase())
        })
        .collect::<Vec<_>>();

    let random_id = RandomInput::rand_id();

    html! {
        <div
            ref={props.node_ref.clone()}
            class={classes!(
                "fixed", "top-1/2", "left-1/2", "-translate-x-1/2", "-translate-y-1/2",
                "z-50", "max-h-96", "overflow-y-auto", "bg-[#121214]",
                "flex", "flex-col", "gap-4", "h-full"
            )}
        >
            <label
                for={&random_id}
                class={classes!(
                    "bg-[#1f1f25]", "flex", "items-center", "gap-2",
                    "_text-200", "pl-10", "pr-4", "relative",
                )}
            >
                <span class={classes!("absolute", "left-4", "_text-400")}>
                    {svg!("../../../public/svgs/search", "14")}
                </span>
                <input
                    id={random_id}
                    type={"text"}
                    class={classes!(
                        "text-white", "focus:outline-none", "w-full", "ml-1",
                        "h-10", "bg-transparent",
                    )}
                    value={(*search_query).clone()}
                    placeholder={"Search by name"}
                    oninput={oninput}
                />
            </label>
            <div class={classes!("grid", "gap-4", "grid-cols-12")}>
                {for visible_values.iter().map(|item| item.html.clone())}
            </div>
        </div>
    }
}
