use std::path::PathBuf;

use glob::glob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<PathBuf> = glob("proto/smartauto/v1/*.proto")?
        .filter_map(Result::ok)
        .collect();

    tonic_build::configure().compile_protos(&files[..], &["proto"])?;

    Ok(())
}
