use super::{GroupID, MemberID};
use crate::herald::Herald;
use crate::radiant::Radiant;
use crate::shard::{ShardKey, /*ShardKeyType,*/ ShardLoad, ShardbearerMessage};

use indexmap::IndexMap;
use std::boxed::Box;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub enum BondsmithState {
    INIT,
    LOCKED,
    ACTIVE,
}

pub struct BondsmithNode {
    //<M: ShardKeyType> {
    heralds_tx: IndexMap<MemberID, UnboundedSender<Box<dyn ShardbearerMessage>>>,
    heralds_rx: IndexMap<MemberID, UnboundedReceiver<Box<dyn ShardbearerMessage>>>,

    pub shard_key_group_mapping: IndexMap<ShardKey /*dyn ShardKeyType*/, GroupID>,
    group_load: IndexMap<GroupID, ShardLoad>,
}

impl Default for BondsmithNode {
    fn default() -> Self {
        Self {
            heralds_tx: IndexMap::new(),
            heralds_rx: IndexMap::new(),
            shard_key_group_mapping: IndexMap::new(),
            group_load: IndexMap::new(),
        }
    }
}

impl BondsmithNode {
    pub fn new() -> Self {
        BondsmithNode::default()
    }
}

pub trait Bondsmith: Herald + Radiant {
    type ShardKeyMapType;
    type GroupKeyType;
    type ShardLoadTracker: Clone;

    fn add_new_key_mapping(
        &mut self,
        group_key: Self::GroupKeyType,
        shard_key: Self::ShardKeyMapType,
    ) -> Result<(), ()>;
    fn get_current_shard_load(&self, group_id: Self::GroupKeyType) -> Self::ShardLoadTracker;
    fn adjust_shard_load(&mut self, added: Self::ShardLoadTracker, group_id: Self::GroupKeyType);
}
