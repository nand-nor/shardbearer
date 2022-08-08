use crate::shard::{Shard, ShardKey, ShardEntry, ShardAction, ShardGroupKey,ShardbearerMessage, ShardLoad};
use crate::herald::{HeraldNode, Herald};
use crate::bondsmith::{BondsmithNode, Bondsmith};

use crate::order::OrderState;
use crate::sys::SysState;
use crate::consensus::{ShardbearerConsensus, ShardbearerReplication};

use tokio::sync::mpsc::{UnboundedSender,UnboundedReceiver};

use indexmap::IndexMap;
use tracing::{debug, error, info, trace, warn};

use super::{Timestamp, GroupID, RadiantID, MemberID};



#[derive(Clone, Hash, Debug, Eq, PartialEq)]
pub struct RadiantKey {
    pub gid: GroupID,
    pub mid: MemberID,
}

/// As a radiant, the server node has a base set of operations it
/// is responsible for performing. These operations are categorized into
/// as two trait bounds on the base `Radiant` trait: `RadiantGroupMgmt` and
/// `RadiantShardMgmt`. The two traits are meant to cover broadly the functions for
/// basic shard group membership management and replication data tracking. This is
/// necessary because we want to
/// increase fault tolerance, and enable any given radiant node to take up the
/// mantle of `Herald` should the group's `Herald` node experience some unrecoverable
/// fault
pub trait Radiant: RadiantGroupMgmt + RadiantShardMgmt {
    type Role;

    fn set_role(&mut self, role: Self::Role);
    fn role(&self) -> Self::Role;

    fn set_member_id(&mut self, mid: <Self as RadiantGroupMgmt>::MemberId);
    fn member_id(&self) -> <Self as RadiantGroupMgmt>::MemberId;

    fn set_group_id(&mut self, gid: <Self as RadiantGroupMgmt>::GroupId);
    fn group_id(&self) -> <Self as RadiantGroupMgmt>::GroupId;

    //TODO make custom error types
    //fn send_group_broadcast(&mut self)->Result<(),()>;
    fn full_reset(&mut self)->Result<(),()>;
    fn soft_reset(&mut self)->Result<(),()>;
}

/// The functional requirements represented by the `RadiantShardMgmt` trait is
/// focused around maintaining accurate shard data both locally and within a shard group.
/// Each `Radiant` will maintain a database of group members' relevant shard information
/// and local shard information. Watermarking should be implemented to ensure that
/// we are able to indicate what good states are and enable us to roll back to some
/// last known good state in the event of an error
///
/// Watermark can be a timestamp or some other data type that is relevant to the
/// consensus protocol being used. Must be a type that implements the ShardWatermark
/// trait
pub trait RadiantShardMgmt {
    //type ShardAction;
    type Watermark: ShardWatermark;

    fn update_shard_list<A: ShardAction>(&mut self, action: A);

   //     fn update_shard_list(&mut self, action: Self::ShardAction);
    fn set_watermark(&mut self, mark: Self::Watermark);

    //TODO make custom error types
    //evict all shard data
    fn full_shard_reset(&mut self)->Result<(),()>;
    //roll back to the last "good" state as determined by watermark
    fn soft_shard_reset(&mut self)->Result<(),()>;
}

pub trait ShardWatermark {

}


/// The functional requirements represented by the `RadiantGroupMgmt` trait is
/// focused around maintaining accurate membership data within a shard group.
/// Each `Radiant` will maintain a database of group members' relevant information
/// (addresses, ports, id's, roles, etc.)
pub trait RadiantGroupMgmt {

    type GroupId;
    type MemberId;

    type ShardMemberDataTx;
    type ShardMemberDataRx;
    type MemberListType;

    type HeraldMemberDataTx;
    type HeraldMemberDataRx;
    type HeraldMemberListType;

    fn add_shard_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
        tx: Self::ShardMemberDataTx,
        rx: Self::ShardMemberDataRx,
    ) -> Result<(), ()>;

    fn remove_shard_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
    ) -> Result<Self::MemberId, ()>;

    fn add_herald_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::HeraldMemberListType,
        tx: Self::HeraldMemberDataTx,
        rx: Self::HeraldMemberDataRx,
    ) -> Result<(), ()>;

    fn remove_herald_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::HeraldMemberListType,
    ) -> Result<Self::MemberId, ()>;

    /// Can return `None` if we are in a state where no herald is elected
    fn herald(&self) -> Option<Self::MemberId>;
    fn set_shard_group_herald(&mut self, mid: Self::MemberId);


    /// Can return `None` if we are in a state where no bondsmith is elected
    fn bondsmith(&self) -> Option<Self::MemberId>;
    fn set_bondsmith(&mut self, mid: Self::MemberId);

    //todo make custom error types
    ///evict all group membership data
    fn group_reset(&mut self)->Result<(),()>;
}


