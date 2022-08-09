use grpcio::{ChannelBuilder, Environment};
use shardbearer::rhndlr::RadiantRpcClientHandler;
use shardbearer_proto::common::common::Beacon;
use shardbearer_proto::radiant::radiant_grpc::RadiantRpcClient;
use std::sync::Arc;
use tracing::{debug, info, trace, warn};
use tracing_subscriber;
#[tracing::instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Test client running");
    let env = Arc::new(Environment::new(2));
    let channel = ChannelBuilder::new(env).connect("127.0.0.1:50051");

    let mut client = RadiantRpcClient::new(channel);

   // perform_handshake(&client).await?;

   // let mut other_clients: RadiantRpcClientHandler = RadiantRpcClientHandler::new();
    //#[cfg(feature = "client_tests")]
   // tests::check_rpc_cli_tests(&mut other_clients);

    Ok(())
}

/*
#[cfg(feature = "client_tests")]
mod tests {
    use super::RadiantRpcClientHandler;
    #[cfg(feature = "client_tests")]
    use shardbearer::rhndlr::tests;
    use tracing::{info, trace};
    pub fn check_rpc_cli_tests(cli: &mut RadiantRpcClientHandler) {
        info!("Runing check_rpc_cli functions...");

        tests::setup_tests();
        tests::drop_tests();
        tests::reset_tests();
    }

}

async fn perform_handshake(client: &RadiantRpcClient) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Async perform handshake...");
    let mut beacon = Beacon::new();
    beacon.set_mid(100);
    let res = client.beacon_handshake_async(&beacon)?.await?;
    //   res.await?;
    info!(
        "beacon is {:?} gid is {:?}",
        beacon.get_mid(),
        res.get_gid()
    );

    Ok(())
}
*/