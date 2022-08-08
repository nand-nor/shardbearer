use shardbearer_state::radiant::*;
use shardbearer_state::bondsmith::*;
use shardbearer_state::order::*;
use crate::sys::*;
use std::boxed::Box;


pub trait ShardbearerMessage {//: Sized {

}


/*
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
}*/