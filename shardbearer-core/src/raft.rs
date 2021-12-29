use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::radiant::{GroupID, MemberID};
use crate::system::RadiantSystem;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use thiserror::Error;

/*
use simple_raft::log::mem::RaftLogMemory;
use simple_raft::node::{RaftConfig, RaftNode};
use simple_raft::message::{RaftMessage, RaftMessageDestination, SendableRaftMessage};
*/

use protobuf::Message as PbMessage;
use raft::storage::MemStorage;
use raft::{prelude::*, StateRole};
use raft::{raw_node::RawNode, Config};
use slog::Drain;
use std::collections::VecDeque;

pub struct RaftNode {
    inner: RawNode<MemStorage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaftCfg {
    id: MemberID,
    election_timeout: u64,
    heartbeat_interval: u64,
    replication_max_bytes: usize,
    //These are the other raft config vals from raft-rs...
    /* applied: u64,
    max_size_per_msg: 0,
    max_inflight_msgs: 256,
    check_quorum: bool,
    pre_vote: bool,
    min_election_tick: 0,
    max_election_tick: 0,
    read_only_option: ReadOnlyOption::Safe,
    skip_bcast_commit: bool,
    batch_append: bool,
    priority: 0,
    max_uncommitted_size: NO_LIMIT,
    max_committed_size_per_ready: NO_LIMIT,*/
}

impl RaftCfg {
    pub fn as_cfg(&self) -> Config {
        Config {
            id: self.id,
            election_tick: self.election_timeout as usize,
            heartbeat_tick: self.heartbeat_interval as usize,
            ..Default::default()
        }
    }

    pub fn id(&self) -> MemberID {
        self.id
    }
    pub fn election_timeout(&self) -> u64 {
        self.election_timeout
    }
    pub fn heartbeat_interval(&self) -> u64 {
        self.heartbeat_interval
    }
    pub fn replication_max_bytes(&self) -> usize {
        self.replication_max_bytes
    }
}


impl RaftNode {
    pub fn new(rcfg: RaftCfg, logger: &slog::Logger) -> Result<Self, ()> {
        let storage = MemStorage::new();
        /* let decorator = slog_term::TermDecorator::new().build();
         let drain = slog_term::FullFormat::new(decorator).build().fuse();
         let drain = slog_async::Async::new(drain)
             .chan_size(4096)
             .overflow_strategy(slog_async::OverflowStrategy::Block)
             .build()
             .fuse();
         let logger = slog::Logger::root(drain, o!());

        */
        /*
         let mut snapshot = Snapshot::default();

        match storage.wl().apply_snapshot(snapshot) {
            Ok(t)=>{

            }
            Err(e)=>{
                return Err(())
            }
        }*/

        let cfg = rcfg.as_cfg();
        let node = match RawNode::new(&cfg, storage, &logger) {
            Ok(node) => node,
            Err(e) => return Err(()),
        };
        Ok(Self { inner: node })
    }


}

