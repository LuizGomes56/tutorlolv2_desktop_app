use crate::{
    color,
    components::Dropdown,
    context::{GlobalContextActions, HoverDocs, SettingsContext},
};
use yew::{Html, classes, function_component, html, use_callback, use_context};

#[function_component(Settings)]
pub fn settings() -> Html {
    let context = use_context::<SettingsContext>().expect("SettingsContext não encontrado");
    let hover_docs_callback = {
        let context = context.clone();
        use_callback((), move |v, _| {
            context.dispatch(GlobalContextActions::SetHoverDocs(v));
        })
    };

    html! {
        <div class={classes!(
            "flex", "gap-6", "flex-col",
            "p-6", "max-w-3xl", "mb-48"
        )}>
            <h1 class={classes!(
                "text-2xl", "sm:text-3xl", "font-medium"
            )}>
                { "Application Settings" }
            </h1>
            /*
            const ChangeThemeButton = () => {
                const { theme, toggleTheme } = useTheme();
                return (
                    <button
                        aria-label="toggle theme"
                        type="button"
                        className={`relative h-7 w-16 mx-6 rounded-full cursor-pointer transition-colors duration-300 shadow-inner ${theme ? "bg-gradient-to-r from-purple-600 to-indigo-500" : "bg-gradient-to-r from-teal-400 to-emerald-400"}`}
                        onClick={toggleTheme}
                        style={{ boxShadow: theme ? 'inset 0 0 4px rgba(79,70,229,0.7)' : 'inset 0 0 4px rgba(16,185,129,0.7)' }}
                    >
                        <span
                            className={`absolute top-0.5 left-0.5 w-6 h-6 rounded-full !transition-transform duration-300 flex items-center justify-center shadow-md
                            ${theme ? "transform translate-x-9 bg-indigo-600" : "bg-emerald-500"}`}
                            style={{ boxShadow: theme ? '0 2px 4px rgba(67,56,202,0.5)' : '0 2px 4px rgba(5,150,105,0.5)' }}
                        >
                            {theme
                                ? <RiMoonFill className="w-4 h-4 text-white" />
                                : <MdSunny className="w-4 h-4 text-white" />
                            }
                        </span>
                    </button>
                )
            }
            */
            <div class={classes!(
                "w-full", "border-b",
                color!(border-700),
                "pb-5", "flex", "flex-col", "gap-4"
            )}>
                <div class={classes!(
                    "flex", "flex-col", "gap-4"
                )}>
                    <h2>{ "Object-hover behavior" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p>{ "Choose how hover interactions reveal object information:" }</p>
                        <ol class="list-disc list-inside mt-2 space-y-1">
                            <li>
                                <strong>{ "None" }</strong>
                                { ": Disables nearly all hover effects." }
                            </li>
                            <li>
                                <strong>{ "Partial" }</strong>
                                { ": Displays only names, stats, and limited item information. Source code is hidden." }
                            </li>
                            <li>
                                <strong>{ "Full" }</strong>
                                { ": Reveals source code after 1 second, along with all available details." }
                            </li>
                        </ol>
                    </div>
                </div>
                <Dropdown<3, HoverDocs>
                    current_index={context.docs}
                    callback={hover_docs_callback.clone()}
                    name={"hover_attr"}
                    iterator={HoverDocs::to_array()}
                />
            </div>
        </div>
    }
}
