


use shardbearer_proto::radiant::radiant::*;
use shardbearer_proto::radiant::radiant_grpc::RadiantNode as RadiantNodeRPC;
use shardbearer_proto::common::common::{Roles, Radiant as RadiantID, Beacon, BeaconResponse,
                                        Role,ConfigSummary,Controller,
                                        ShardMoveRequest,ShardMoveRequestResponse};

pub trait ShardSender {
    type KeyType;
    type ValueType;
    //this will set up an RPC client, set up a tokio runtime to handle the sending asynchronously
    fn setup_request(sid: u64, ip: String, port: u32, size: u64, key_type: Self::KeyType, val_type: Self::ValueType)->ShardMoveRequest;

}

