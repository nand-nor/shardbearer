# Shardbearer: Generic Sharded Storage Service

:warning:!:warning:! :construction: THIS IS A WIP! Most features are still under heavy development :construction: !:warning:!:warning:

A for-fun-and-learning project intended to facilitate my personal interest in learning distributed systems/
back-end engineering fundamentals by building a *generic* distributed sharded storage service. The naming theme 
of the project is inspired by [Brandon Sanderson's Stormlight Archive](https://www.brandonsanderson.com/)
series. No spren were harmed in the making of this code!

## Project Description

The `shardbearer` project is loosely based on the problem specification from a [distributed systems course
final lab](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html). There are a number of departures from 
the problem specification, mainly in that I am focusing on designing a system that is not specific to the `raft` 
algorithm and focuses more on making the system generic. Ultimately this project is intended to provide myself 
a framework to explore and evaluate different approaches to replication and consensus in distributed systems. 
At the same time it represents my attempt to design a modular, user-extensible / user-customizeable, highly 
configurable, distributed, somewhat autonomous, somewhat fault tolerant/recoverable, sharded key-value storage system. 

To make the system generic, I am leveraging Rust's powerful type system for generics/dynamic dispatch/traits, 
making both the protocols for consensus (and/or optionally replication) and the data stored in the shards generic.
Utilizing something like dependency injection, a user can implement or plug in objects for which they have implemented
the appropriate traits for consensus and/or optionally for data replication, which can be provided dynamically at 
runtime (or optionally at compile time). Given these end-user-provided implementations, `shardbearer` provides a
framework for handling system bring up/bootstrapping, system configuration, system membership & election (i.e. 
formation of `shard` groups and electing nodes to roles within the system), and general system maintenance. 

The `shardbearer` framework uses `gRPC` for system bringup/bootstrapping, voting/role election, the formation 
of `shard` groups, and generally for the majority of communication between nodes in the system. 

The system should be able to support any number of nodes greater than 3 and less than some large number 
(can arbitrarily say 100 as the max possible number of servers within a `shardbearer` system, but really it should be
 a function of the memory/disk space required to maintain state and data structures etc. for tracking
membership along with providing ample room for the actual `K`,`V` store, robust/verbose logging, and any other
facilities needed for performance tuning/monitoring). 

### Naming & high-level structure

Each node in the system is referred to as a `radiant`. The `radiants` make up `shard` groups, with each `shard` group
represented by a `herald`, a `radiant` that is elected at random* during bootstrap. The `herald` nodes all report to/
coordinate with a top-level `shard` controlling `herald` called a `bondsmith` (named for a type of herald that are very rare/big
boss kind types). The `radiant`, `herald`, and `bondsmith` roles are defined as traits that represent each role, with
a subtyping structure to enable any node to take on any role at any time.  

(*something something pseudo random number not really random something something make better later)

For a full description of system structure and further terminology see [the code architecture doc](./docs/ARCHITECTURE.md).


### Repo structure

The `docs` dir provides a lot more details but is largely in need of cleanup. 

The following sub-crates are part of the `shardbearer` crate/build system:
- `shardbearer-proto` where the protocol buffer definitions are generated
- `shardbearer-core`  where all of the traits are defined and some simple/example/default trait implementations provided
- `shardbearer-sys` is the crate that exposes a user API and further wraps the implementations of `shardbearer-core`,
`shardbearer-proto`, in methods that provide easy building blocks to customize and further
 define services


Maybe more to come, TBD! TODO describe whats in each
 Also tests and examples need descriptions once they are more fully implemented!

## Project Status

Very much a WIP! A good reference point to check this are the open git issues I am using to track my TODOs, but
those are not nearly comprehensive enough. 

- [x] Initial RPC definitions, service impls (just enough to provide scaffolding)
- [ ] Generic structure for server setup: requires trait definitions (just enough for initial scaffolding)
- [ ] System setup:
  - bootstrapping,
  - general configuration around replication / consensus
    - toml parsing utility and generic config traits where needed
  - role election (voting), role assignments handling all entry cases:
    - new system (no pre-existing roles, heralds and bondsmiths will be randomly selected at runtime)
    - multiple new joins
    - single new join
- [ ] System maintenance / membership handling 
  - Soft and hard resets 
    - order reset
    - system reset
    - shard soft reset (rollback to some last indicated good state on the event of some error arising within shard group)
- [ ] `shard` management methods: generic trait definition and provide some examples
- [ ] `serde` traits for shards (for enabling generic `K`,`V` passing in protobuf messages)
- [ ] `shard` controller `herald` (`bondsmith`) methods:
  - load-rebalancing
  - system reconfig
  - dynamic membership support
- [ ] heartbeat checks and role-reassignment if needed
- [ ] Documentation



## References
- [Lab specification](https://pdos.csail.mit.edu/6.824/labs/lab-shard.html)
- [Flat Datacenter Storage](https://www.usenix.org/system/files/conference/osdi12/osdi12-final-75.pdf)
- [The Raft Consensus Algorithm](https://raft.github.io/)
- [The Stormlight Archives](https://www.brandonsanderson.com/books-and-art/#cosmere) 