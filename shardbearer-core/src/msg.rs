use protobuf::Message;
use shardbearer_state::radiant::*;
use shardbearer_state::bondsmith::*;
use shardbearer_state::order::*;
use shardbearer_state::sys::*;
use std::boxed::Box;
use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, ConfigId, ConfigSummary, Bondsmith, HeraldInfo, JoinGroup, LeaveGroup,
    Order, OrderId, Radiant as RadiantID, Role, Roles, ShardMoveRequest, ShardMoveRequestResponse,
};

pub struct BondsmithMsg {
    pub hid: HeraldInfo,
    pub msg: dyn Message,
}

pub struct HeraldMsg {
    pub hid: HeraldInfo,
    pub msg: dyn Message,
}


pub struct RadiantMsg {
    pub rid: RadiantID,
    pub msg: dyn Message,
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
    //CLUSTERSTATE(RadiantOrderState), //TODO have dedicated cluster state
}