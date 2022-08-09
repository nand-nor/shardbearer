//use protobuf::Message;
use shardbearer_core::radiant::RadiantState;
use shardbearer_core::bondsmith::BondsmithState;
use shardbearer_core::order::OrderState;
use shardbearer_core::sys::SysState;
use std::boxed::Box;
use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, ConfigId, ConfigSummary, Bondsmith, HeraldInfo, JoinGroup, LeaveGroup,
    Order, OrderId, Radiant as RadiantID, Role, Roles, ShardMoveRequest, ShardMoveRequestResponse,
};

use shardbearer_core::shard::ShardbearerMessage;

pub struct BondsmithMsg {
    pub hid: HeraldInfo,
    pub msg: Box<dyn ShardbearerMessage>,
}

pub struct HeraldMsg {
    pub hid: HeraldInfo,
    pub msg: Box<dyn ShardbearerMessage>,
}


pub struct RadiantMsg {
    pub rid: RadiantID,
    pub msg: Box<dyn ShardbearerMessage>,
}

pub enum ClientCommand {
    PEER(Box<RadiantMsg>),
    HERALD(Box<HeraldMsg>),
    CTRL(Box<BondsmithMsg>),
}

pub struct Vote{}

pub enum StateMessage {
    INITSTATE(BeaconResponse),
    VOTESTATE(Vote),
    SYSSTATE(SysState),
    ORDERSTATE(OrderState),
    RADIANTSTATE(RadiantState),
    RADIANTROLE(super::RadiantRole),
    ORDERHERALDSTATE(super::HeraldRole),
    BONDSMITHSTATE(BondsmithState),
    CLUSTERSTATE(OrderState), //TODO have dedicated cluster state
}