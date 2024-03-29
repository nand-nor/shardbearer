use std::io;
use std::io::Read;
use std::sync::{Arc, Mutex};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{
    Environment,
    RpcContext,
    ServerBuilder,
    UnarySink,
};
use protobuf;
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber;

use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, ConfigId, ConfigSummary, Bondsmith, HeraldInfo, JoinGroup, LeaveGroup,
    Order, OrderId, Radiant as RadiantID, Role, Roles, ShardMoveRequest, ShardMoveRequestResponse,
};

use crate::rsvc::RadiantService;

use crate::config::ShardbearerConfig;
use crate::rctrl::{RadiantCtrl};
use crate::rpc_cli_handler::RadiantRpcClientHandler;

use shardbearer_core::consensus::ShardbearerConsensus;
use shardbearer_core::consensus::ShardbearerReplication;
use shardbearer_core::order::OrderState;
use shardbearer_core::radiant::{Radiant, RadiantNode};
use shardbearer_core::shard::{ShardHashMap, ShardMap};
use shardbearer_core::sys::SysState;//RadiantSystem;

use shardbearer_core::radiant::{RadiantStateMachine, RadiantState};



use shardbearer_proto::bondsmith::bondsmith_grpc::BondsmithRpc;
use shardbearer_proto::herald::herald_grpc::HeraldRpc;
use shardbearer_proto::radiant::radiant_grpc::{create_radiant_rpc, RadiantRpc};
use std::convert::TryInto;

use tokio::sync::mpsc::unbounded_channel;

use shardbearer_core::shard::ShardbearerMessage;

/// Users can call this as the entry point or optionally
/// define a custom entry point that calls radiant_server()
/// see the custom-entrypoint example
pub fn server_main<C: ShardbearerConsensus, R: ShardbearerReplication, K: 'static + std::hash::Hash + Eq + Clone, V: 'static + Clone, M: ShardbearerMessage>(
) -> Result<(), Box<dyn std::error::Error>> {

    //TODO make this a verbose mode feature config
    std::env::set_var("RUST_LOG", "info,warn,debug,trace,error");
    let sub = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(sub)
        .expect("Error setting default global trace subscriber");

    if let Some(toml) = std::env::args().nth(1) {
        let cfg = match crate::config::parse_cfg(&toml) {
            Ok(cfg) => cfg,
            Err(e) => {
                tracing::error!(
                    "RadiantServer: Invalid config file provided: {:?}",
                    e.to_string()
                );
                std::process::exit(1);
            }
        };
        let tokio_cfg = cfg.runtime_cfg();

        let rt = match tokio::runtime::Builder::new_multi_thread()
            .worker_threads(tokio_cfg.num_threads())
            .thread_stack_size(tokio_cfg.thread_stack_size())
            .thread_name(tokio_cfg.runtime_name())
            .enable_all()
            .build()
        {
            Ok(t) => t,
            Err(e) => {
                tracing::error!(
                    "RadiantServer: Unable to build requested tokio runtime: {:?}",
                    e.to_string()
                );
                std::process::exit(1);
            }
        };
        rt.block_on(async move {
            if let Err(e) = radiant_server::<C, R, K, V, M>(cfg).await {
                tracing::error!("RadiantServer: error running server {:?}", e.to_string());
            }
        });
    } else {
        tracing::error!("RadiantServer: No config file provided");
        std::process::exit(1);
    }
    Ok(())
}

#[tracing::instrument]
pub async fn radiant_server<C: ShardbearerConsensus, R: ShardbearerReplication, K: 'static + std::hash::Hash + Eq + Clone, V: 'static + Clone, M: ShardbearerMessage>(
    cfg: ShardbearerConfig,
) -> Result<(), Box<dyn std::error::Error>> {

    let env = Arc::new(Environment::new(2));
    let radiant_ip = cfg.my_ip();
    let radiant_port = cfg.my_port();
    let (ctrl_chan_tx, mut ctrl_chan_rx) = unbounded_channel();
    let (cmd_chan_tx, mut cmd_chan_rx) = unbounded_channel();
    let bootstrap_tx = ctrl_chan_tx.clone();
    let rpc_cli_tx = ctrl_chan_tx.clone();

    let shard_map: ShardHashMap<K, V> = ShardHashMap::new();
    let mut radiant_node: RadiantNode<K, C,R,M> = RadiantNode::default();
    radiant_node.set_cfg(&cfg);
    let arc_node = Arc::new(Mutex::new(radiant_node));

    let instance: RadiantService<C,R,K, V,M> = RadiantService::new(
        Arc::clone(&arc_node),
        Arc::new(Mutex::new(shard_map)),
        cfg.clone(),
        bootstrap_tx,
    );
    let controller_node = Arc::clone(&arc_node);
    let map_clone = instance.share_map();

    let rhndlr: &'static mut RadiantRpcClientHandler = Box::leak(Box::new(
        RadiantRpcClientHandler::new(cmd_chan_rx, rpc_cli_tx, Arc::clone(&arc_node)),
    ));
    let rctrl: &'static mut RadiantCtrl<K, C, R,M> = Box::leak(Box::new(RadiantCtrl::new(
        ctrl_chan_rx,
        ctrl_chan_tx,
        cmd_chan_tx,
        Arc::clone(&arc_node), //controller_node,
    )));

    let service = create_radiant_rpc(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind(radiant_ip, radiant_port)
        .build()?;
       // .unwrap();

    server.start();

    tokio::spawn(async move {
        if let Err(_) = rctrl.run(cfg).await {
            error!("Error running control thread")
        }
    });

    tokio::spawn(async move {
        if let Err(_) = rhndlr.run().await {
            error!("Error running client handler thread")
        }
    });

    for (host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }

    //TODO leave this in for easy testing for now. Later make a feature for specific test flags
    let (tx_break, rx) = oneshot::channel();
    tokio::spawn(async move {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx_break.send(())
    });

    block_on(rx)?;
    block_on(server.shutdown())?;
    Ok(())
}
