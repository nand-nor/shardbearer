//use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use tracing::{info, trace}; //{error, info, span, trace, warn, Level};
use tracing_subscriber;

//use indexmap::IndexMap;

//use shardbearer_proto::herald::herald::{Feature, Point, Rectangle, RouteSummary};

//use shardbearer::herald::HeraldService;
//use shardbearer_proto::herald::herald_grpc::{create_herald, Herald};
/*
use shardbearer_core::membership::{GroupID, MemberID, Membership};
use shardbearer_core::membership::{RadiantMembership, Role as RadiantRole};
use shardbearer_core::order::{Order, OrderShardAction, RadiantOrderState};
use shardbearer_core::radiant::{Radiant, RadiantNode};
use shardbearer_core::shard::{Shard, ShardKey};
use shardbearer_core::system::{RadiantSystem, SysState, System};

use shardbearer::order::RadiantOrder;
use shardbearer::radiant::RadiantService;
use shardbearer_core::shard::ShardEntry;
*/

use grpcio::{ChannelBuilder, Environment};

use crate::rctrl::StateMessage;
use shardbearer_core::order::RadiantOrderState;
use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, ConfigSummary, Controller, Radiant as RadiantID, Role, Roles,
    ShardMoveRequest, ShardMoveRequestResponse,
};
use shardbearer_proto::radiant::radiant_grpc::{RadiantNode as RadiantNodeRPC, RadiantNodeClient};

pub async fn perform_handshake_async(
    client: &RadiantNodeClient,
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
    // my_ip: String,
    // my_port: u16,
    neighbor_ip: String,
    neighbor_port: u16,
    tx: tokio::sync::mpsc::UnboundedSender<StateMessage>, //tokio::sync::mpsc::UnboundedSender<BeaconResponse>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::trace!("Executing bootstrap! Creating client for handshake");

    let env = Arc::new(Environment::new(2));
    let channel = ChannelBuilder::new(env)
        .connect(format!("{}:{}", neighbor_ip.clone(), neighbor_port.clone()).as_str());

    let mut client = RadiantNodeClient::new(channel);

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
    client: &RadiantNodeClient,
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
