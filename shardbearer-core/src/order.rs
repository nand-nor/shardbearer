use crate::shard::{Shard, ShardKey};
use std::convert::TryFrom;

//use shardbearer_proto::common::Timestamp;

use shardbearer_state::order::OrderState;

use crate::radiant::GroupID;

#[derive(Clone)]
pub enum OrderShardAction {
    ADD(dyn ShardKey),
    MOVE((dyn ShardKey, GroupID)),
    REMOVE(dyn ShardKey),
    CONFIRM(dyn ShardKey),
    COMMIT_P1(dyn ShardKey),
    COMMIT_P2(dyn ShardKey)
    //TIMESTAMP((dyn ShardKey, Timestamp))
}

impl<K: ShardKey + Default> Default for OrderShardAction {
    fn default() -> Self {
        OrderShardAction::ADD(K::default())
    }
}
