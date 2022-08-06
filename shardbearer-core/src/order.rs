use crate::shard::{Shard, ShardKey};
use std::convert::TryFrom;

use shardbearer_state::order::RadiantOrderState;

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