pub struct RadiantNode<K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> {
    mid: MemberID,
    gid: GroupID,
    role: super::RadiantRole,

    state: RadiantState,
    order_state: OrderState,

    //Channels connecting one or more threads to ports opened up for comms
    //to & from shard members. Open a port for each new/existing member
    shard_members_tx: IndexMap<MemberID, UnboundedSender<Box<M>>>,
    shard_members_rx: IndexMap<MemberID, UnboundedReceiver<Box<M>>>,
/*
    //Channels connecting one or more threads to ports opened up for comms
    //to & from herald. Open a port for each new/existing member
    shard_heralds_rx: IndexMap<MemberID, UnboundedReceiver<RadiantMsg>>,
    shard_heralds_tx: IndexMap<MemberID, UnboundedSender<RadiantMsg>>,

    //Channels connecting one or more threads to ports opened up for comms
    //to & from bondsmith controllers. Open a port for each new/existing member
    bondsmith_rx: UnboundedReceiver<RadiantMsg>,
    bondsmith_tx: UnboundedSender<RadiantMsg>,
*/


    order_herald: MemberID,
    pub order_members: Vec<RadiantID>,

    /// what radiant group has what shards
    shards: IndexMap<ShardGroupKey, GroupID>,

    /// Will instantiate if this particular service instance is elected herald
    hsvc: HeraldNode<K, M>,
    /// Will instantiate if this particular service instance is elected herald bondsmith
    ctrlsvc: Option<BondsmithNode>,

    consensus: Option<C>,
    replication: Option<R>,
    watermark: Timestamp,
}

impl ShardWatermark for Timestamp {

}

impl<K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> RadiantNode<K, C,R, M> {
    pub fn default() -> Self {
        Self {
            mid: 0,
            gid: 0,
            role: super::RadiantRole::default(),
            state: RadiantState::default(),
            order_state: OrderState::default(),
            shard_members_tx: IndexMap::new(),
            shard_members_rx: IndexMap::new(),

            //Channels connecting one or more threads to ports opened up for comms
            //to & from herald. Open a port for each new/existing member
          /*

            shard_heralds_rx: UnboundedReceiver::<RadiantMsg>::default(),
            shard_heralds_tx: UnboundedSender::<RadiantMsg>::default(),

            //Channels connecting one or more threads to ports opened up for comms
            //to & from bondsmith controllers. Open a port for each new/existing member
            bondsmith_rx: UnboundedReceiver::<RadiantMsg>::default(),
            bondsmith_tx: UnboundedSender::<RadiantMsg>::default(),
        */

            order_herald: 0,
            order_members: Vec::new(),
            shards: IndexMap::new(),
            hsvc: HeraldNode::<K, M>::default(),
            ctrlsvc: None::<BondsmithNode>,
            consensus: None,
            replication: None,
            watermark: Timestamp::default(),
        }
    }

   // pub fn set_cfg(&mut self, cfg: &ShardbearerConfig) {
    //    self.mid = cfg.id();
        //todo
    //}
}



impl <K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> Radiant for RadiantNode<K, C,R, M>{
    type Role = super::RadiantRole;

    fn set_role(&mut self, role: Self::Role) {
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

    fn set_group_id(&mut self, gid: Self::GroupId) {
        self.gid = gid;
    }

    fn group_id(&self) -> Self::GroupId {
        self.gid
    }

    //TODO make custom error types
    //fn send_group_broadcast(&mut self)->Result<(),()>;
    fn full_reset(&mut self)->Result<(),()>{
        Ok(())
    }
    fn soft_reset(&mut self)->Result<(),()>{
        Ok(())
    }
}


impl <K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> RadiantShardMgmt for RadiantNode<K, C,R, M> {
    type Watermark = Timestamp;

    fn update_shard_list<A: ShardAction>(&mut self, action: A){
        unimplemented!();
    }

    fn set_watermark(&mut self, mark: Self::Watermark){
        unimplemented!();
    }

    //TODO make custom error types
    //evict all shard data
    fn full_shard_reset(&mut self)->Result<(),()>{
        Ok(())
    }
    //roll back to the last "good" state as determined by watermark
    fn soft_shard_reset(&mut self)->Result<(),()>{
        Ok(())
    }
}


impl <K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> RadiantGroupMgmt for RadiantNode<K, C,R, M>{

    type GroupId = GroupID;
    type MemberId = MemberID;

    type ShardMemberDataTx = UnboundedSender<Box<M>>;
    type ShardMemberDataRx = UnboundedReceiver<Box<M>>;
    type MemberListType = RadiantID;

