# Architecture

The `shardbearer` system uses a hierarchy of roles, similar to a tiering system, to coordinate activities 
within the system. Groups of `radiants` are called `orders`, which are assigned `shards`; for each `shard` group a 
`herald` must be elected. A `herald` coordinates the activity of `radiants` within the `shard` group, mostly 
relating to moving shards, or fracturing/consolidating `shards`. The `bondsmith` coordinates the activity of the 
whole system, communicating with `heralds` to perform things like load balancing and generally maintaining 
distributed state of the system. System roles, like `radiant`, `heralrd`, are defined as traits, with structs that 
implement said traits, which are then wrapped into services and combined with `protobuf` `gRPC` implementations to 
structure external communication among nodes in the system. 
 
As mentioned above, the system is comprised of `radiants`, `heralds`, and a controlling `herald` called the `bondsmith`. 
There is additionally a `shard` component.

- A `shard` is a subset of the key-value store that a given `shardbearer` system instance is serving. 

- A `radiant` essentially represents an individual server instance responsible for serving the data
  contained in a `shard`. The data can be an entire `shard` or multiple `shards`, depending
  on current load, group membership (detailed further below), and system resources. 

- An `order` is a group of `radiants` responsible for serving requests to create/remove/update values in one or
  more`shards`. An `order` is the term applied to a replica group, composed of two or more `radiants`, responsible for
  one or more `shards`. Each order has a leader or `herald`, that must be elected at the time of `order` formation.

- A `herald` is elected within an `order` to act as the leader of the replica group. A `herald` is a covariant of
  a `radiant`, meaning all `heralds` are `radiants` but not all `radiants` are `heralds`. All `radiant` objects
  have the ability to become a `herald` at any time, for example if a group's `herald` experienced some unrecoverable
  failure.  A `shardbearer` system can have multiple `heralds` depending on number of nodes in the system. A system 
  must consist of no less than 3 nodes, and `order` sizes shall be determined based on appropriate load balancing
  w.r.t group membership. (Note to self: Need to decide what logic will determine when a group will need to be 
  split, if not from a human admin). 
  
  All the `heralds` in a system report to/coordinate with a top-level controller called a `bondsmith`
  
- A `bondsmith` is a `radiant`, `herald` node with an additional role as the top-level system controller. It's
main function is to provide system configuration (what nodes (`radiants`) are in what replica groups (`orders`), 
and what shards each group is responsible for replicating/serving, etc.). It is both the entrypoint for client 
requests as well as for system administrators who want to manually reconfigure a system. `heralds` coordinate with 
eachother & the `bondsmith` to maintain distributed state in the system. Dynamic handling of system membership and
`shard` load balancing
is also a future goal of the `bondsmith`, to reduce the need for human interaction/input (and increase fault tolerance/
recoverability). As new servers (`radiants`) enter and leave the system, `shards` must be redistributed to balance 
the load dynamically. 

    The top level controller `bondsmith` represents the main entrypoint for receiving client
requests & therefore a node must be assigned this role prior to system bring up. Similar to a 3 tiered system,
the `bondsmith` behaves somewhat like tier 1. The `heralds` in this system behave similarly to tier 2, and the 
`radiants` tier 3. Need to determine if it is feasible to have a dynamically chosen `bondsmith` e.g. what that would
mean in terms of an additional layer providing the client entry point(s), if/how that role could spontaneously
arise & then set up *secure* channels with such an additional layer, etc. 

    At system startup, a `bondsmith` will wait some amount of time collecting beacons from other nodes in the system,
    and will eventually determine a number of `orders` (if such configuration information is not provided via config file)
    and assign the accounted-for `radiants` to each order. At that point, each `order` coordinates internally to elect
    a `herald`. The `herald` will then take on direct comms with the `bondsmith`; at no other point will the `bondsmith`
    have direct communication (RPC) to `radiant` unless that node is in the `herald` role or some system error has
    arisen (like a system reset is needed, a new `herald` was elected, etc.)
    
## System configuration

