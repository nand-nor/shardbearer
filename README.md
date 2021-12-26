# Shardbearer: Sharded Storage Service

A for-fun-and-learning project where I attempt to build a distributed sharded storage service using the `raft` consensus algorithm.
For the problem specification I am using this [distributed systems course 
final lab](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html) framework. (Note: Im not enrolled in the course, I 
just like the problem challenge). I have also changed/added a few things in an attempt to make the system more 
fault tolerant/recoverable, decentralized, and autonomous. 

The general naming theme is inspired by [Brandon Sanderson's Stormlight Archive](https://www.brandonsanderson.com/) 
book series. No spren were harmed in the making of this code

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
decentralization.  Another interesting
feature I want this system to have is autonomy. So increasing decentralization and enabling more advanced autonomous
operation are the two main departures from the problem spec. 

#### Increase Decentralization


Similarly, I have added implementations to enable the system to
recover from cases where nodes experience system failure in an unplanned-for manner. Nodes can explicitly leave
the system, for example if there is some fault detected that requires the node to either restart the service or 
restart the entire server. Additionally, `heralds` have functionality to periodically poll the `radiant` members of
the `order` for which they are responsible, 

 
 is that I have implemented the code so that any server at any time can take 
on any role in the system. This means that there is no separate `shard` controller service: a `shard` controller is 
elected from the available `heralds` and any `herald` is compiled with the code needed to execute that role. 
Furthermore, any `radiant` at any time can become a `herald`, so any `radiant` can take on the controller role as long
as it has first taken on the `herald` role.  

The goal being to avoid creating a system that has any single point of failure by enabling 



Each replica group leader 
additionally votes to elect a top level `shard` controller `herald`. 


#### Increase System Autonomy

To increase the system's ability to execute more autonomously, I have attempted to remove the need for any
human input, like that from a sysdmin. 

A new node in the system can be added without input from a 3rd party (such as a system administrator) as long as it
has the address of at least one other node in the system. As part of the runtime configuration file provided to
a `radiant`, the address of another member of the system must be included. This address will be used for system
 association, and the overall system association graph must meet the requirements/ specified invariants 
listed in the  [Association Graph Invariants](#association-graph-invariants). This address will be used to enter the system
via a series of RPCs sent throughout the system. 

The functionality required to enable outside-system input (via human sysadmin or otherwise) is also retained to enable 
the add/remove actions for `radiant` nodes. This is done for testability and also as a failsafe (no system that is 
fully autonomous should run without some kind of killswitch)

A system of at least two nodes can bootstrap itself (ideally) just from being able to send an RPC to
at least one other cluster member

Generally I have ignored any specification that seems to be purely for enabling automated testing/grading of the lab. 
Things that made sense for general system testing/integration have been kept. 



### Capabilities and Limitations 

#### Capabilities

The `shardbearer` crate is designed to enable an end user to define at compile time the key-value data types, 
using Rust's generic type system. In this way the system can store an arbitrary set of key-value pairs depending 
on the end-user's needs. TODO keys must be fixed but values can possibly be dynamic types?

The system supports any number of nodes in the system greater than 2 but less than some large number (need to test this) 
as a function of the memory required to maintain the state needed to track membership etc. 

The system assumes that all nodes in the cluster will be running a linux based distro that supports `tc`!

#### Limitations

There are lots of limitations as this is just a for-fun project that I am using to teach myself some distibuted
system theory and work on build `async` Rust skills. 
TODO comprehensive list

#### Optional features and future work 

 At some point this repo will provide a simple `ebpf`-based `qdisc` classifier for ingress that 
can be loaded into `tc` to provide somewhat better performance/security to whitelist only approved
subnets. And at a later point an egress classifier to help with routing within the cluster in a semi-ordered fashion
(assigning higher priority to egress packets marked for replicating `shard` data, for example). This will need some
experimentation

~~It also utilizes Rust's conditional compilation system to provide optional build features that add more~~

### Code Organization / Architecture
The following sub-crates are part of the `shardbearer` crate/build system:
- `shardbearer-proto`
- `shardbearer-core`

Maybe more to come, TBD!

The main components giving structure to this system are `shards`, `radiants`, `orders`, and `heralds`. 

- A `shard` is a subset of the key-value store that a given `shardbearer` system instance is serving. The size
(i.e. the number of key-value pairs it contains) of any given `shard` is a function of the available disk
 space of any given server in the system (for simplicity I assume this is consistently the same*)
 across all on number of servers
in the system and the total amount of data contained across all shards. 

- A `radiant` essentially represents an individual server instance responsible for serving the data
contained in a `shard`. This can be a subset of a `shard`, an entire `shard`, or multiple `shards`, depending
on current load, group membership (detailed further below), and system resources. Each `radiant` is part of an `order`.

- An `order` is a group of `radiants` responsible for serving requests to create/remove/update `shards`. An
`order` is effectively a replica group, composed of two or more `radiants`, responsible for one or more `shards`.
An `order` instance (replica group) uses a consensus algorithm to replicate the group's `shards` across
all `radiants` (servers) in the given `order` (replica group). 

- A `herald` is elected within an `order` to act as the leader of the replica group. A `herald` is a covariant of
a `radiant`, meaning all `heralds` are `radiants` but 
can have one of two roles depending on how many
`heralds` are launched. A `shardbearer` system can have multiple `heralds` but only one `herald` instance will act
 as the controlling coordinator. 
 Non-controlling `heralds`act as top-level control for one or more `orders`. They coordinate with the controlling
 `herald` to maintain distributed state for the system. The controlling `herald` in a `shardbearer` system 
is the point from which `shards` are distributed across all servers participating. It is responsible for 
determining which `orders` serve which `shards`. The controlling `herald` effectively performs dynamic 
load balancing as `radiants` leave and join the system. The `heralds` in a `shardbearer` system also use the
`Raft` consensus algorithm to replicate the state data that determines what information is stored where. 
This enables the system additional fault-tolerance: if one `herald` goes down, another `herald` can step in
and perform the coordination needed by the system 

*later optimizations will treat this as a value to be set individually i.e. not fixed/the same across all nodes
but something provided during the configuration stage when a `radiant` enters the system. 

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
All nodes are `radiant`, e.g. `radiant` is the supertype trait that all nodes implement
All `heralds` are `radiant` but not all `radiants` are `heralds`
All `radiants` belong to an `order`
A `shard` controller `herald` is a radiant that belongs to an order but that has switched into a special `herald` mode
to serve as the elected coordinator for all `shards` across all `orders`  

So the relation `R` between `radiants` and `heralds` is transitive, non-reflexive, and non-symmetric (does this matter). 
 An interesting thing in Rust is the notion of variance and supertypes/subtypes. 


### Execution flow

A server is loaded with a `radiant` binary compiled with the appropriate types for the key-value store. 
Use the *.service definition to set up the binary to load & run at
 a specific time as part of something like `systemd`. 
 
 Upon entry, it reads a toml config file to parse the following 
```- port
- ip address
- name
- permissions
- heartbeat interval
- beacon interval
- initial beacon ip address of -at least one other node in the system- 
- initial election seed?
- what else?
```
It then starts to send out a beacon handshake message at `beacon interval` to it's assigned beacon IP address, and then waits for a 
beacon response. `BeaconHandshake` sending some metadata to the neighbor (TBD but might include ip/port)
Upon success, will receive a `BeaconResponse` that will assign the joiner with a temporary member ID and the
joiner will temporarily be added to the group that the neighbor belongs to (if one exists).

The beacon response is then followed up by an explicit `JoinSystem` RPC, which is relayed up to the top level
controller, and upon success the current system state and membership information is exchanged. 
So, a system admin can directly add a new `radiant` via direct call to the controller, or a `radiant` 



Each `radiant` maintains a lookup table to store the address of every other `radiant` it receives a beacon or
other message from. Each `radiant` also maintains basic information regarding the current `order` it belongs to in
the current configuration, storing the set of `radiants` in it's own `order` (replica group) and the `shards` it's
`order` is currently responsible for serving. Each `radiant` holds the information needed to become a `herald` of
it's own `order` at any time, in case the `order's` `herald` goes offline for any reason, any `radiant` within the `order`
can be elected to act as `herald`. 
to hold the information that `radints` share with eachother i.e. a `radiant` can query another `radiant` to
share the addresses it currently has stored

The following scenarios are handled: 

- Everyone is new: this is the first instance of the system and no `orders` have been established and no `heralds` 
nor a `shard` controller have yet been elected. The beacon response indicates that this is the current state of the 
system (at least from the point of view of the responder). 
This requires that, as part of the config, initial beacon ip addresses are assigned to each node in the system
such that the specific configuration parameter invariants hold. For more details see the 
[Assigning Configuration Parameters](#assigning-configuration-parameters)'s
subsection [Association Graph Invariants](#association-graph-invariants).

- Multiple new nodes join an established system 
A system is considered "established" when the following states are met: the system has least 2 nodes currently running,
 they have passed the beacon phase & established handshakes, & all roles have been elected and voted on.

Possible enhancement: When the number of new nodes joining a pre-established system doubles the total nodes in the system,
all `orders` are re-organized and existing roles are relinquished so new `heralds` and a `shard` controlling `herald`
are voted on and established.   
  
-A single new node joins an established system. 

The initial beacon emission and beach response is an RPC call. 

## Maintaining State

Each `radiant` maintains the following state information:
- basic self-state information (current role, disk space used/free, add more later)
- current configuration of `shards` the `order` the `radiant` belongs to is responsible for
- current `order` id
- current `order` membership e.g. addresses of all `radiants` in the `order` (replica group)
- current `order` elected `herald`
- list of `orders` in the system and their associated `herald`
- the `shard` controlling `herald`

- Possibly also a lookup table to store the address of every other `radiant` known to exist in the system, currently

Each `radiant` also maintains basic information regarding the current `order` it belongs to in
the current configuration, storing the set of `radiants` in it's own `order` (replica group) and the `shards` it's
`order` is currently responsible for serving. 

Each `radiant` holds the information needed to become a `herald` of it's own `order` at any time, in case the
 `order's` currently elected `herald` goes offline for any reason, any `radiant` within the `order`
can be elected to act as `herald`. 


## Assigning Configuration Parameters

Each binary produced (by default in the basic `radiant` state) must be provided with a set of parameters as a
toml file to be parsed at runtime. 

### Association Graph Invariants


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
- Also following the above, the graph can be cyclic (TODO this may be too permissive, need to consider pros/cons)

## Dependencies

This service will largely rely on the `gRPC-rs` framework, `tokio-rs`, and `async-raft` 

All interactions between nodes in the system and between client and system utilize `gRPC-rs`. 

This crate leverages `tokio-rs` to provide asynchronous execution and configurable levels of tracing/logging.  


## Performance Tuning

This system can possibly benefit from individual nodes loading custom `qdisc` implementations for ingress/egress


## TO DO

- Add in more security features for RPC (tls?) using `cfg` macros
- Utilize `cargo bench` to profile individual node service performance
- Test how this scales in simulation (possibly using `mininet` or raspberry pi cluster testbed) to tens of
server nodes, maybe hundreds? May need a beefy many-core AWS server instance to run `mininet` for that
- Use more intelligent sync capabilities, possibly ieee1588 PTP protocol?
- Determine best way to perform association grph setup:  provide some script with a list of all the ip:port pairs of 
all the nodes that will be set up in the initial system (must be greater than or equal to 2). The script will then
generate a graph to assign each node an initial beacon address. 
- Make the algorithm used for consensus generic / dynamic? Why just use raft?
- setup service skeleton 
- set up toml parsing
- set up simple TC ebpf program to drop all traffic not from approved subnets (to increase security while
still enabling unknown members to join )

## References
- [Lab specification](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html)
- [Flat Datacenter Storage](https://www.usenix.org/system/files/conference/osdi12/osdi12-final-75.pdf)
- [The Raft Consensus Algorithm](https://raft.github.io/)
- [The Stormlight Archives](https://www.brandonsanderson.com/books-and-art/#cosmere) 