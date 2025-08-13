use crate::{
    color,
    components::{Image, ImageType, calculator::StaticIterator},
    hooks::mouseout::use_mouseout,
    svg,
    utils::UnsafeCast,
};
use generated_code::{ITEM_ID_TO_NAME, RUNE_ID_TO_NAME};
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html,
    use_callback, use_memo, use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct U32SelectorProps<T: PartialEq + UnsafeCast + 'static> {
    pub static_iter: StaticIterator,
    pub callback: Callback<T>,
    pub current_value: T,
}

#[derive(Clone, PartialEq)]
struct U32Item {
    index: usize,
    name: &'static str,
    html: Html,
}

#[function_component(U32Selector)]
pub fn u32_selector<T>(props: &U32SelectorProps<T>) -> Html
where
    T: PartialEq + UnsafeCast + Copy + 'static,
    T::Repr: TryInto<usize> + TryFrom<usize>,
    ImageType: From<T>,
{
    let is_open = use_state(|| false);
    let search_query = use_state(|| String::new());
    let id_to_name: &[&'static str] = match props.static_iter {
        StaticIterator::Items => &ITEM_ID_TO_NAME,
        StaticIterator::Runes => &RUNE_ID_TO_NAME,
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
        id_to_name
            .into_iter()
            .enumerate()
            .map(|(index, &name)| {
                let html = html! {
                    <U32Options<T>
                        static_iter={props.static_iter}
                        key={index}
                        callback={callback}
                        value_id={T::from_usize_unchecked(index)}
                    />
                };
                U32Item { index, name, html }
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
                    placeholder={*id_to_name.get(T::into_usize_unchecked(props.current_value)).unwrap_or(&"Unknown")}
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
pub struct U32OptionsProps<T: PartialEq + UnsafeCast + 'static> {
    pub static_iter: StaticIterator,
    pub callback: Callback<T>,
    pub value_id: T,
}

#[function_component(U32Options)]
fn u32_options<T>(props: &U32OptionsProps<T>) -> Html
where
    T: Copy + PartialEq + UnsafeCast + 'static,
    T::Repr: TryInto<usize>,
    ImageType: From<T>,
{
    let id_to_name: &[&'static str] = match props.static_iter {
        StaticIterator::Items => &ITEM_ID_TO_NAME,
        StaticIterator::Runes => &RUNE_ID_TO_NAME,
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
            <Image class={classes!("w-5", "h-5")} source={ImageType::from(props.value_id)} />
            <span>{id_to_name.get(T::into_usize_unchecked(props.value_id)).unwrap_or(&"Unknown")}</span>
        </button>
    }
}
