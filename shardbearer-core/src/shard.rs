use indexmap::{map, IndexMap}; //, Entry};
use std::boxed::Box;

pub type ShardKey = usize;

pub trait ShardKeyType: std::hash::Hash + Eq + Default {

}


pub type ShardGroupKey = usize;

pub trait ShardAction {

}

pub trait ShardbearerMessage { //: Sized {

}

/// Some way to represent the load a group currently bears
/// with respect to the shards it is holding at the level of
/// the `Bondsmith`, to dynamically determine if load rebalancing
/// is needed
/// TODO revisit this
#[derive(Clone)]
pub struct ShardLoad{
    size_bytes: usize,
    capacity_bytes: usize
}

pub trait Shard<K, V>: Send + Sync {
    fn remove(&mut self, key: K) -> Result<V, ()>;
    fn insert(&mut self, key: K, val: V) -> Option<V>;
}

//#[derive(Serialize,Deserialize)]
pub struct ShardEntry<K: std::hash::Hash + Eq, V: Clone> {
    entry: IndexMap<K, V>,
    _key: std::marker::PhantomData<K>, //std::marker::PhantomData<&'a ()>,
    _val: std::marker::PhantomData<V>,
}

unsafe impl<K: std::hash::Hash + Eq, V: Clone> Send for ShardEntry<K, V> {}
unsafe impl<K: std::hash::Hash + Eq, V: Clone> Sync for ShardEntry<K, V> {}

pub trait ShardHashEntry<K, V>: Send + Sync {
    fn new(&self) -> Box<dyn Shard<K, V> + '_>;
}

impl<K: std::hash::Hash + Eq, V: Clone> ShardHashEntry<K, V> for ShardEntry<K, V> {
    fn new(&self) -> Box<dyn Shard<K, V> + '_> {
        Box::new(Self {
            entry: IndexMap::new(),
            _key: std::marker::PhantomData,
            _val: std::marker::PhantomData,
        })
    }
}

pub trait ShardMap<K, V>: Send + Sync {
    // type Shard;
}

pub struct ShardHashMap<K: std::hash::Hash + Eq, V: Clone> {
    map: IndexMap<ShardKey, Box<dyn ShardHashEntry<K, V>>>, //ShardEntry<K,V>>,
}

impl<K: std::hash::Hash + Eq, V: Clone> ShardHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }
}

impl<K: std::hash::Hash + Eq, V: Clone> ShardMap<K, V> for ShardHashMap<K, V> {
    //type Shard = dyn Shard<K,V>;//ShardEntry<K,V>;
}

unsafe impl<K: std::hash::Hash + Eq, V: Clone> Send for ShardHashMap<K, V> {}
unsafe impl<K: std::hash::Hash + Eq, V: Clone> Sync for ShardHashMap<K, V> {}

impl<K: std::hash::Hash + Eq, V: Clone> Shard<K, V> for ShardEntry<K, V> {
    fn remove(&mut self, key: K) -> Result<V, ()> {
        Err(())
    }
    fn insert(&mut self, key: K, val: V) -> Option<V> {
        None
    }
}

impl<K: std::hash::Hash + Eq, V: Clone> ShardEntry<K, V> {
    pub fn new() -> Self {
        Self {
            entry: IndexMap::<K, V>::new(),
            _key: std::marker::PhantomData,
            _val: std::marker::PhantomData,
        }
    }
    pub fn entry(&mut self, key: K) -> map::Entry<'_, K, V> {
        self.entry.entry(key)
    }

    pub fn entry_or_insert(&mut self, key: K, val: V) {
        //->indexmap::map::Entry<'_, K, V>{
        self.entry.entry(key).or_insert(val);
    }
}

/*
use indexmap::IndexMap;

pub type ShardKey = u64;

pub trait Shard<K, V: Clone>: Serialize + Deserialize {
    //   type Key;
    //   type Value;
    /*    fn put_shard_entry(&mut self, key: K, val: V){

    }
    fn get_shard_entry(&self,key: K)->Option<V>{
        None
    }
    fn remove_shard_entry(&mut self, key: K)->Result<V,()>{
        Err(())
    }*/
    // fn new()->Self where Self: Sized;
    // fn entry<T>(&mut self, key: K)->T;//<K,V>;//indexmap::map::Entry<'_, K, V>{
    // self.entry.entry(key)
}



//use shardbearer_core::shard::{Shard,ShardKey};

#[derive(Serialize, Deserialize)]
pub struct ShardEntry<K: std::hash::Hash + Eq, V: Clone> {
    entry: IndexMap<K, V>,
}

pub trait ShardHash {}

impl<K: std::hash::Hash + Eq, V: Clone> Shard<K, V> for ShardEntry<K, V> {
    //fn entry(&mut self, key: K)
    //fn entry(&mut self, key: K)->indexmap::map::Entry<'_, K, V>{
    //    self.entry.entry(key)
    //}
}

impl<K: std::hash::Hash + Eq, V: Clone> ShardEntry<K, V> {
    pub fn new() -> Self {
        Self {
            entry: IndexMap::<K, V>::new(),
        }
    }
    pub fn entry(&mut self, key: K) -> indexmap::map::Entry<'_, K, V> {
        self.entry.entry(key)
    }

    pub fn entry_or_insert(&mut self, key: K, val: V) {
        //->indexmap::map::Entry<'_, K, V>{
        self.entry.entry(key).or_insert(val);
    }
}
*/
