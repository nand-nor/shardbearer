# Clients of a `Shardbearer` system

As long as a client has the address of at least one currently active `radiant` node in the 
cluster, then the service will be able to serve the client request.

In this way, a client does not need to know the address of the `shard` controlling  `herald`
in advance, because it can request that information from the known-`radiant` node.

## User interface TODO
Need to determine best method to handle this, balancing complexity of the implementation and 
UI i.e. benefits from a useability perspective. I see the following possibilities:

1. allowing a client to make a request to any node and having said node act as a proxy 
between client and `shard` controlling `herald` 
2. allow client to first request the CID (controller ID) of the elected leader and then
make whatever request. This puts more burden on the client   