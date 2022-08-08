
use crate::shard::{ShardKey, ShardKeyType, ShardLoad};
//use protobuf::Message;

//use shardbearer_state::bondsmith::BondsmithState;
//use shardbearer_state::order::OrderState;
//use shardbearer_state::radiant::{RadiantState, RadiantStateMachine};
//use std::sync::{Arc, Mutex};

//use crate::{RadiantRole,HeraldRole,ControllerRole};

//use shardbearer_state::sys::{RadiantSystem, SysState};

use crate::herald::Herald;
use indexmap::IndexMap;

use crate::radiant::{Radiant, GroupID, MemberID};
use crate::msg::*;

use crate::consensus::{ShardbearerConsensus, ShardbearerReplication};

use tokio::sync::mpsc::{ UnboundedReceiver, UnboundedSender};
//use tokio::sync::oneshot::channel;

use std::boxed::Box;

pub struct BondsmithNode{//<M: ShardKeyType> {
    heralds_tx: IndexMap<MemberID, UnboundedSender<Box<BondsmithMsg>>>,
    heralds_rx: IndexMap<MemberID, UnboundedReceiver<Box<BondsmithMsg>>>,

    pub shard_key_group_mapping: IndexMap<ShardKey/*dyn ShardKeyType*/, GroupID>,
    group_load: IndexMap<GroupID, ShardLoad>,

}

impl Default for BondsmithNode {
    fn default()->Self{
        Self{
            heralds_tx: IndexMap::new(),
            heralds_rx: IndexMap::new(),
            shard_key_group_mapping: IndexMap::new(),
            group_load: IndexMap::new(),
        }
    }
}

impl BondsmithNode {
    pub fn new()->Self{
        BondsmithNode::default()
    }
}



pub trait Bondsmith: Herald + Radiant {
    type ShardKeyMapType;
    type GroupKeyType;
    type ShardLoadTracker: Clone;

    fn add_new_key_mapping(&mut self, group_key: Self::GroupKeyType, shard_key: Self::ShardKeyMapType)->Result<(),()>;
    fn get_current_shard_load(&self, group_id: Self::GroupKeyType) -> Self::ShardLoadTracker;
    fn adjust_shard_load(&mut self, added: Self::ShardLoadTracker, group_id: Self::GroupKeyType);

}
