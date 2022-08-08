use protobuf::Message;


pub struct BondsmithMsg {
    pub hid: HeraldInfo,
    pub msg: dyn Message,
}

pub struct HeraldMsg {
    pub hid: HeraldInfo,
    pub msg: dyn Message,
}


pub struct RadiantMsg {
    pub rid: RadiantId,
    pub msg: dyn Message,
}

pub enum ClientCommand {
    PEER(RadiantMsg),
    HERALD(HeraldMsg),
    CTRL(CtrlHeraldMsg),
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