use crate::{
    color,
    components::{Image, ImageType, calculator::StaticIterator},
    hooks::mouseout::use_mouseout,
    svg,
};
use generated_code::{ITEM_ID_TO_NAME, ITEM_NAME_TO_ID, RUNE_ID_TO_NAME, RUNE_NAME_TO_ID};
use std::borrow::Cow;
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html,
    use_callback, use_memo, use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct U32SelectorProps {
    pub static_iter: StaticIterator,
    pub callback: Callback<u32>,
    pub current_value: u32,
}

#[derive(Clone, PartialEq)]
struct U32Item<'a> {
    name: Cow<'a, str>,
    html: Html,
}

#[function_component(U32Selector)]
pub fn u32_selector(props: &U32SelectorProps) -> Html {
    let is_open = use_state(|| false);
    let search_query = use_state(|| String::new());
    let (name_to_id, id_to_name) = match props.static_iter {
        StaticIterator::Items => (&ITEM_NAME_TO_ID, &ITEM_ID_TO_NAME),
        StaticIterator::Runes => (&RUNE_NAME_TO_ID, &RUNE_ID_TO_NAME),
    };
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

    let all_values = use_memo(callback, |callback| {
        name_to_id
            .entries()
            .enumerate()
            .map(|(index, (name, value_id))| {
                let html = html! {
                    <U32Options
                        static_iter={props.static_iter}
                        key={index}
                        callback={callback}
                        value_id={*value_id}
                    />
                };
                U32Item {
                    name: Cow::Borrowed(name),
                    html,
                }
            })
            .collect::<Vec<_>>()
    });

    let visible_champions = all_values
        .iter()
        .filter(|value| {
            value
                .name
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
                    placeholder={*id_to_name.get(&props.current_value).unwrap_or(&"Unknown")}
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
pub struct U32OptionsProps {
    pub static_iter: StaticIterator,
    pub callback: Callback<u32>,
    pub value_id: u32,
}

#[function_component(U32Options)]
fn u32_options(props: &U32OptionsProps) -> Html {
    let id_to_name = match props.static_iter {
        StaticIterator::Items => &ITEM_ID_TO_NAME,
        StaticIterator::Runes => &RUNE_ID_TO_NAME,
    };
    let source = match props.static_iter {
        StaticIterator::Items => ImageType::Items(props.value_id),
        StaticIterator::Runes => ImageType::Runes(props.value_id),
    };
    html! {
        <button
            class={classes!(
                "p-1", "cursor-pointer", color!(hover:bg-800),
                "flex", "items-center", "gap-2", color!(text-200),
                "text-sm", "select-none"
            )}
            onclick={{
                let value_id = props.value_id;
                props.callback.reform(move |_| value_id)
            }}
        >
            <Image class={classes!("w-5", "h-5")} source={source} />
            <span>{id_to_name.get(&props.value_id).unwrap_or(&"Unknown")}</span>
        </button>
    }
}
