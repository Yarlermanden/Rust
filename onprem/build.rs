fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .compile(
        &["protos/on_prem.proto"],
        &["protos"],
    ).unwrap();
    Ok(())
}