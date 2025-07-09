use crate::{color, svg};
use yew::{Html, classes, function_component, html};

fn container<const N: usize>(
    title: &'static str,
    array: [&'static str; N],
    svg: Html,
    color: &'static str,
) -> Html {
    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-2",
        )}>
            <span class={classes!(
                "font-semibold", "text-white", "font-mono"
            )}>
                {title}
            </span>
            {
                for array
                .into_iter()
                .map(|name| {
                    html! {
                        <div class={classes!(
                            "flex", "items-center", "gap-2",
                            "font-semibold", "w-full", "px-2",
                            "cursor-pointer",
                            "transition-colors", "duration-300",
                        )}>
                            {svg.clone()}
                            <span class={classes!(
                                color, "text-sm", "truncate"
                            )}>
                                {name}
                            </span>
                        </div>
                    }
                })
            }
        </div>
    }
}

#[function_component(FormulaSidebar)]
pub fn formula_sidebar() -> Html {
    html! {
        <div class={classes!(
            "flex", "flex-col", "gap-4",
            "w-52", "border-r-2", "h-screen",
            color!(border-r-800), "p-4",
            "overflow-y-auto", "bg-neutral-900"
        )}>
            {container(
                "MACROS",
                [
                    "ability!",
                    "passive!",
                    "merge_damage!",
                    "merge_ability!",
                    "get!",
                    "insert!",
                ],
                html! {
                    <span class={classes!("text-[#569CD6]")}>
                        {svg!("../../../public/formulas/macro", "16")}
                    </span>
                },
                "text-[#569CD6]"
            )}
            {container(
                "FUNCTIONS",
                [
                    "extract_ability_damage",
                    "extract_damagelike_expr",
                    "extract_passive_damage",
                    "extract_scaled_values",
                    "process_damage_expr",
                    "process_scaled_value",
                ],
                html! {
                    <span class={classes!("text-purple-400")}>
                        {svg!("../../../public/formulas/function", "16")}
                    </span>
                },
                "text-[#DCDCAA]"
            )}
            {container(
                "STRUCTS",
                [
                    "Ability",
                    "CdnChampion",
                    "Champion",
                    "Target",
                    "FxHashMap",
                ],
                html! {
                    <span class={classes!("text-white")}>
                        {svg!("../../../public/formulas/struct", "16")}
                    </span>
                },
                "text-[#4EC9B0]"
            )}
        </div>
    }
}
