use std::io::Read;
use std::sync::{Arc, Mutex};
//use std::time::Instant;
use std::{io, thread};

use futures::channel::oneshot;
use futures::executor::block_on;
//use futures::prelude::*;
use grpcio::{
    //ChannelBuilder,
    Environment, //ResourceQuota, RpcContext, Server,
    ServerBuilder,
    // ServerStreamingSink, UnarySink, WriteFlags,
};

use tracing::{debug, error, info, warn};
//use tracing_subscriber;

//use shardbearer_proto::herald_grpc::*;
//use indexmap::IndexMap;

//use shardbearer_proto::herald::herald::*;

//use crate::herald::HeraldService;
//use shardbearer_proto::herald::herald_grpc::{create_herald, Herald};

//use shardbearer_core::membership::{GroupID, MemberID, Membership};
use shardbearer_core::membership::RadiantMembership;
//use shardbearer_core::order::{Order, OrderShardAction, RadiantOrderState};
use shardbearer_core::radiant::RadiantNode;
use shardbearer_core::shard::ShardHashMap;
use shardbearer_core::system::RadiantSystem;

use crate::order::RadiantOrder;
use crate::radiant::RadiantService;
//use shardbearer_core::shard::ShardEntry;

use chrono::*;
use shardbearer_proto::common::common::BeaconResponse;
use shardbearer_proto::radiant::radiant_grpc::{
    create_radiant_node, RadiantNode as RadiantNodeRPC,
};
use timer::*;

use crate::rctrl::RadiantController;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[tracing::instrument]
pub async fn radiant_server(toml: &str) -> Result<(), Box<dyn std::error::Error>> {
    //tracing_subscriber::fmt::init();

    let cfg = match crate::config::parse_cfg(&toml) {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("Invalid config file provided: {:?}", e.to_string());
            std::process::exit(1);
        }
    };

    let env = Arc::new(Environment::new(2));

    let radiant_ip = cfg.my_ip();
    let radiant_port = cfg.my_port();

    let (ctrl_chan_tx, mut ctrl_chan_rx) = tokio::sync::mpsc::unbounded_channel();

    let (tx, mut bootstrap_rx) = tokio::sync::mpsc::unbounded_channel();
    let bootstrap_tx = ctrl_chan_tx.clone();

    let mut shard_map: ShardHashMap<u64, u64> = ShardHashMap::new();

    let mut radiant_node: RadiantNode<RadiantOrder, RadiantMembership, RadiantSystem> =
        RadiantNode::<RadiantOrder, RadiantMembership, RadiantSystem>::default();

    let arc_node = Arc::new(Mutex::new(radiant_node));
    // let arc_map = Arc::new(Mutex::new(shard_map));
    let mut instance: RadiantService<u64, u64> = RadiantService::new(
        Arc::clone(&arc_node),
        Arc::new(Mutex::new(shard_map)),
        cfg.clone(),
        tx,
    );
    let controller_node = Arc::clone(&arc_node);
    let map_clone = instance.share_map();

    let mut rctrl: &'static mut RadiantController = Box::leak(Box::new(RadiantController::new(
        ctrl_chan_rx,
        ctrl_chan_tx,
        controller_node,
    )));

    let service = create_radiant_node(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind(radiant_ip, radiant_port)
        .build()
        .unwrap();
    server.start();

    rctrl
        .run(
            cfg,
        )
        .await;

    for (host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }

    let (tx_break, rx) = oneshot::channel();
    tokio::spawn(async move {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx_break.send(())
    });

    block_on(rx).unwrap();
    block_on(server.shutdown()).unwrap();
    Ok(())
}
