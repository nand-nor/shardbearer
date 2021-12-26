extern crate futures;
extern crate shardbearer_core;
extern crate shardbearer_proto;
extern crate tracing;
extern crate tracing_subscriber;

/*
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{
    ChannelBuilder, Environment, ResourceQuota, RpcContext, Server, ServerBuilder, UnarySink,
};*/

//#[cfg(not(feature = "client"))]
pub mod herald;
//#[cfg(not(feature = "client"))]
pub mod radiant;
pub mod shard;

pub mod config;
pub mod order;

//#[cfg(feature = "client")]
pub mod client;
pub mod handler;
pub mod rctrl;
pub mod service;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
