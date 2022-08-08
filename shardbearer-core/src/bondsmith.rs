use crate::config::ShardbearerConfig;
use crate::radiant::{RadiantMsg, RadiantNode};
use crate::shard::{ShardKey, ShardLoad};
use protobuf::Message;

use shardbearer_state::bondsmith::BondsmithState;
use shardbearer_state::order::OrderState;
use shardbearer_state::radiant::{RadiantState, RadiantStateMachine};
use std::sync::{Arc, Mutex};

use crate::{RadiantRole,HeraldRole,ControllerRole};

use shardbearer_state::sys::{RadiantSystem, SysState};

use shardbearer_proto::common::common::{BeaconResponse, Radiant as RadiantId};
use timer::{Guard, Timer};

use crate::herald::HeraldMsg;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::consensus::{ShardbearerConsensus, ShardbearerReplication};

use tokio::sync::mpsc::{Receiver, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::channel;



pub struct BondsmithService {
    heralds_tx: IndexMap<MemberID, UnboundedSender<BondsmithMsg>>,
    heralds_rx: IndexMap<MemberID, UnboundedReceiver<BondsmithMsg>>,

    shard_key_group_mapping: IndexMap<ShardKey, GroupID>,
    group_load: IndexMap<GroupID, ShardLoad>,

}

impl Default for BondsmithService {
    fn default()->Self{
        Self{
            heralds_tx: IndexMap::new(),
            heralds_rx: IndexMap::new(),
            shard_key_group_mapping: IndexMap::new(),
            group_load: IndexMap::new(),
        }
    }
}

impl BondsmithService {
    pub fn new()->Self{
        BondsmithService::default()
    }
}



pub trait Bondsmith: Herald + Radiant {
    type ShardKeyType;
    type GroupKeyType;
    type ShardLoadTracker: Clone;

    fn add_new_key_mapping(&mut self, group_key: Self::GroupKeyType, shard_key: Self::ShardKeyType)->Result<(),()>;
    fn get_current_shard_load(&self, group_id: Self::GroupKeyType) -> Self::ShardLoadTracker;
    fn adjust_shard_load(&mut self, added: Self::ShardLoadTracker, group_id: Self::GroupKeyType);

}

impl<K, C: ShardbearerConsensus, R: ShardbearerReplication> Herald for RadiantNode<K, C,R> {
    type ShardKeyType = ShardKey;
    type GroupKeyType = GroupID;
    type ShardLoadTracker = ShardLoad;

    fn add_new_key_mapping(&mut self, group_key: Self::GroupKeyType, shard_key: Self::ShardKeyType)->Result<(),()>{
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