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
//use tracing_subscriber;

//use shardbearer_proto::herald_grpc::*;

use shardbearer_proto::herald::herald::*; //{Feature, Point, Rectangle, RouteSummary};

use shardbearer::herald::HeraldService;
use shardbearer_proto::herald::herald_grpc::{create_herald, Herald};

fn main() {
    tracing_subscriber::fmt::init();

    // let _guard = log_util::init_log(None);

    let env = Arc::new(Environment::new(2));
    let instance = HeraldService {
        //data: Arc::new(Vec::new()),
        //     received_notes: Arc::default(),
    };
    let service = create_herald(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    block_on(rx).unwrap();
    block_on(server.shutdown()).unwrap();
}
