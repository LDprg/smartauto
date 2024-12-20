// JS wrapper for simplifing Rust access to extism

import { createPlugin } from "/mod-ed1c65e98195f935.js";

export async function runWasm() {
    const plugin = await createPlugin(
        "http://127.0.0.1:8080/test_plugin.wasm",
        { useWasi: true }
    );

    let out = await plugin.call("greet", "Yellow, World!");
    console.log(out.text());
};