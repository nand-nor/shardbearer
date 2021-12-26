# Radiant 

# Bootstrapping: Execution flow

A server is loaded with a `radiant` binary compiled with the appropriate types for the key-value store. 
Use the *.service definition to set up the binary to load & run at a specific time as part of something like `systemd`.

Assume all nodes in the cluster are somewhat in sync. Maybe using PTP? Not too much clock drift 

 
 
Upon executing it's `start` entry point, read a toml config file to parse the following 
```
- port
- ip address
- name
- random bootstrap backoff time value
- permissions
- heartbeat interval
- beacon interval
- initial beacon ip address of -at least one other node in the system- 
- initial election seed?
- what else?
```

Setup radiant server to receive RPCs from other nodes (set up self base radiant server capabilities), set up data 
structures, set up system information data, etc. All initial self set up done at this point. 

Launch another thread. Set timer to wait whatever amount of time defined by the bootstrap backoff value. 
At expiration, check self state. If we have already gotten a hello from the neighbor we are assigned, then
the neighbor will "be in charge" (need this to break the case where we have a two node cycle in the
system association graph) and we go to the next setup phase (`herald` election). Otherwise, setup RPC client and 
send out the hello. Based on the information received back, do the following



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
