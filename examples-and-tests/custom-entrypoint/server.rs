use shardbearer;

use tokio;
use tracing;

//this illustrates a very simple custom entrypoint
//instead of using the one provided
#[tracing::instrument]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    if let Some(toml) = std::env::args().nth(1) {
            let cfg = match shardbearer::config::parse_cfg(&toml) {
                Ok(cfg) => cfg,
                Err(e) => {
                    tracing::error!("RadiantServer: Invalid config file provided: {:?}", e.to_string());
                    std::process::exit(1);
                }
            };

        shardbearer::server::radiant_server::<u64,u64>(cfg).await?;
    } else {
        tracing::error!("No config file provided");
        std::process::exit(1);
    }

    Ok(())
}

