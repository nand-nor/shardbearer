use crate::shard::{Shard, ShardKey, ShardKeyType};
use std::convert::TryFrom;

//use shardbearer_proto::common::Timestamp;
use std::boxed::Box;
use shardbearer_state::order::OrderState;

use crate::radiant::GroupID;

#[derive(Clone)]
pub enum OrderShardAction<K: ShardKeyType> {
    ADD(Box<K>),
    MOVE((Box<K>, GroupID)),
    REMOVE(Box<K>),
    CONFIRM(Box<K>),
    COMMIT_P1(Box<K>),
    COMMIT_P2(Box<K>)
    //TIMESTAMP((dyn ShardKey, Timestamp))
}

impl<K: ShardKeyType> Default for OrderShardAction<K> {
    fn default() -> Self {
        OrderShardAction::ADD(Box::new(K::default()))
    }
}
