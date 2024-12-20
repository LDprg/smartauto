use extism_pdk::*;

macro_rules! html{
    ($($x:tt)*) => {
        Ok(html_to_string_macro::html!($($x)*))
    };
}

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[plugin_fn]
pub fn getHtml(name: String) -> FnResult<String> {
    html!(
        <h1>{name}</h1>
    )
}
