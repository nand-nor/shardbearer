extern crate futures;
extern crate protobuf;
extern crate shardbearer_core;
extern crate shardbearer_proto;
extern crate tracing;
extern crate tracing_subscriber;

pub mod client;
pub mod config;
pub mod herald;
pub mod radiant;

//radiant handler for handling commands
pub mod rhndlr;

//radiant controller for responding to RPCs and propagating
// the needed comands to the handler
pub mod rctrl;

//radiant service impls for all RPC service defines in shardbearer-proto
pub mod rsvc;


//top level server function calls to create & run all needed objects
//within a tokio runtime
pub mod server;