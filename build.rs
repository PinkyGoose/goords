fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("api/solar-system-info/solar-system-info.proto")?;
    tonic_build::compile_protos("api/api_v1_game.proto")?;
    Ok(())
}