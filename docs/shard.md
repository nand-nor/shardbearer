# `Shard` Trait

The `Shard` trait is meant to enable datastores of dynamic types for keys (`K`) and values (`V`), so as such is defined as 
`Shard<K,V>` . 



The following traitbounds are not required but recommended:
- `K`: `Hash`, `Eq`, `PartialEq`, `Ord`, `ParialOrd`, `Clone` 
- `V`: `Clone`

These traitbounds -are- required / will be someday when this is closer to being done:
- `K`: `Serialize`, `Deserialize`
- `V`: `Serialize`, `Deserialize`

This is needed to move `shards` between `orders`  