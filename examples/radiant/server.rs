use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{io, thread};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{
    ChannelBuilder, Environment, ResourceQuota, RpcContext, Server, ServerBuilder,
    ServerStreamingSink, UnarySink, WriteFlags,
};

use tracing::{error, info, span, warn, Level};
use tracing_subscriber;

//use shardbearer_proto::herald_grpc::*;
use indexmap::IndexMap;

use shardbearer_proto::herald::herald::*;

use shardbearer::herald::HeraldService;
use shardbearer_proto::herald::herald_grpc::{create_herald, Herald};

use shardbearer_core::membership::{GroupID, MemberID, Membership};
use shardbearer_core::membership::{RadiantMembership, Role as RadiantRole};
use shardbearer_core::order::{Order, OrderShardAction, RadiantOrderState};
use shardbearer_core::radiant::{Radiant, RadiantNode};
use shardbearer_core::shard::{Shard, ShardHashMap, ShardKey};
use shardbearer_core::system::{RadiantSystem, SysState, System};

use shardbearer::order::RadiantOrder;
use shardbearer::radiant::RadiantService;
use shardbearer_core::shard::ShardEntry;

use chrono::*;
use shardbearer_proto::common::common::BeaconResponse;
use shardbearer_proto::radiant::radiant_grpc::{
    create_radiant_node, RadiantNode as RadiantNodeRPC,
};
use timer::*;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
    /*
     let env = Arc::new(Environment::new(2));

     let radiant_ip = cfg.my_ip();
     let radiant_port = cfg.my_port();

     let neighbor_ip = cfg.neighbor_ip();
     let neighbor_port = cfg.neighbor_port();
     let bootstrap_backoff = cfg.bootstrap_backoff();

     let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
     let tx2 = tx.clone();
    // let rx2 = rx.clone();
     //  let (mut send, mut rcv) = tokio::sync::oneshot::channel();

     let mut shard_map: ShardHashMap<u64, u64> = ShardHashMap::new();

     let mut radiant_node: RadiantNode<RadiantOrder, RadiantMembership, RadiantSystem> =
         RadiantNode::<RadiantOrder, RadiantMembership, RadiantSystem,>::default();

     let arc_node = Arc::new(Mutex::new(radiant_node));
    // let arc_map = Arc::new(Mutex::new(shard_map));
     let mut instance: RadiantService<u64, u64> = RadiantService::new(
         Arc::clone(&arc_node),
         Arc::new(Mutex::new(shard_map)),//Arc::clone(&arc_map as _),
         cfg,
         tx,
         //rx,
     );

     let map_clone = instance.share_map();

     let mut backoff: u64 = StdRng::seed_from_u64(bootstrap_backoff).gen();
     let timer = Timer::new();

     backoff = backoff % 100;
     tracing::trace!(
         "Bootstrap backoff time: {:?} ip {:?} port {:?}",
         backoff,
         neighbor_ip,
         neighbor_port
     );
     //let rt = tokio::runtime::Runtime::new()
     //    .unwrap();
     let ip = radiant_ip.clone();
     let port = radiant_port.clone();
     tracing::trace!("Executing bootstrap! p1");

     let g = timer.schedule_repeating(chrono::Duration::milliseconds(backoff as _), move || {
         shardbearer::client::handshake(
             ip.clone(),
             port.clone(),
             neighbor_ip.clone(),
             neighbor_port.clone(),
         tx2.clone(),
         );
     });

     let service = create_radiant_node(instance);
     let mut server = ServerBuilder::new(env)
         .register_service(service)
         .bind(radiant_ip, radiant_port)
         .build()
         .unwrap();
     server.start();

     //    drop(g);

     for (host, port) in server.bind_addrs() {
         info!("listening on {}:{}", host, port);
     }

     tokio::spawn(async move {
         //  loop {
         tracing::trace!("Waiting to drop");

         match rx.recv().await {
             Some(v) => {
                 tracing::trace!("got = {:?}", v);
                 drop(g);
                 //   break;
             }
             None => tracing::trace!("the sender dropped"),
         };
         //   break;
         //}
     }); //.await;
     let (tx_break, rx) = oneshot::channel();
     tokio::spawn(async move {
         info!("Press ENTER to exit...");
         let _ = io::stdin().read(&mut [0]).unwrap();
         tx_break.send(())
     });

     block_on(rx).unwrap();
     block_on(server.shutdown()).unwrap();*/
    Ok(())
}