Each binary produced (by default in the basic `radiant` state) must be provided with a set of parameters as a
    toml file to be parsed at runtime. This must include at least one address of a known "neighbor" `radiant` node in 
    the system through which the node can associate with the rest of the system. Additionally this should also
    include the address of the assigned-in-advance `bondsmith`. Communication in the system is represented by a graph 
    of essentially what nodes have the ability to directly send RPCs to what other nodes, referred to here as the 
    association graph. To support system goals around recoverability, autonomy, etc., there are
    association graph invariants that must hold and that determine certain parameters to be passed in system config files. 

#### Variance: System Roles and Relationships

The main traits defined here relevant to the function of a node in terms of group role/membership are 
`radiant`, `herald`, and `bondsmith`. These roles-as-traits establish a system of variance that defines the way 
any given node will function with respect to the rest of the system. The goal here is to enable any node in the
 system to take on any role appropriate given a perceived set of conditions. In this system, all nodes are `radiant`, 
 e.g. `radiant` is the supertype trait that all nodes
implement, and `radiant` is the role that all nodes in all states execute in.

Depending on the voting results in the system, a `radiant` can become the `herald` of it's `order`. So, included
in each `radiant` binary is the ability to, at runtime, execute methods from the `herald` trait object. And because 
`radiant` is the supertype, any `bondsmith` or `herald` can downgrade to become just a replica group member.

The following relationships hold:
- All nodes are `radiant`, e.g. `radiant` is the supertype trait that all nodes implement
- All `heralds` are `radiant` but not all `radiants` are `heralds`
- All `bondsmiths` are `radiant` and `heralds`, but the reverse does not hold
- All `radiants` belong to an `order`. The `order` of the `bondsmith` is made up by the `heralds`, and this `order` is
special in that is does not serve `shard` requests

##### Association Graph Invariants

The association graph, where an edge is a network connection between two nodes as provided by a known-in-advance
 (via config file providing a list of one or more neighbors) network address, must hold the following invariants:

- edges are directed
- no self-loops
- each node must have at least one edge incident from (leaving) itself and incident to (entering) another 
node (not itself). Put more simply, a `radiant` node must have a directed edge pointing to a neighbor.
- each node can have `t` incident-from edges (leaving) from other nodes where `t` falls in the range`0<=t<=n-1`  
  and `n` is the number of `radiant` nodes currently in the system. Put more simply, multiple `radiant` nodes can
  have an edge pointing to a single `radiant` node, and a `radiant` node can also have no edges incident to (entering) it,
  but no node can point to itself. The case where a `radiant` node has `0` incident-to-itself edges will arise when a new
  `radiant` enters an already-established system; the other nodes in the system will have no prior knowledge of it's
  existence and therefore no edges from the existing `radiant` nodes will be incident to it.
- holding with the above, the degree (in-degree + out-degree) of any given node in the graph must at all times
  be at least 1 but no greater than `(n-1)*2`.
- Also following the above, the graph can be cyclic


## Association, communication, & data topologies

After system bringup, in the `bondsmith` role, a node will only ever communicate via RPC to an elected `herald`, 
where the `herald` must "register" itself with the `bondsmith` after a successful election. Therefore edges from the
`bondsmith` are bi-directional but only incident to `herald` nodes. 

Within `orders`, only the `order`'s `herald` maintains edges to the `radiants` that make up the `order`. This gives the 
system's authority/command structure a star-like topology, or a tree, where the `bondsmith` is the center/root, with 
edges to `heralds`, and each `herald` having multiple edges incident to the `radiants` in it's `order`. Note that this
is the formation post-bootstrap, as during bootstrap a node will spin up and reach out to any "neighbor" node provided
in it's config file. After bootstrap, any commands re: configuration changes or data movement, including association
probing, will follow this structure. 

### Data movement

This will be part of the generic replication design-- the following possibilities can be implemented (need to research/
read more about this)

1. When `shard` data must move, the requester routes the data: a `bondsmith` requests to a `herald`, a `herald` requests 
to the `radiants`, the data moves from `radiant` to `herald` to `bondsmith` then back to wherever the data is destined
(either to a client or to another `order` via `herald`) 