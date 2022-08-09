#![allow(dead_code)] //just while features are still being implemented
#![allow(unused_variables)] //^^
#![allow(clippy::result_unit_err)] //need to open git issue to impl custom error types

extern crate futures;
extern crate protobuf;
extern crate shardbearer_core;
extern crate shardbearer_proto;
extern crate tracing;
extern crate tracing_subscriber;

pub mod config;
pub mod rctrl;
pub mod rpc_cli_handler;
//radiant service impls for all RPC service defines in shardbearer-proto
pub mod msg;
pub mod rsvc;
//top level server function calls to create & run all needed objects
//within a tokio runtime
//pub mod server;

//pub mod utils;
