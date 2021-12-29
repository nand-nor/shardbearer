
use tracing::{debug, error, info, trace, warn};
use shardbearer_core::radiant::{
    GroupID, MemberID, Radiant, RadiantKey, RadiantRole, RadiantState, RadiantStateError,
    RadiantStateMachine,
};
use shardbearer_core::system::{RadiantSystem, System};
use indexmap::IndexMap;
use shardbearer_core::order::{OrderShardAction, RadiantOrderState};
use shardbearer_core::shard::ShardKey;
use shardbearer_core::system::SysState;
use shardbearer_proto::common::common::Radiant as RadiantId;
use shardbearer_core::raft::*;
use shardbearer_core::raft::RaftCfg;

use raft::eraftpb::Message;

pub type RadiantShardMap = Vec<ShardKey>;

use crate::herald::{ControllerHeraldService, HeraldService};
use crate::config::ShardbearerConfig;


pub struct RadiantMsg {
    pub rid: RadiantId,
    pub msg: Message,
}

pub struct RadiantNode {
    mid: MemberID,
    gid: GroupID,
    sys: RadiantSystem,
    state: RadiantState,
    role: RadiantRole,
    members_tx: IndexMap<MemberID, tokio::sync::mpsc::UnboundedSender<RadiantMsg>>,
    members_rx: IndexMap<MemberID, tokio::sync::mpsc::UnboundedReceiver<RadiantMsg>>,

    order_herald: MemberID,
    pub order_members: Vec<RadiantId>,
    shards: RadiantShardMap,
    order_state: RadiantOrderState,

    hsvc: Option<HeraldService>,
    ctrlsvc: Option<ControllerHeraldService>,
    raft: Option<RaftNode>,
}

impl RadiantNode {
    pub fn default() -> Self {
        Self {
            mid: 0,
            sys: RadiantSystem::default(),
            state: RadiantState::default(),
            role: RadiantRole::default(),
            gid: 0,
            members_tx: IndexMap::new(),
            members_rx: IndexMap::new(),
            order_herald: 0,
            order_members: Vec::new(),
            shards: RadiantShardMap::new(),
            order_state: RadiantOrderState::default(),
            hsvc: None,
            ctrlsvc: None,

            raft: None, //RaftNode::new()
                        //cluster:
        }
    }

    pub fn set_cfg(&mut self, cfg:  &ShardbearerConfig){
        self.mid = cfg.id();
    }
}

impl Radiant for RadiantNode {
    type Role = RadiantRole;
    type GroupId = GroupID;
    type MemberId = MemberID;

    type MemberDataTx = tokio::sync::mpsc::UnboundedSender<RadiantMsg>;
    type MemberDataRx = tokio::sync::mpsc::UnboundedReceiver<RadiantMsg>;
    type MemberListType = RadiantId;

    fn add_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
        tx: Self::MemberDataTx,
        rx: Self::MemberDataRx,
    ) -> Result<(), ()> {
        self.members_tx.entry(mid).or_insert(tx);
        self.members_rx.entry(mid).or_insert(rx);
        self.order_members.push(member_data);

        Ok(())
    }
    fn remove_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
    ) -> Result<Self::MemberId, ()> {
        Ok(0)
    }

    fn update_role(&mut self, role: Self::Role) {
        self.role = role;
    }

    fn role(&self) -> Self::Role {
        self.role.clone()
    }

    fn set_member_id(&mut self, mid: Self::MemberId) {
        self.mid = mid;
    }

    fn member_id(&self) -> Self::MemberId {
        self.mid
    }

    fn group_id(&self) -> Self::GroupId {
        self.gid
    }

    fn set_group_id(&mut self, gid: Self::GroupId) {
        self.gid = gid;
    }

    /*
        fn add_member_tx(&mut self, mid: Self::MemberId, data: Self::MemberDataTx) -> Result<(), ()> {
            self.members_tx.entry(mid).or_insert(data);
            Ok(())
        }

        fn add_member_rx(&mut self, mid: Self::MemberId, data: Self::MemberDataRx) -> Result<(), ()> {
            self.members_rx.entry(mid).or_insert(data);

            Ok(())
        }
    */

    fn herald(&self) -> Option<Self::MemberId> {
        None
    } //None if we are in a state where no herald is elected
    fn set_herald(&mut self, mid: Self::MemberId) {
        self.order_herald = mid;
    }

    fn update_shard_list(&mut self, action: OrderShardAction) {
        match action {
            OrderShardAction::ADD(key) => self.shards.push(key),
            OrderShardAction::REMOVE(key) => {
                //this is bad :(
                for i in &self.shards {}
            }
        }
    }

    fn reset_shards(&mut self) {
        self.shards.clear()
    }

    fn elect_herald(&mut self) {}
}

impl RadiantStateMachine for RadiantNode {
    type RadiantState = RadiantState;
    type SystemState = SysState;
    type ClusterState = RadiantOrderState;
    type OrderState = RadiantOrderState;

    fn order_state(&self) -> Self::OrderState {
        self.order_state.clone()
    }
    fn update_order_state(&mut self, state: Self::OrderState) {
        self.order_state = state
    }
    fn update_cluster_state(&mut self, state: Self::ClusterState) {
        self.update_order_state(state);
    }
    //TODO make an impl just for tracking cluster state
    fn cluster_state(&self) -> Self::ClusterState {
        self.order_state()
    }
    fn update_radiant_state(&mut self, state: Self::RadiantState) {
        self.state = state;
    }
    fn radiant_state(&self) -> Self::RadiantState {
        self.state.clone()
    }

    fn update_system_state(&mut self, state: Self::SystemState) {
        self.sys.update_state(state)
    }

    fn system_state(&self) -> Self::SystemState {
        self.sys.report_state()
    }
}
