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

use shardbearer_proto::herald::herald::*;
use shardbearer_proto::herald::herald_grpc::Herald;

use shardbearer_proto::common::common::{Radiant, Roles};
//use shardbearer_proto::herald::herald_grpc::Herald;

#[derive(Clone)]
pub struct HeraldService {}

impl Herald for HeraldService {
    fn herald_vote(&mut self, ctx: RpcContext<'_>, radiant: Radiant, sink: UnarySink<Roles>) {
        let resp = Roles::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
}
