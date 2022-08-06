
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
 
 
 
## Order states:

<img title = "Order State Machine" alt="Order State Machine-- see .dot file" src=docs/order_states.png>

``` 
pub enum OrderState {
 BOOTSTRAP,  //nodes are still setting up the system, have no state info yet
 RESET,      //reset, reconfiguration, or recovery in process
 ACTIVE,     //nodes are set up, all info complete (Roles, shard services, etc.), order is not currently performing an operation
 LOCKED(OrderStateOp),  //Order is locked for some operation
 ERROR(OrderStateError), //Order is in error state
}
``` 
It is up to individual nodes, making requests or sending data to any given order, to respond appropriately when 
receiving a `LOCKED` response. To that end, the `OrderStateOp` enum represents the current operation of the order
for which it has locked itself (e.g. rejecting any external RPCs that would potentially result in a state change). Using
this information, a requester/sender node can, for example, set up an appropriate retry rate. The `OrderStateOp` enum
are as follows:

``` 
pub enum OrderStateOp {
 MEMBERSHIP_UPDATE, //adding a new node to the order
 MEMBERSHIP_SET,    //establishing entirely new membership group (during bootstrap or subsequent resets)
 REPLICATING,
 VOTING,
 PROBE,              //order is being probed to ensure all order members up and responsive

}
```

The `OrderStateError` enum essentially wraps a more detailed order state and indicates the level of
recoverability of the state, so that the next state transition can be performed when appropriate.
It is represented as follows:
``` 
pub enum OrderStateError{
 RECOVERABLE(OrderError), //the order can recover without a reset
 RECOVER_RESET, //recoverable with an order-wide reset
 SYSTEM_RESET, //recoverable with a system-wide reset
 UNRECOVERABLE,
}

pub enum OrderError {
 STATE,
 MEMBERSHIP,
 SHARD,
 OTHER
}


```
The finer-grain detail `OrderError` enum indicates the current error at the order level, used to determine
what state the order should transition to next (e.g. if the error is recoverable) as well as what RPCs should
be generated in response to the detected error (e.g. rolling back to the last-agreed-upon "good" state, 
request then perform an order-wide reset, request a system-wide reset). This will also be useful for
logging purposed when that framework is ready to be built out.

- `STATE`: order state across all nodes in order is inconsistent
- `MEMBERSHIP`: membership data across all nodes in order is inconsistent. This can be
either a member is accounted for by some but not all nodes, or a member of the
order has gone down. Should lead to a reset at the order level and potentially at the
system level, depending
- `SHARD`: shard data across all nodes in order is inconsistent/incorrect
- `OTHER`: something else, maybe wrap an error type here?


## Order Role Restrictions

The role any given node in an order currently has will determine what RPCs it can send/respond to with respect
to the `order` state. If some error state is detected, and the error state is recoverable, only the `order`'s `herald`
can for example request an `order` reset, or send a system-reset request. 




