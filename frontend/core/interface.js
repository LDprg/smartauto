// JS wrapper for simplifing Rust access to extism

import { createPlugin } from "./mod.js";

export async function runWasm() {
    const plugin = await createPlugin(
        'https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm'
    );

    let out = await plugin.call("count_vowels", "Yellow, World!");
    console.log(out.text());
};