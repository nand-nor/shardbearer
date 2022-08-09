use super::MemberID;
use serde::{Deserialize, Serialize};

pub trait ShardbearerConsensus {}

pub trait ShardbearerReplication {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsensusCfg {
    id: MemberID,
    election_timeout: u64,
    heartbeat_interval: u64,
}

impl ConsensusCfg {
    pub fn id(&self) -> MemberID {
        self.id
    }
    pub fn election_timeout(&self) -> u64 {
        self.election_timeout
    }
    pub fn heartbeat_interval(&self) -> u64 {
        self.heartbeat_interval
    }
    //pub fn replication_max_bytes(&self) -> usize {
    //   self.replication_max_bytes
    //}
}

/// The `ReplicationCfg` trait is an optional configuration parameter that enables
/// the use of additional params for setting up consensus/replication
/// in a generic shardbearder system
pub trait ReplicationCfg {
    //: Serialize + Deserialize + Clone  {
    type ConfigParams;
    type ReplicationLimits;

    fn get_limits(&self) -> Self::ReplicationLimits;
    fn get_params(&self) -> Self::ConfigParams;
}

/// `DefaultReplicationCfg`: Example wrapper struct with parameters & replication
/// limits similar to (but not a complete set) those used by raft
///
/// Intended to illustrate the use of the `ReplicationCfg` trait
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultReplicationCfg {
    params: DefaultReplicationCfgParams,
    limits: DefaultReplicationCfgLimits,
}

/// Example wrapper struct for simple config
/// parameters (used in `DefaultReplicationCfg` struct)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultReplicationCfgParams {
    //multistage_commit: bool,
    role_prioritization: bool,
    broadcast_commits: bool,
}

/// Example wrapper struct for simple replication
/// parameters (used in `DefaultReplicationCfg` struct)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultReplicationCfgLimits {
    max_inflight_msgs: usize,
    max_bytes_per_msg: usize,
    replication_max_bytes: usize,
}
/// Example implementation of the ReplicationCfg trait
/// for the `DefaultReplicationCfg` struct
impl ReplicationCfg for DefaultReplicationCfg {
    type ConfigParams = DefaultReplicationCfgParams;
    type ReplicationLimits = DefaultReplicationCfgLimits;

    fn get_limits(&self) -> Self::ReplicationLimits {
        self.limits.clone()
    }
    fn get_params(&self) -> Self::ConfigParams {
        self.params.clone()
    }
}
