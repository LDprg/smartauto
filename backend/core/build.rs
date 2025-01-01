use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let path = PathBuf::from(env::var("CARGO_WORKSPACE_DIR").unwrap());
    let tonic_builder = tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("smartauto_v1_descriptor.bin"));

    tonic_buf_build::compile_from_buf_with_config(
        tonic_builder,
        None,
        tonic_buf_build::TonicBufConfig {
            buf_dir: Some(path),
        },
    )?;
    Ok(())
}