    type HeraldMemberDataTx = UnboundedSender<M>;
    type HeraldMemberDataRx = UnboundedReceiver<M>;
    type HeraldMemberListType = RadiantID;

    fn add_shard_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
        tx: Self::ShardMemberDataTx,
        rx: Self::ShardMemberDataRx,
    ) -> Result<(), ()> {
        self.shard_members_tx.entry(mid).or_insert(tx);
        self.shard_members_rx.entry(mid).or_insert(rx);
        self.order_members.push(member_data);

        Ok(())
    }

    fn remove_shard_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
    ) -> Result<Self::MemberId, ()> {
        unimplemented!();
        //Ok(0)
    }

    fn add_herald_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::HeraldMemberListType,
        tx: Self::HeraldMemberDataTx,
        rx: Self::HeraldMemberDataRx,
    ) -> Result<(), ()>{

        self.hsvc.herald_peers_tx.entry(mid).or_insert(tx);
        self.hsvc.herald_peers_rx.entry(mid).or_insert(rx);
        //self.order_members.push(member_data);
        Ok(())
    }

    fn remove_herald_group_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::HeraldMemberListType,
    ) -> Result<Self::MemberId, ()>{

        //Ok(0)
        unimplemented!();
    }

    /// Can return `None` if we are in a state where no herald is elected
    fn herald(&self) -> Option<Self::MemberId>{

        None
    }

    fn set_shard_group_herald(&mut self, _mid: Self::MemberId){

    }


    /// Can return `None` if we are in a state where no bondsmith is elected
    fn bondsmith(&self) -> Option<Self::MemberId>{
        None
    }
    fn set_bondsmith(&mut self, _mid: Self::MemberId){

    }

    //todo make custom error types
    ///evict all group membership data
    fn group_reset(&mut self)->Result<(),()>{
        Ok(())
    }
}


impl<K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> RadiantStateMachine for RadiantNode<K, C,R, M> {
    type RadiantState = RadiantState;
    type SystemState = SysState;
    type ClusterState = OrderState;
    type OrderState = OrderState;

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
       // self.sys.update_state(state)
        unimplemented!()
    }

    fn system_state(&self) -> Self::SystemState {
        //self.sys.report_state()
        unimplemented!()

    }
}


impl<K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> Herald for RadiantNode<K, C,R, M> {

    type ShardKeyType = K;
    type MapKeyType = usize;

    //TODO need error handling-- return err if the key is already in the map
    fn add_new_key_mapping(&mut self, map_key: Self::MapKeyType, shard_key: Self::ShardKeyType)->Result<(),()>{

        self.hsvc.shard_key_mapping.entry(map_key).or_insert(shard_key);
        Ok(())
    }

}

impl<K, C: ShardbearerConsensus, R: ShardbearerReplication, M: ShardbearerMessage /*+ ?Sized*/> Bondsmith for RadiantNode<K, C,R, M> {
    type ShardKeyMapType = ShardKey;
    type GroupKeyType = GroupID;
    type ShardLoadTracker = ShardLoad;

    fn add_new_key_mapping(&mut self, group_key: Self::GroupKeyType, shard_key: Self::ShardKeyMapType)->Result<(),()>{
        // self.
        unimplemented!();
    }
    fn get_current_shard_load(&self, group_id: Self::GroupKeyType) -> Self::ShardLoadTracker{
        unimplemented!();
    }
    fn adjust_shard_load(&mut self, added: Self::ShardLoadTracker, group_id: Self::GroupKeyType){
        unimplemented!();

    }


}

//use std::error::Error;
use thiserror::Error;

#[derive(Clone, Debug)]
pub enum RadiantState {
    RESET,
    UNASSOCIATED,
    ASSOCIATED,
    LOCKED,     //replicating, voting, or performing some op that requires atomicity
    ERROR(RadiantStateError),
}

//TODO add more of these later
#[derive(Clone, Debug, Error)]
pub enum RadiantStateError {
    #[error("radiant state error")]
    ERROR,
}

impl Default for RadiantState {
    fn default() -> Self {
        RadiantState::UNASSOCIATED
    }
}


pub trait RadiantStateMachine {
    type RadiantState;
    type SystemState;
    type ClusterState;
    type OrderState;

    fn order_state(&self) -> Self::OrderState;
    fn update_order_state(&mut self, state: Self::OrderState);
    fn update_cluster_state(&mut self, state: Self::ClusterState);
    fn cluster_state(&self) -> Self::ClusterState;

    fn update_radiant_state(&mut self, state: Self::RadiantState);
    fn radiant_state(&self) -> Self::RadiantState;

    fn update_system_state(&mut self, state: Self::SystemState);
    fn system_state(&self) -> Self::SystemState;
}