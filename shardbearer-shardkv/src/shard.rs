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
