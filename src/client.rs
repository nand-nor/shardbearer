use grpcio::{ChannelBuilder, Environment};
use std::sync::Arc;
use tracing::{info, trace};

use crate::rctrl::StateMessage;
use shardbearer_proto::common::common::{Beacon, BeaconResponse};
use shardbearer_proto::radiant::radiant_grpc::RadiantRpcClient;

pub async fn perform_handshake_async(
    client: &RadiantRpcClient,
) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn handshake(
    neighbor_ip: String,
    neighbor_port: u16,
    tx: tokio::sync::mpsc::UnboundedSender<StateMessage>, //tokio::sync::mpsc::UnboundedSender<BeaconResponse>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::trace!("Executing bootstrap! Creating client for handshake");

    let env = Arc::new(Environment::new(2));
    let channel = ChannelBuilder::new(env)
        .connect(format!("{}:{}", neighbor_ip.clone(), neighbor_port.clone()).as_str());

    let mut client = RadiantRpcClient::new(channel);

    let resp = perform_handshake(&client)?;

    match tx.send(StateMessage::INITSTATE(resp.clone())) {
        Ok(_) => {
            tracing::trace!("End bootstrap");
        }
        Err(_) => {
            tracing::error!("Error sending on oneshot channel");
        }
    };

    Ok(())
}

pub fn perform_handshake(
    client: &RadiantRpcClient,
) -> Result<BeaconResponse, Box<dyn std::error::Error>> {
    let mut beacon = Beacon::new();
    beacon.set_mid(100);
    let res = client.beacon_handshake(&beacon)?;

    info!(
        "beacon is {:?} gid is {:?} {:?}",
        beacon.get_mid(),
        res.get_gid(),
        res
    );

    Ok(res)
}
