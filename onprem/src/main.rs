use tokio_util::sync::CancellationToken;

mod scan_service;
mod datasources;

pub mod on_prem {
    tonic::include_proto!("on_prem");
}

use on_prem::ServerConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = CancellationToken::new();

    //need some way of reading the config from somewhere else
    let config = ServerConfig {
        company_id: "".to_string(),
        sleep_interval_sec: 30,
    };

    scan_service::run(config, token);
    Ok(())
}