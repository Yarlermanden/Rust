fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true) //default true
        .build_client(true) //default true
        //.out_dir("src")
        .compile(
            &["protos/common/vector.proto",
            "protos/player_input/player.proto"],
            &["protos"],
        ).unwrap();
    Ok(())
}