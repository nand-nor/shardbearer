use crate::msg::*;
use crate::shard::*;
use crate::radiant::{MemberID, Radiant, RadiantNode};
use crate::consensus::{ShardbearerConsensus, ShardbearerReplication};

//use shardbearer_proto::common::common::HeraldInfo;
use tracing::{debug, error, info, trace, warn};

use indexmap::IndexMap;

use protobuf::Message;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

pub struct HeraldService<K> {

    shard_key_mapping: IndexMap<ShardKey, K>,

    herald_peers_tx: IndexMap<MemberID, UnboundedSender<HeraldMsg>>,
    herald_peers_rx: IndexMap<MemberID, UnboundedReceiver<HeraldMsg>>,

    bondsmith_rx: UnboundedReceiver<BondsmithMsg>,
    bondsmith_tx: UnboundedSender<BondsmithMsg>,

    bondsmith: MemberID,
}

impl<K> Default for HeraldService<K> {
    fn default()->Self{
        Self{
            shard_key_mapping: IndexMap::new(),

            herald_peers_tx: IndexMap::new(),
            herald_peers_rx: IndexMap::new(),

            bondsmith_rx: UnboundedReceiver::<BondsmithMsg>::default(),
            bondsmith_tx: UnboundedSender::<BondsmithMsg>::default(),

            bondsmith: 0,
        }
    }
}


pub trait Herald: Radiant {
    type ShardKeyType;
    type MapKeyType;

    fn add_new_key_mapping(&mut self, map_key: Self::MapKeyType, shard_key: Self::ShardKeyType)->Result<(),()>;
}


impl<K, C: ShardbearerConsensus, R: ShardbearerReplication> Herald for RadiantNode<K, C,R> {

    type ShardKeyType = K;
    type MapKeyType = usize;

    //TODO need error handling-- return err if the key is already in the map
    fn add_new_key_mapping(&mut self, map_key: Self::MapKeyType, shard_key: Self::ShardKeyType)->Result<(),()>{
        self.shard_key_mapping.entry(map_key).or_insert(shard_key);
        Ok(())
    }

}



