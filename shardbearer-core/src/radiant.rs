use crate::herald::HeraldState;
use crate::membership::Role;
use crate::membership::{GroupID, MemberID, Membership};
use crate::order::{Order, OrderShardAction};
use crate::shard::{Shard, ShardKey};
use crate::system::System;

use crate::shard::ShardEntry;
use indexmap::IndexMap;

pub trait Radiant<O: Order, M: Role, S: System> {
    type OrderState;
    type RadiantState;
    type SystemState;
    type RoleState;
    type ClusterState;

    fn update_cluster_state(&mut self, state: Self::ClusterState);
    fn update_order_state(&mut self, state: Self::OrderState);
    fn update_radiant_state(&mut self, state: Self::RadiantState);
    fn update_system_state(&mut self, state: Self::SystemState);
    fn update_role(&mut self, state: Self::RoleState);
    fn set_member_id(&mut self, mid: M::RoleId);
    fn set_group_id(&mut self, gid: O::GroupId);
    fn member_id(&self) -> M::RoleId;
    fn group_id(&self) -> O::GroupId;
    fn cluster_state(&self) -> Self::ClusterState;
    fn order_state(&self) -> Self::OrderState;
    fn radiant_state(&self) -> Self::RadiantState;
    fn system_state(&self) -> Self::SystemState;
    fn role(&self) -> Self::RoleState;
}

#[derive(Clone)]
pub enum RadiantState {
    MEMBER,
    HERALD(HeraldState),
}

impl Default for RadiantState {
    fn default() -> Self {
        RadiantState::MEMBER
    }
}

//#[derive(Clone)]
pub struct RadiantNode<O: Order, M: Role, S: System> {
    order: O,
    mid: MemberID,
    member_state: M,
    sys: S,
    rstate: Membership,
    //cluster: ClusterState,
}

impl<O: Order, M: Role, S: System> Radiant<O, M, S> for RadiantNode<O, M, S> {
    type OrderState = O::OrderState;
    type RadiantState = Membership;
    type SystemState = S::SystemState;
    type RoleState = M::RoleState;
    type ClusterState = O::OrderState;

    fn update_cluster_state(&mut self, state: Self::ClusterState) {
        self.order.update_state(state);
    }

    fn update_order_state(&mut self, state: Self::OrderState) {
        self.order.update_state(state);
    }

    fn update_radiant_state(&mut self, state: Self::RadiantState) {
        self.rstate = state;
    }
    fn update_system_state(&mut self, state: Self::SystemState) {
        self.sys.update_state(state)
    }

    fn update_role(&mut self, state: Self::RoleState) {
        self.member_state.set_state(state)
    }
    fn set_member_id(&mut self, mid: M::RoleId) {
        self.member_state.set_id(mid);
    }

    fn set_group_id(&mut self, gid: O::GroupId) {
        self.order.group_id();
    }
    fn member_id(&self) -> M::RoleId {
        self.member_state.get_id()
    }
    fn group_id(&self) -> O::GroupId {
        self.order.group_id()
    }

    //TODO make an impl just for tracking cluster state
    fn cluster_state(&self) -> Self::ClusterState {
        self.order.report_state()
    }

    fn order_state(&self) -> Self::OrderState {
        self.order.report_state()
    }
    fn radiant_state(&self) -> Self::RadiantState {
        self.rstate.clone()
    }
    fn system_state(&self) -> Self::SystemState {
        self.sys.report_state()
    }
    fn role(&self) -> Self::RoleState {
        self.member_state.get_state()
    }
}

impl<O: Order, M: Role, S: System> RadiantNode<O, M, S> {
    pub fn default() -> Self {
        Self {
            order: O::default(),
            mid: 0,
            member_state: M::default(),
            sys: S::default(),
            rstate: Membership::UNASSOCIATED,
            //cluster:
        }
    }
}
