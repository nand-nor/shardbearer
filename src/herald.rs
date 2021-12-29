use shardbearer_core::herald::Herald;
use shardbearer_core::radiant::MemberID;
use tracing::{debug, error, info, trace, warn};
use shardbearer_proto::common::common::HeraldInfo;

use indexmap::IndexMap;
use raft::eraftpb::Message;


pub struct HeraldMsg {
    pub hid: HeraldInfo,
    pub msg: Message,
}

pub struct CtrlHeraldMsg {
    pub hid: HeraldInfo,
    pub msg: Message,
}

pub struct HeraldService {
    peers_tx: IndexMap<MemberID, tokio::sync::mpsc::UnboundedSender<HeraldMsg>>,
    peers_rx: IndexMap<MemberID, tokio::sync::mpsc::UnboundedReceiver<HeraldMsg>>,
    controller_rx: tokio::sync::mpsc::UnboundedReceiver<CtrlHeraldMsg>,
    controller_tx: tokio::sync::mpsc::UnboundedSender<CtrlHeraldMsg>,
    controller: MemberID,
}

pub struct ControllerHeraldService {
    heralds_tx: IndexMap<MemberID, tokio::sync::mpsc::UnboundedSender<CtrlHeraldMsg>>,
    heralds_rx: IndexMap<MemberID, tokio::sync::mpsc::UnboundedReceiver<CtrlHeraldMsg>>,
}

impl Herald for HeraldService {
    type ControllerId = MemberID;

    fn elect_controller(&mut self) -> Self::ControllerId {
        0
    }
    fn controller(&mut self) -> Self::ControllerId {
        0
    }
}
