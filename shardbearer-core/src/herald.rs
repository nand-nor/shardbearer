use crate::msg::*;
use crate::shard::*;
use crate::radiant::{MemberID, Radiant, RadiantNode};
use crate::consensus::{ShardbearerConsensus, ShardbearerReplication};

//use shardbearer_proto::common::common::HeraldInfo;
use tracing::{debug, error, info, trace, warn};

use indexmap::IndexMap;

use protobuf::Message;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use std::boxed::Box;

pub struct HeraldNode<K> {

    pub shard_key_mapping: IndexMap<ShardKey, K>,

    pub herald_peers_tx: IndexMap<MemberID, UnboundedSender<Box<HeraldMsg>>>,
    pub herald_peers_rx: IndexMap<MemberID, UnboundedReceiver<Box<HeraldMsg>>>,

    bondsmith_rx: UnboundedReceiver<Box<BondsmithMsg>>,
    bondsmith_tx: UnboundedSender<Box<BondsmithMsg>>,

    bondsmith: MemberID,
}

impl<K> Default for HeraldNode<K> {
    fn default()->Self{
        let (mut b_tx, mut b_rx) = unbounded_channel();
        Self{
            shard_key_mapping: IndexMap::new(),

            herald_peers_tx: IndexMap::new(),
            herald_peers_rx: IndexMap::new(),

            bondsmith_rx: b_rx, //UnboundedReceiver::<Box<BondsmithMsg>>::new(),//default(),
            bondsmith_tx: b_tx, //UnboundedSender::<Box<BondsmithMsg>>::default(),

            bondsmith: 0,
        }
    }
}


pub trait Herald: Radiant {
    type ShardKeyType;
    type MapKeyType;

    fn add_new_key_mapping(&mut self, map_key: Self::MapKeyType, shard_key: Self::ShardKeyType)->Result<(),()>;
}




