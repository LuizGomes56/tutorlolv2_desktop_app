#[macro_export]
macro_rules! svg {
    ($path:literal, $size:expr) => {{
        let content = include_str!(concat!($path, ".svg"));
        yew::virtual_dom::VNode::from_html_unchecked(content.replace("{size}", $size).into())
    }};
}

#[macro_export]
macro_rules! url {
    (@inner) => { "http://localhost:8082" };
    (static $path:expr) => {
        url!(@inner).to_owned() + $path
    };
    ($path:literal) => {
        concat!(url!(@inner), $path)
    };
    ($fmt:literal $(, $vars:expr)*) => {
        format!(
            concat!(url!(@inner), $fmt)
            $(, $vars)*
        )
    };
}
