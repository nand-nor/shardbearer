
# `Orders`: `Shard` Replica Groups

Each radiant keeps state for the `order` of which is belongs. This is meant to enable any
`radiant` in the order at any time to take on the `herald` role. In the event that the previously elected
`herald` goes offline unexpectedly without first sharing it's currently held state data, the `order` will 
be able to swiftly recover: a new election will be held to determine the new `herald` for the group without
any major disruption in service or significant downtime taken to recreate previously watermarked state. 
 
 
 As part of the `Order` trait, the following state is kept:
 - A list of the keys in the `shards` that the `order` is responsible for. This list is separate from the
 actual `shard` data storage object. This is effectively meant to be a shallow copy of the data the `order`
 is responsible for, for `herald` reporting purposes / interaction with the `shard` controller `herald`
 
 
 
 Order states:
 INACTIVE
 VOTING
 ACTIVE
 RESETLOCK
 ERROR
 
 
 voting and resetlock states are lock states. Until the system state changes no new node can enter the system,
 request system info, make a change, etc. It is up to individual nodes to repeat requests as often as needed
 if their requests are denied during these states. Possibly need to add in a RetryLast state?