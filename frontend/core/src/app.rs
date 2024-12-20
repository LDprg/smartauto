use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/js/interface.js")]
extern "C" {
    fn runWasm();
}

#[function_component(App)]
pub fn app() -> Html {
    runWasm();

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
