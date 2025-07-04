use yew::{Html, function_component, html};

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let menu_items = vec!["Home", "Realtime", "Calculator", "Formulas", "Source Code"];
    html! {
        <div style={"
            background-color: #1e1e2f;
            color: #ffffff;
            width: 220px;
            height: 100vh;
            padding: 20px;
            box-sizing: border-box;
        "}>
            <h1 style={"font-size: 1.5em; margin-bottom: 1em;"}>{"Menu"}</h1>
            <ul style={"list-style: none; padding: 0;"}>
                { for menu_items.iter().map(|item| html! {
                    <li style={"
                        padding: 10px 0;
                        cursor: pointer;
                        border-radius: 4px;
                        margin-bottom: 4px;
                        transition: background-color 0.2s;
                    "}>
                        { *item }
                    </li>
                })}
            </ul>
        </div>
    }
}
