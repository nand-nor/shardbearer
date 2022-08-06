# Shardbearer: Gneric Sharded Storage Service

:warning:!:warning:! :construction: THIS IS A WIP! Most features are still under development :construction: !:warning:!:warning:

The naming theme is inspired by [Brandon Sanderson's Stormlight Archive](https://www.brandonsanderson.com/)
series. No spren were harmed in the making of this code

## Project Description

A for-fun-and-learning project where I attempt to teach myself some distributed system fundamentals by building
a *generic* distributed sharded storage service, loosely based on the problem specification from [distributed systems course 
final lab](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html) framework. There are a number of 
departures from the problem specification, mainly in that I have attempted to design something
that is generic and not specific to the `raft` algorithm. 

Ultimately the `shardbearer` project is intended to provide myself a framework to facilitate learning/evaluating
distributed systems fundamentals, such as the different approaches to replication in distributed systems. I have 
attempted to design a system that takes advantage of rust's generic type system, traits, and dynamic dispatch to
create a modular, user-extensible system, with special focus on providing trait definitions that will 
encourage/enforce system implementations that are fault-tolerant/recoverable, decentralized, and autonomous.

More concretely, the `shardbearer` project workspace can be used for building and launching a user-customizable 
sharded key-value storage system. The `shardbearer` project workspace, when complete (it's still WIP!) will enable 
a user/builder to implement or, if appropriate, plug in pre-existing consensus algorithms/protocols. Utilizing 
something like dependency injection, the consensus mechanism can be provided dynamically at runtime (or optionally 
at compile time). Along with a consensus protocol implementation, the end-user can implement various methods for 
data replication in a manner that is relevant to the system & the data it is storing. Given these end-user-provided 
implementations, `shardbearer` provides a framework for handling system bring up/bootstrapping, system configuration, 
and system membership & election i.e. formation of `shard` groups and roles within the system. 

The `shardbearer` framework provides server implementations leveraging `gRPC` for system bringup/bootstrapping, as well 
as for voting/role election/the formation of `shard` groups. During system bring up there is a bootstrapping period wherein system membership, `shard` group
formation, and role election is performed. The system uses a hierarchy of roles, similar to a tiering system, 
to coordinate activities within the system: there are `radiants`, `heralds`, and controlling `herald`s. 
Each node in the system is referred to as a `radiant`. The `radiants` make up `shard` groups, with each `shard` group 
 represented by a `herald`, a `radiant` that is elected at random during bootstrap. The `herald` nodes all report to/
coordinate with a top-level `shard` controlling `herald` (a.k.a. a `scherald`). The system supports any number of 
nodes in the system greater than 3 but less than some large number (need to test this) as a function of the memory 
required to maintain the state needed to track membership etc. For a full description of `shards`, `radiants`,
`orders`, and `heralds`, see [the code architecture doc](./docs/ARCHITECTURE.md).


### Repo structure

The `docs` dir provides a lot more details but is largely in need of cleanup. 

The following sub-crates are part of the `shardbearer` crate/build system:
- `shardbearer-proto` where the protocol buffer definitions are generated
- `shardbearer-core`  where all of the traits are defined
- `shardbearer` pulls together the protos,  traits, and everything else to define the needed services
- `shardbearer-state` is a crate for various state enums that are used in building state machines for 
coordinating system function (maybe rename)

Maybe more to come, TBD! TODO describe whats in each


## Project Status

Very much a WIP!

- [x] Initial RPC definitions, service impls (just enough to provide scaffolding)
- [ ] Generic structure for server setup: requires trait definitions (just enough for initial scaffolding)
- [ ] System setup:
 - bootstrapping,
 - role election (voting),
 - role assignment/resets-- must handle all entry cases:
  - new system,
  - multiple new joins,
  - single new join
  - order reset
  - system reset
 - general configuration around replication / consensus
- [ ] `shard` controller `herald` methods:
 - load-rebalancing
 - system reconfig
 - dynamic membership support
- [ ] `shard` data replication within `orders`
- [ ] heartbeat checks and role-reassignment if needed
- [ ] Documentation




## References
- [Lab specification](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html)
- [Flat Datacenter Storage](https://www.usenix.org/system/files/conference/osdi12/osdi12-final-75.pdf)
- [The Raft Consensus Algorithm](https://raft.github.io/)
- [The Stormlight Archives](https://www.brandonsanderson.com/books-and-art/#cosmere) 