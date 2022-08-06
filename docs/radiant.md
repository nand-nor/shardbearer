# Radiant 

## Internal State Machines

### Top Level RPC Control
This is the state machine that determines how a radiant handles RPCs from external nodes. External
nodes in this case include `order` local, system local, and external nodes (e.g. nodes looking to join the system
or the order)
<img title = "Radiant State Machine" alt="Radiant State Machine-- see .dot file" src=docs/radiant_states.png>


# Execution flow

A server is loaded with a `radiant` binary compiled with the appropriate types for the key-value store. 
Use the *.service definition to set up the binary to load & run at a specific time as part of something like `systemd`.

Assume all nodes in the cluster are somewhat in sync. Maybe using ieee1588 PTP? Or some global sync state
from the controller syncing on controller heartbeat? For now keep it simple & hope for not too much clock drift 
 
Upon executing it's `start` entry point, read a toml config file to parse the needed info to set up it's own
system and to join or bootstrap a new system. Setup radiant server to receive RPCs from other nodes (set up self
 base radiant server capabilities), set up data structures, set up system information data, etc. All initial 
 self set up done at this point. 

Launch another thread. Set timer to wait whatever amount of time defined by the bootstrap backoff value. 
At expiration, check self state. 
It starts to send out a beacon handshake message at `beacon interval` to it's assigned beacon neighbor IP address, 
and then waits for a 
beacon response. `BeaconHandshake` sending some metadata to the neighbor (TBD but might include ip/port)
Upon success, will receive a `BeaconResponse` that will assign the joiner with a temporary member ID and the
joiner will temporarily be added to the group that the neighbor belongs to (if one exists).

If we have already gotten a hello from the neighbor we are assigned, then
the neighbor will "be in charge" (need this to break the case where we have a two node cycle in the
system association graph) and we go to the next setup phase depending on the current system state. In a totally
new system (no node currently in the `INACTIVE` state has an edge incident to a node that is in a different state)
we then move on to voting ( at this point `herald` election then from `herald` pool the `controller`). Otherwise, 
the node is entering an existing system, so skip the voting setup RPC client and 
send out an explicit `JoinSystem` RPC, which is relayed up to the top level
controller, and upon success the current system state and membership information is exchanged. 
So, a system admin can directly add a new `radiant` via direct call to the controller, or a `radiant` 

More details later

Each `radiant` maintains a lookup table to store the address of every other `radiant` it receives a beacon or
other message from. Each `radiant` also maintains basic information regarding the current `order` it belongs to in
the current configuration, storing the set of `radiants` in it's own `order` (replica group) and the `shards` it's
`order` is currently responsible for serving. Each `radiant` holds the information needed to become a `herald` of
it's own `order` at any time, in case the `order's` `herald` goes offline for any reason, any `radiant` within the `order`
can be elected to act as `herald`. 
to hold the information that `radints` share with eachother i.e. a `radiant` can query another `radiant` to
share the addresses it currently has stored

## Identifiers

Member IDs, Group IDs, Leader IDs, Controller IDs

0 is a non-valid identifier for any identifier type. If a response is sent with a 0 value identifier, that
means that the identifier has not been set. Unless the corresponding state is UNASSOCIATED, this is an error

## Entry Cases

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


## `Radiant` traits and struct objects 

## `Radiant` objects

1. `RadiantController` --> this is the controller of state
When an RPC comes in that requires a modification to the state of the radiant, the radiant's system, the
radiants order, or anything relating to the cluster the radiant is a  member of, this is the 
object implementation that controls the thread(s) that receive messages from the RPC service impl
and handles them accordingly

2. `RadiantRpcClientHandler`
3. `RadiantService<K,V>`

### `RadiantService` Struct

``` 
pub struct RadiantService<K, V> {
    pub radiant: Arc<Mutex<RadiantNode<RadiantOrder, RadiantMembership, RadiantSystem>>>,
    pub neighbor: RadiantID,
    setup: bool,
    pub ctrl_chan_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
    pub herald: HeraldInfo,
    pub shard_map: Arc<Mutex<dyn ShardMap<K, V>>>,
}
```

#### RPC implementations
- `impl Herald for RadiantService`
- `impl HeraldController for RadiantService`
- `impl RadiantNode for RadiantService`



## Toml Configuration

- `my_ip`
- `my_port`
- `neighbor_ip`
- `neighbor_port`   
- `bootstrap_backoff`       //random seed to generate a backoff period for bootstrapping
- `election_timeout`        //tick interval for timeout of election
- `heartbeat_interval`     //interval at which heartbeats are sent in ticks
- `replication_max_bytes`  //max size of a replication chunk