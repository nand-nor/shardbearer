use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{io, thread};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{error, info, span, trace, warn, Level};
use tracing_subscriber;

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{
    ChannelBuilder, Environment, ResourceQuota, RpcContext, Server, ServerBuilder,
    ServerStreamingSink, UnarySink, WriteFlags,
};

use indexmap::IndexMap;

//use shardbearer_core::radiant::{GroupID, MemberID, Membership};
//use shardbearer_core::membership::{RadiantMembership, Role as RadiantRole};
//use shardbearer_core::order::{Order, OrderShardAction, RadiantOrderState};
//use shardbearer_core::radiant::{Radiant};
//use shardbearer_core::shard::{Shard, ShardKey};
//use shardbearer_core::system::{RadiantSystem, SysState, System};

//use shardbearer::order::RadiantOrder;
use shardbearer::rsvc::RadiantService;
use shardbearer_core::shard::ShardEntry;
use shardbearer_proto::common::common::{Beacon, OrderId, Order};

use shardbearer_proto::radiant::radiant_grpc::{RadiantRpc, RadiantRpcClient};

#[tracing::instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Test client running");
    let env = Arc::new(Environment::new(2));
    let channel = ChannelBuilder::new(env).connect("127.0.0.1:50051");

    let mut client = RadiantRpcClient::new(channel);

    perform_handshake(&client).await?;
    i_can_has_list(&client).await?;

    Ok(())
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

async fn i_can_has_list(client: &RadiantRpcClient) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Async perform order member request");
    let mut order = OrderId::new();
    order.set_gid(0);

    let res = client.current_order_membership_async(&order)?.await?;
    //   res.await?;

    for bonk in res.members.iter().map(|x| x.clone()) {
        info!(
            "Order results: {:?}", bonk
        );
    }
    Ok(())
}
