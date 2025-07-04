use crate::components::sidebar::Sidebar;
use yew::{Html, function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div style="display:flex; height:100vh;">
            <Sidebar />
            <div style={"
                flex: 1;
                background-color: #2e2e3e;
                color: #ffffff;
                padding: 20px;
                overflow: auto;
                box-sizing: border-box;
            "}>
                <h2>{"Sample Data Table"}</h2>
                <table style={"
                    width: 100%;
                    border-collapse: collapse;
                    margin-bottom: 40px;
                "}>
                    <thead>
                        <tr>
                            <th style={"border-bottom: 1px solid #555; padding: 8px;"}>{"Col 1"}</th>
                            <th style={"border-bottom: 1px solid #555; padding: 8px;"}>{"Col 2"}</th>
                            <th style={"border-bottom: 1px solid #555; padding: 8px;"}>{"Col 3"}</th>
                            <th style={"border-bottom: 1px solid #555; padding: 8px;"}>{"Col 4"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (1..=5).map(|i| html! {
                            <tr>
                                <td style={"padding: 8px; border-bottom: 1px solid #444;"}>{format!("Row {} Col 1", i)}</td>
                                <td style={"padding: 8px; border-bottom: 1px solid #444;"}>{format!("Row {} Col 2", i)}</td>
                                <td style={"padding: 8px; border-bottom: 1px solid #444;"}>{format!("Row {} Col 3", i)}</td>
                                <td style={"padding: 8px; border-bottom: 1px solid #444;"}>{format!("Row {} Col 4", i)}</td>
                            </tr>
                        })}
                    </tbody>
                </table>

                <h2>{"Another Table"}</h2>
                <table style={"
                    width: 50%;
                    border-collapse: collapse;
                "}>
                    <thead>
                        <tr>
                            <th style={"border-bottom: 1px solid #555; padding: 8px;"}>{"A"}</th>
                            <th style={"border-bottom: 1px solid #555; padding: 8px;"}>{"B"}</th>
                            <th style={"border-bottom: 1px solid #555; padding: 8px;"}>{"C"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (1..=3).map(|i| html! {
                            <tr>
                                <td style={"padding: 8px; border-bottom: 1px solid #444;"}>{format!("{}1", i)}</td>
                                <td style={"padding: 8px; border-bottom: 1px solid #444;"}>{format!("{}2", i)}</td>
                                <td style={"padding: 8px; border-bottom: 1px solid #444;"}>{format!("{}3", i)}</td>
                            </tr>
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
