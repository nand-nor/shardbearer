use shardbearer;

#[tracing::instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    if let Some(toml) = std::env::args().nth(1) {
        shardbearer::service::radiant_server(toml.as_str()).await?;
    } else {
        tracing::error!("No config file provided");
        std::process::exit(1);
    }

    Ok(())
}
