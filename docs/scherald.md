# `Shard` Controller `Herald` Trait 

Details / notes on this trait

From the problem spec:

/*The Join RPC is used by an administrator to add new replica groups.
Its argument is a set of mappings from unique, non-zero replica group identifiers
(GIDs) to lists of server names. The shardctrler should react by creating a new
configuration that includes the new replica groups. The new configuration should
divide the shards as evenly as possible among the full set of groups, and should
move as few shards as possible to achieve that goal. The shardctrler should allow
re-use of a GID if it's not part of the current configuration (i.e. a GID should
be allowed to Join, then Leave, then Join again).*/

/*
The Leave RPC's argument is a list of GIDs of previously joined groups. The shardctrler
should create a new configuration that does not include those groups, and that assigns
those groups' shards to the remaining groups. The new configuration should divide the
shards as evenly as possible among the groups, and should move as few shards as possible
 to achieve that goal.
 */

/*

The Query RPC's argument is a configuration number. The shardctrler replies with
the configuration that has that number. If the number is -1 or bigger than the biggest
known configuration number, the shardctrler should reply with the latest configuration.
The result of Query(-1) should reflect every Join, Leave, or Move RPC that the shardctrler
finished handling before it received the Query(-1) RPC.

The very first configuration should be numbered zero. It should contain no groups,
and all shards should be assigned to GID zero (an invalid GID). The next configuration
(created in response to a Join RPC) should be numbered 1, &c. There will usually be significantly
more shards than groups (i.e., each group will serve more than one shard), in order that load
can be shifted at a fairly fine granularity.
*/
