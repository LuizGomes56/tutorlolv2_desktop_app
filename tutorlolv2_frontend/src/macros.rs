#[macro_export]
macro_rules! svg {
    ($path:literal, $size:expr) => {{
        let content = include_str!(concat!($path, ".svg"));
        yew::virtual_dom::VNode::from_html_unchecked(content.replace("{size}", $size).into())
    }};
}

#[macro_export]
macro_rules! color {
    ($property:ident-$weight:literal) => {
        concat!(stringify!($property), "-", "zinc", "-", $weight)
    };
    ($property:ident-$alignment:ident-$weight:literal) => {
        concat!(
            stringify!($property),
            "-",
            stringify!($alignment),
            "-",
            "zinc",
            "-",
            $weight
        )
    };
    ($modifier:ident:$property:ident-$weight:literal) => {
        concat!(
            stringify!($modifier),
            ":",
            stringify!($property),
            "-",
            "zinc",
            "-",
            $weight
        )
    };
}
