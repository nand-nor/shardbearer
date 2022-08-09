use super::MemberID;
use crate::radiant::Radiant;
use crate::shard::*;

use indexmap::IndexMap;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
//use std::boxed::Box;

pub struct HeraldNode<K, M: ShardbearerMessage /*+ ?Sized*/> {
    pub shard_key_mapping: IndexMap<ShardKey, K>,

    pub herald_peers_tx: IndexMap<MemberID, UnboundedSender<M>>,
    pub herald_peers_rx: IndexMap<MemberID, UnboundedReceiver<M>>,

    bondsmith_rx: UnboundedReceiver<M>,
    bondsmith_tx: UnboundedSender<M>,

    bondsmith: MemberID,
}

impl<K, M: ShardbearerMessage /*+ ?Sized*/> Default for HeraldNode<K, M> {
    fn default() -> Self {
        let (b_tx, b_rx) = unbounded_channel();
        Self {
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

    fn add_new_key_mapping(
        &mut self,
        map_key: Self::MapKeyType,
        shard_key: Self::ShardKeyType,
    ) -> Result<(), ()>;
}
