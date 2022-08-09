# `Shard` Trait

WIP this trait is still being defined! The below are more notes-to-self at this point in time

The `Shard` trait is meant to enable datastores of dynamic types for keys (`K`) and values (`V`), so as such is defined as 
`Shard<K,V>` . 


The following traitbounds are required
- `K`: `Hash`, `Eq`, `PartialEq`, `Ord`, `ParialOrd`, `Clone`,  `Serialize`, `Deserialize` 
- `V`: `Clone`,  `Serialize`, `Deserialize`


This is needed to move `shards` between `orders`  