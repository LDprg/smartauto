use std::{env, path::PathBuf};

use glob::glob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let files: Vec<PathBuf> = glob("proto/smartauto/v1/*.proto")?
        .filter_map(Result::ok)
        .collect();

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("smartauto_v1_descriptor.bin"))
        .compile_protos(&files[..], &["proto"])?;
    Ok(())
}
