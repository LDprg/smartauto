use extism::*;

fn main() {
    let file = Wasm::file("./test_plugin.wasm");
    let manifest = Manifest::new([file]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    let res = plugin.call::<&str, &str>("greet", "Hello, world!").unwrap();
    println!("{}", res);
}
