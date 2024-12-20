// JS wrapper for simplifing Rust access to extism

import { createPlugin } from "/js/mod.js";

export async function runWasm() {
    const plugin = await createPlugin(
        document.location.origin + "/wasm/test_plugin.wasm",
        { useWasi: true }
    );

    let out = await plugin.call("getHtml", "Yellow, World!");
    console.log(out.text());
    return String(out.text());
};
