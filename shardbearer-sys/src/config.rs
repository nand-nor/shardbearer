use serde::{Deserialize, Serialize};
use shardbearer_core::radiant::MemberID;
use shardbearer_core::consensus::{ConsensusCfg, ReplicationCfg};
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub const RUNTIME_DEF_NUM_THREADS: usize = 1;
pub const RUNTIME_DEF_THREADSTACK: usize = 65536;
pub const RUNTIME_DEF_NAME: &str = "radiant";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShardbearerConfig {
    my_ip: String,
    my_port: u16,
    neighbor_ip: String,
    neighbor_port: u16,
    bootstrap_backoff: u64,
    consensus_cfg: ConsensusCfg,
    replication_cfg: Option<dyn ReplicationCfg>,
    runtime: Option<RuntimeCfg>,
}

impl ShardbearerConfig {
    pub fn my_port(&self) -> u16 {
        self.my_port
    }
    pub fn my_ip(&self) -> String {
        self.my_ip.clone()
    }
    pub fn neighbor_port(&self) -> u16 {
        self.neighbor_port
    }
    pub fn neighbor_ip(&self) -> String {
        self.neighbor_ip.clone()
    }

    pub fn bootstrap_backoff(&self) -> u64 {
        self.bootstrap_backoff
    }

   // pub fn raft_cfg(&self) -> Config {
   //     self.consensus_cfg.as_cfg()
   // }

    pub fn runtime_cfg(&self) -> RuntimeCfg {
        if let Some(rt) = &self.runtime {
            return rt.clone();
        }
        RuntimeCfg::default_build()
    }

    pub fn id(&self) -> MemberID {
        self.consensus_cfg.id()
    }

    pub fn election_timeout(&self) -> u64 {
        self.consensus_cfg.election_timeout()
    }
    pub fn heartbeat_interval(&self) -> u64 {
        self.consensus_cfg.heartbeat_interval()
    }
    //pub fn replication_max_bytes(&self) -> usize {
   //     self.replication_cfg.replication_max_bytes()
   // }

    pub fn rep_cfg(&self)->Option<dyn ReplicationCfg>{
        self.replication_cfg
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeCfg {
    num_threads: usize,
    thread_stack_size: usize,
    runtime_name: String,
}

impl RuntimeCfg {
    pub fn default_build() -> Self {
        Self {
            num_threads: RUNTIME_DEF_NUM_THREADS,
            thread_stack_size: RUNTIME_DEF_THREADSTACK,
            runtime_name: RUNTIME_DEF_NAME.to_string(),
        }
    }
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
    pub fn thread_stack_size(&self) -> usize {
        self.thread_stack_size
    }
    pub fn runtime_name(&self) -> String {
        self.runtime_name.clone()
    }
}

pub fn parse_cfg(file: &str) -> Result<ShardbearerConfig, Box<dyn Error>> {
    let mut fp = match File::open(file) {
        Err(why) => {
            tracing::error!(
                "Could not open provided toml: {}: {}",
                file,
                why.to_string()
            );
            return Err(Box::new(why));
        }
        Ok(fp) => fp,
    };

    let mut ops = vec![];
    fp.read_to_end(&mut ops)?;

    let cfg = toml::from_slice(&ops)?;
    tracing::info!("Pulled ShardbearerConfig!: {:?}", cfg);
    Ok(cfg)
}
