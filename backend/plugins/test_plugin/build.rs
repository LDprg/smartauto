fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../../proto/helloworld/v1/helloworld.proto")?;
    Ok(())
}
