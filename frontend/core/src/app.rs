use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[wasm_bindgen(module = "/js/interface.js")]
extern "C" {
    fn runWasm() -> Promise;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_async(async move { JsFuture::from(runWasm()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    console_log!("running");

    html! {
        <main>
            <button {onclick} disabled={state.loading}>{ "Start loading" }</button>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            {
                if state.loading {
                    html! { "Loading" }
                } else {
                    html! {}
                }
            }
            {
                if let Some(data) = &state.data {
                    let data = data.as_string().unwrap();
                    html!{
                        <SafeHtml html={data}/>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &state.error {
                    let error = error.as_string().unwrap();
                    html! { error }
                } else {
                    html! {}
                }
            }
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
