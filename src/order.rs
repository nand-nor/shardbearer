use indexmap::IndexMap;
use shardbearer_core::membership::Role;
use shardbearer_core::membership::{GroupID, MemberID, Membership};
use shardbearer_core::order::{Order, OrderShardAction, RadiantOrderState};
use shardbearer_core::radiant::Radiant;
use shardbearer_core::shard::{Shard, ShardKey};
use shardbearer_core::system::System;

#[derive(Default)]
pub struct RadiantData {} //move this to shardbearer_core::radiant?

pub type RadiantShardMap = Vec<ShardKey>; //IndexMap<ShardKey, std::boxed::Box<dyn Shard>>;

#[derive(Default)]
pub struct RadiantOrder {
    gid: GroupID,
    members: IndexMap<MemberID, RadiantData>,
    herald: MemberID,
    shards: RadiantShardMap,
    state: RadiantOrderState,
}

impl Order for RadiantOrder {
    type OrderState = RadiantOrderState;
    type MemberData = RadiantData;
    type GroupId = GroupID;
    type MemberId = MemberID;
    //type ShardType = ShardKey;//std::boxed::Box<dyn Shard>;
    //type ShardMap = RadiantShardMap;

    fn set_group_id(&mut self, gid: Self::GroupId) {
        self.gid = gid;
    }

    fn group_id(&self) -> Self::GroupId {
        self.gid
    }
    fn add_member(&mut self, mid: Self::MemberId, data: Self::MemberData) -> Result<(), ()> {
        Ok(())
    }

    fn remove_member(&mut self, mid: Self::MemberId) -> Result<Self::MemberData, ()> {
        Err(())
    }
    fn herald(&self) -> Option<Self::MemberId> {
        None
    } //None if we are in a state where no herald is elected
    fn set_herald(&mut self, mid: Self::MemberId) {
        self.herald = mid;
    }
    fn report_state(&self) -> Self::OrderState {
        self.state.clone()
    }
    fn update_state(&mut self, state: Self::OrderState) {
        self.state = state
    }
    //fn shards(&self)->Self::Shards {

    //  }

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
