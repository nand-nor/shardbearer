# Shardbearer: Sharded Storage Service

:warning:!:warning:! :construction: THIS IS A WIP! Most features are still under development :construction: !:warning:!:warning:

A for-fun-and-learning project where I attempt to build a distributed sharded storage service using the `raft` consensus algorithm.
For the problem specification I am using this [distributed systems course 
final lab](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html) framework. (Note: Im not enrolled in the course, I 
just like the problem challenge). I have also changed/added a few things in an attempt to make the system more 
fault tolerant/recoverable, decentralized, and autonomous. 

The general naming theme is inspired by [Brandon Sanderson's Stormlight Archive](https://www.brandonsanderson.com/) 
book series. No spren were harmed in the making of this code

## Project Status

-[x] RPC definitions, service impls, data types, general structure \
-[] System setup: bootstrapping, election (voting), role assignment (must handle all entry cases: new system, 
multiple new joins, single new join) \
-[] `shard` controller `herald` methods: load-rebalancing/system reconfig \
-[] `shard` data replication within `orders` \
-[] heartbeat checks and role-reassignment if needed \
-[] benchmarking \
-[] stress testing and performance tuning


## Project Description

In the simplest terms, the `shardbearer` crate and the sub-crates within provide a framework for building and launching
 a sharded key-value storage system. The `shardbearer` system uses the `raft` consensus algorithm to replicate `shard` 
data within replica groups (a.k.a. `orders` of `radiants`), to elect `order` leaders (`heralds`), and to elect (from 
the pool of `heralds`) a top level `shard` controller `herald`. For a full description of `shards`, `radiants`, 
`orders`, and `heralds`, see [the code architecture section](#code-architecture).

Users of the `shardbearer` repo can compile, configure, and launch a sharded key-value storage system that is fault 
tolerant, semi-decentralized, fully distributed, and slightly more autonomous than not. When fully compiled, the binaries 
produced are intended to be distributed across multiple servers, executing as system services. While this initial 
setup may require some human-configuration input, the system is designed to run semi-autonomously as well as be 
responsive to human (sysadmin) commands.

Some things in the problem spec have not been implemented, or have been changed; 
see the next section [departures from the problem spec section](#departures-from-the-problem-spec).


### Departures from the Problem Spec

I have made a few changes/optimizations to make the system a bit more interesting (in my humble opinion). One challenge
I see to systems like these is ensuring fault tolerance and recoverability while still maintaining some level of
decentralization.  

Another interesting feature I want this system to have is autonomy. So increasing decentralization and enabling 
more advanced autonomous operation are the two main departures from the problem spec. 

#### Increase Decentralization

I have attempted to add logic to enable the system to recover from cases where nodes experience system failure in
 an unplanned-for manner. Nodes can explicitly leave the system, for example if there is some fault detected that
requires the node to either restart the service or restart the entire server. Nodes can also go down without
any notice to the rest of the cluster; `heralds` have functionality to periodically poll the `radiant` members of
the `order` for which they are responsible, and `heralds` can also poll each other. 
The logic I have attempted to add is intended to enable any server at any time to take 
on any role in the system depending on these polling operations. 

This means that there is no separate `shard` controller service: a `shard` controller is 
elected from the available `heralds` and any `herald` is compiled with the code needed to execute that role. 
Furthermore, any `radiant` at any time can become a `herald`, so any `radiant` can take on the controller role as long
as it has first taken on the `herald` role. 

#### Increase System Autonomy

To increase the system's ability to execute more autonomously, I have attempted to remove the need for any
human input. A new node in the system can be added without input from a 3rd party (such as a system administrator) 
as long as it has the address of at least one other node in the system. As part of the runtime configuration file 
provided to a `radiant`, the address of another member of the system must be included. This address will be used 
for system association, and the overall system association graph must meet the requirements/ specified invariants 
listed in the  [Association Graph Invariants](#association-graph-invariants). This address will be used to enter
the system via a series of RPCs sent throughout the system. 

The functionality required to enable outside-system input (via human sysadmin or otherwise) is also retained to enable 
the add/remove actions for `radiant` nodes. This is done for testability and also as a failsafe (no system that is 
fully autonomous should run without some kind of killswitch)

A system of at least two nodes can bootstrap itself (ideally) just from being able to send an RPC to
at least one other cluster member. 

Generally I have ignored any specification that seems to be purely for enabling automated testing/grading of the lab. 
Things that made sense for general system testing/integration have been kept. 

### Capabilities and Limitations 

#### Capabilities

The `shardbearer` crate is designed to enable an end user to define at compile time the key-value data types, 
using Rust's generic type system. In this way the system can store an arbitrary set of key-value pairs depending 
on the end-user's needs. TODO keys must be fixed but values can possibly be dynamic types?

The system supports any number of nodes in the system greater than 3 but less than some large number (need to test this) 
as a function of the memory required to maintain the state needed to track membership etc. 

The system assumes that all nodes in the cluster will be running a linux based distro that supports `tc`! For when
the system is ready for some custom `qdisc` classifiers (this is a long way off)

#### Limitations

There are lots of limitations as this is just a for-fun project that I am using to teach myself some distributed
system theory and work on build `async` Rust skills. Lots of things discussed in this README are under development
even though the wording may sound like the system does these things already. 
TODO comprehensive list of limitations

### Code Architecture
The following sub-crates are part of the `shardbearer` crate/build system:
- `shardbearer-proto` where the protocol buffer definitions are generated
- `shardbearer-core`  where all of the traits are defined
- `shardbearer` pulls together the protos,  traits, and everything else to define the needed services
- `shardbearer-shardkv` (currently empty)

Maybe more to come, TBD! TODO describe whats in each

The main components giving structure to this system are `shards`, `radiants`, `orders`, and `heralds`. 

- A `shard` is a subset of the key-value store that a given `shardbearer` system instance is serving. The size
(i.e. the total size of key-value pairs in bytes as stored on disk) of any given `shard` is a function of the  
available disk space on each `radiant` node (for simplicity I assume this is consistently the same* across all
`radiant` nodes) the size of an `order` (also for simplicity I assume this is always the same rounding down if
there is an odd number of `radiant` nodes i.e. if there are two `orders` and 7 nodes then the odd-node-out is not
factored into calculations* and each `order` is treated as if it is 3 members for load balancing) and the total amount
of data in bytes contained across all `shards`. 

- A `radiant` essentially represents an individual server instance responsible for serving the data
contained in a `shard`. The data can be an entire `shard` or multiple `shards`, depending
on current load, group membership (detailed further below), and system resources. Each `radiant` is part of an `order`,
which is effectively a replica group.

- An `order` is a group of `radiants` responsible for serving requests to create/remove/update values in one or
more`shards`. An `order` is the term applied to a replica group, composed of two or more `radiants`, responsible for 
one or more `shards`. An `order` instance uses the `raft` consensus algorithm to replicate the group's `shards` 
across all `radiants` in the `order`. Similarly, a leader or `herald` must be elected to lead each `order`.

- A `herald` is elected within an `order` to act as the leader of the replica group. A `herald` is a covariant of
a `radiant`, meaning all `heralds` are `radiants` but not all `radiants` are `heralds`. A `herald` can have one of two 
roles: it will either be a leader of it's `order`, or, if elected, it will take on the role of the controlling 
coordinator `herald` and a new `herald` of it's `order` must be elected. Therefore, a `shardbearer` system can have
multiple `heralds` but only one `herald` instance will act as the controlling coordinator (the `shard` controlling 
`herald`).  Non-controlling `heralds`act as top-level control for one or more `orders`. They coordinate with the 
controlling `herald` to maintain distributed state for the system. The controlling `herald` in a `shardbearer` system 
is the point from which `shards` are distributed across all servers participating. It is responsible for 
determining which `orders` serve which `shards` and effectively performs dynamic 
load balancing as `radiants` leave and join the system. The `heralds` in a `shardbearer` system also use the
`raft` consensus algorithm to replicate the state data that determines what information is stored where. 
This enables the system additional fault-tolerance: if one `herald` goes down, another `herald` can step in
and perform the coordination needed by the system 

*later optimizations will treat this in a more complex way e.g. not fixed/the same across the whole system

Per the [problem spec](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html), the `shard` controller `herald` is 
responsible for managing configuration: as new servers (`radiants`) enter and leave the system, `shards` must be 
redistributed to balance the load dynamically. In this way, the `shard` controller `herald` provides the needed
information to the `orders` (replica groups) to determine what `shards` to serve. The `shard` controller `herald` 
also acts as a client gateway: clients consult the `shard` controller to be routed to the `order` responsible for 
the `shard` they are attempting to read/write. 

#### Variance: System Roles and Relationships

In this system, all nodes are `radiant`, e.g. `radiant` is the supertype trait that all nodes 
implement, and `radiant` is the role that all nodes in all states execute in.

Depending on the voting results in the system, a `radiant` can become the `herald` of it's `order`. So, included
in each `radiant` binary is the ability to, at runtime, create a dynamic trait object that enables a `radiant` 
to call the `herald` trait methods. Furthermore, A `herald` at any time has the needed logic to enable it to become 
the `shard` controller `herald` based on the results of a `herald`-only vote.   

The following relationships hold:
- All nodes are `radiant`, e.g. `radiant` is the supertype trait that all nodes implement
- All `heralds` are `radiant` but not all `radiants` are `heralds`
- All `radiants` belong to an `order`
- A `shard` controller `herald` is a `radiant` that belongs to an `order` but that has switched into a special `herald` mode
to serve as the elected coordinator for all `shards` across all `orders`. Such an order will need a new `herald` (so
the minimum size of any `order` must be 3.)

### System Bootstrapping, Elections, and Role Assignment
For this reason, on system bootstrap, the voting order (from the pool of all currently registered `radiants`) and 
establishment of memberships etc., is:
1. Bootstrap members initiate a vote to elect `shard` controlling `herald`
2. Controlling `herald` determines the number of `orders` and then assigns each known `radiant` to them. (To Do: 
determine if we are building a `shardbearer` system from an existing database that we will pull into `orders` versus
starting empty, with one `order`, and then growing / rebalancing and the underlying key value store grows?) 
For simplicity/to start, the underying key-value store will always be empty so no matter how many nodes join at
bootstrap, there will only ever be one `order`
3. `Order` members are distributed `radiant` identifiers (metadata like `<ip>`:`<port>`) from the controlling `herald` 
to populate their lookup tables with known members. Heartbeat checks are set up to periodically poll members
(Eachother? or should only the `herald` do this? TODO consider how this impacts autonomy) 
4. `Order` members initiate a vote to determine the `order`'s `herald` 
5. `heralds` report to the `shard` controlling `herald`     

#### Assigning Bootstrap Configuration Parameters

Each binary produced (by default in the basic `radiant` state) must be provided with a set of parameters as a
toml file to be parsed at runtime. 

##### Association Graph Invariants

The association graph must hold the following invariants:

- edges are directed 
- no self-loops
- each node must have at least one edge incident from (leaving) itself and incident to (entering) another node (not itself)
 Put more simply, a `radiant` node must have a directed edge pointing to a neighbor. 
- each node can have `t` incident-from edges (leaving) from other nodes where `t` falls in the range`0<=t<=n-1`  
and `n` is the number of `radiant` nodes currently in the system. Put more simply, multiple `radiant` nodes can
have an edge pointing to a single `radiant` node, and a `radiant` node can also have no edges incident to (entering) it,
but no node can point to itself. The case where a `radiant` node has `0` incident-to-itself edges will arise when a new 
`radiant` enters an already-established system; the other nodes in the system will have no prior knowledge of it's 
existence and therefore no edges from the existing `radiant` nodes will be incident to it. 
- holding with the above, the degree (in-degree + out-degree) of any given node in the graph must at all times 
be at least 1 but no greater than `(n-1)*2`.
- Also following the above, the graph can be cyclic

## Dependencies and 3rd Party Libraries

This service will largely rely on the `gRPC-rs` framework, `tokio-rs`, and `async-raft`  or maybe just `raft` TDB.
Need to determine if the changes in system autonomy and fault tolerance/recoverability require a slightly 
different-than-what-exists `raft` implementation.

All interactions between nodes in the system and between client and system utilize `gRPC-rs`. 

This crate leverages `tokio-rs` to provide asynchronous execution and configurable levels of tracing/logging. 
The system is set up to enable end-user to specify the desired parameters for a `tokio::runtime`  

## TO DO / Future Work

- Add in more security features for RPC (tls?) using `cfg` macros
- Utilize `cargo bench` to profile individual node service performance
- Test how this scales in simulation (possibly using `mininet` or raspberry pi cluster testbed) to tens of
server nodes, maybe hundreds? May need a beefy many-core AWS server instance to run `mininet` for that
- Use more intelligent sync capabilities, possibly ieee1588 PTP protocol?
- Determine best way to perform association graph setup:  provide some script with a list of all the `ip`:`port` pairs of 
all the nodes that will be set up in the initial system (must be greater than or equal to 2). The script will then
generate a graph to assign each node an initial beacon address. 
- Make the algorithm used for consensus generic / dynamic? Why just use raft?
- set up simple `TC` `ebpf` program to drop all traffic not from approved subnets (to increase security while
still enabling unknown members to join )

### Performance Tuning / Security

This system can possibly benefit from individual nodes loading custom `qdisc` implementations for ingress/egress
or some kind of full-linux-network-stack-traffic bypass with netfilter/netlink sockets? 

At some point this repo will provide a simple `ebpf`-based `qdisc` classifier for ingress that 
can be loaded into `tc` to provide somewhat better performance/security to whitelist only approved
subnets. A way to blacklist most traffic except for new nodes attempting to join the system would be
 smart to implement as well..

And at a later point an egress classifier to help with routing within the cluster in a semi-ordered fashion
(assigning higher priority to egress packets marked for replicating `shard` data, for example). This will need some
experimentation

## References
- [Lab specification](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html)
- [Flat Datacenter Storage](https://www.usenix.org/system/files/conference/osdi12/osdi12-final-75.pdf)
- [The Raft Consensus Algorithm](https://raft.github.io/)
- [The Stormlight Archives](https://www.brandonsanderson.com/books-and-art/#cosmere) 