use crate::{
    components::{Image, ImageType},
    hooks::mouseout::use_mouseout,
    svg,
    utils::{ImportedEnum, RandomInput, UnsafeCast},
};
use yew::{
    Callback, Html, InputEvent, Properties, TargetCast, classes, function_component, html,
    use_callback, use_memo, use_node_ref, use_state,
};

#[derive(Properties, PartialEq)]
pub struct SelectorProps<T: PartialEq + UnsafeCast + 'static> {
    pub callback: Callback<T>,
    pub current_value: T,
}

#[derive(Clone, PartialEq)]
struct SelectorItem {
    index: usize,
    name: &'static str,
    html: Html,
}

#[function_component(Selector)]
pub fn selector<T>(props: &SelectorProps<T>) -> Html
where
    T: PartialEq + UnsafeCast + Copy + ImportedEnum + 'static,
    T::Repr: TryInto<usize> + TryFrom<usize>,
    ImageType: From<T>,
{
    let is_open = use_state(|| false);
    let search_query = use_state(|| String::new());
    let id_to_name = T::ID_TO_NAME;
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
                    <SelectorOptions<T>
                        key={index}
                        callback={callback}
                        value_id={T::from_usize_unchecked(index)}
                    />
                };
                SelectorItem { index, name, html }
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
        <div class={classes!("relative")}>
            <label
                for={&random_id}
                ref={label_ref}
                class={classes!(
                    "bg-[#1f1f25]", "hover:_bg-950",
                    "flex", "items-center", "gap-2", "h-10",
                    "_text-200", "pl-10", "pr-4",
                    "relative", "rounded-md"
                )}
            >
                <span class={classes!("absolute", "left-4", "_text-400")}>
                    {svg!("../../public/svgs/search", "14")}
                </span>
                <input
                    id={random_id}
                    type={"text"}
                    class={classes!(
                        "text-white", "focus:outline-none", "w-full", "ml-1", "bg-transparent"
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
                    "flex-col", "_bg-900",
                    "max-h-64", "overflow-y-auto", "z-10",
                    if *is_open { "flex" } else { "hidden" }
                )
            }>
                {for visible_values.iter().map(|item| item.html.clone())}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectorOptionsProps<T: PartialEq + UnsafeCast + 'static> {
    pub callback: Callback<T>,
    pub value_id: T,
}

#[function_component(SelectorOptions)]
fn selector_options<T>(props: &SelectorOptionsProps<T>) -> Html
where
    T: Copy + PartialEq + ImportedEnum + UnsafeCast + 'static,
    T::Repr: TryInto<usize>,
    ImageType: From<T>,
{
    let id_to_name = T::ID_TO_NAME;
    html! {
        <button
            class={classes!(
                "p-1", "cursor-pointer", "hover:_bg-800",
                "flex", "items-center", "gap-2", "_text-200",
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
