use crate::shard::{Shard, ShardKey};
use std::convert::TryFrom;

pub trait Order: Default {
    type OrderState;
    type GroupId;
    type MemberId;
    type MemberData;

    fn group_id(&self) -> Self::GroupId;
    fn set_group_id(&mut self, gid: Self::GroupId);

    fn add_member(&mut self, mid: Self::MemberId, data: Self::MemberData) -> Result<(), ()>;
    fn remove_member(&mut self, mid: Self::MemberId) -> Result<(Self::MemberData), ()>;
    fn herald(&self) -> Option<Self::MemberId>; //None if we are in a state where no herald is elected
    fn set_herald(&mut self, mid: Self::MemberId);
    fn report_state(&self) -> Self::OrderState;
    fn update_state(&mut self, state: Self::OrderState);
    fn update_shard_list(&mut self, action: OrderShardAction);

    fn reset_shards(&mut self);
    fn elect_herald(&mut self);
}

#[derive(Clone)]
pub enum RadiantOrderState {
    INACTIVE, //nodes are still setting up the system, have no state info yet
    VOTING,   //After the inactive state we must vote to fully be setup and active. There are
    //multiple rounds of voting so may need to split this into a state for each voting round?
    ACTIVE, //nodes are set up, all info complete (Roles, shard services, etc.)
    RESET,  //reconfiguration in process
}

impl TryFrom<i32> for RadiantOrderState {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == RadiantOrderState::INACTIVE as i32 => Ok(RadiantOrderState::INACTIVE),
            x if x == RadiantOrderState::VOTING as i32 => Ok(RadiantOrderState::VOTING),
            x if x == RadiantOrderState::ACTIVE as i32 => Ok(RadiantOrderState::ACTIVE),
            x if x == RadiantOrderState::RESET as i32 => Ok(RadiantOrderState::RESET),
            _ => Err(()),
        }
    }
}

impl Default for RadiantOrderState {
    fn default() -> Self {
        RadiantOrderState::INACTIVE
    }
}

#[derive(Clone)]
pub enum OrderShardAction {
    ADD(ShardKey),
    REMOVE(ShardKey),
}

impl Default for OrderShardAction {
    fn default() -> Self {
        OrderShardAction::ADD(0)
    }
}
