use crate::order::OrderShardAction;
use crate::shard::{Shard, ShardKey};
use crate::system::System;

use crate::shard::ShardEntry;

use thiserror::Error;

pub type MemberID = u64;
pub type GroupID = u64;

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
pub struct RadiantKey {
    pub gid: GroupID,
    pub mid: MemberID,
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

pub trait Radiant: RadiantStateMachine {
    type Role;
    type GroupId;
    type MemberId;
    type MemberDataTx;
    type MemberDataRx;
    type MemberListType;

    fn update_role(&mut self, state: Self::Role);
    fn role(&self) -> Self::Role;

    fn set_member_id(&mut self, mid: Self::MemberId);
    fn member_id(&self) -> Self::MemberId;

    fn group_id(&self) -> Self::GroupId;
    fn set_group_id(&mut self, gid: Self::GroupId);

    //fn add_member_tx(&mut self, mid: Self::MemberId, data: Self::MemberDataTx) -> Result<(), ()>;
    // fn add_member_rx(&mut self, mid: Self::MemberId, data: Self::MemberDataRx) -> Result<(), ()>;

    fn add_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
        tx: Self::MemberDataTx,
        rx: Self::MemberDataRx,
    ) -> Result<(), ()>;
    fn remove_member(
        &mut self,
        mid: Self::MemberId,
        member_data: Self::MemberListType,
    ) -> Result<Self::MemberId, ()>;

    fn herald(&self) -> Option<Self::MemberId>; //None if we are in a state where no herald is elected
    fn set_herald(&mut self, mid: Self::MemberId);

    fn update_shard_list(&mut self, action: OrderShardAction);

    fn reset_shards(&mut self);
    fn elect_herald(&mut self);
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
