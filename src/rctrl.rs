use shardbearer_core::herald::HeraldState;
use shardbearer_core::membership::{Membership, RadiantMembership, Role as RadiantRole};
use shardbearer_core::order::{Order, OrderShardAction, RadiantOrderState};
use shardbearer_core::radiant::{Radiant, RadiantNode};
use std::sync::{Arc, Mutex};
//use shardbearer_core::shard::{Shard, ShardKey, ShardMap, ShardEntry};
use crate::config::ShardbearerConfig;
use crate::handler::RadiantRPCClientHandler;
use crate::order::RadiantOrder;
use futures::channel::oneshot;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use shardbearer_core::system::{RadiantSystem, SysState, System};
use shardbearer_proto::common::common::BeaconResponse;
use std::io::Read;
use timer::{Guard, Timer};

pub struct RadiantController<'a> {
    ctrl_chan_rx: tokio::sync::mpsc::UnboundedReceiver<StateMessage>,
    ctrl_chan_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
    _lifetime: std::marker::PhantomData<&'a ()>,
    pub radiant: Arc<Mutex<RadiantNode<RadiantOrder, RadiantMembership, RadiantSystem>>>,
    pub rpc_clients: RadiantRPCClientHandler,
}

pub enum StateMessage {
    INITSTATE(BeaconResponse),
    SYSSTATE(SysState),
    ORDERSTATE(RadiantOrderState),
    RADIANTSTATE(Membership),
    ORDERHERALDSTATE(HeraldState),
    CONTROLLERHERALDSTATE(HeraldState),
    CLUSTERSTATE(RadiantOrderState), //TODO have dedicated cluster state
}

impl RadiantController<'static> {
    pub fn new(
        ctrl_chan_rx: tokio::sync::mpsc::UnboundedReceiver<StateMessage>,
        ctrl_chan_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
        radiant: Arc<Mutex<RadiantNode<RadiantOrder, RadiantMembership, RadiantSystem>>>,
    ) -> Self {
        Self {
            ctrl_chan_rx,
            ctrl_chan_tx,
            _lifetime: std::marker::PhantomData,
            radiant,
            rpc_clients: RadiantRPCClientHandler::new(),
        }
    }

    pub async fn run(&'static mut self, cfg: ShardbearerConfig) {
        let radiant_ip = cfg.my_ip();
        let radiant_port = cfg.my_port();

        let neighbor_ip = cfg.neighbor_ip();
        let neighbor_port = cfg.neighbor_port();
        let bootstrap_backoff = cfg.bootstrap_backoff();

        let mut backoff: u64 = StdRng::seed_from_u64(bootstrap_backoff).gen();
        let timer = Timer::new();

        backoff = backoff % 100;
        tracing::trace!(
            "Bootstrap backoff time: {:?} ip {:?} port {:?}",
            backoff,
            neighbor_ip,
            neighbor_port
        );

        let ip = radiant_ip.clone();
        let port = radiant_port.clone();
        let closure_tx = self.ctrl_chan_tx.clone();

        let g = timer.schedule_repeating(chrono::Duration::milliseconds(backoff as _), move || {
            match crate::client::handshake(
                neighbor_ip.clone(),
                neighbor_port.clone(),
                closure_tx.clone(),
            ) {
                Ok(_) => {}
                Err(_) => {}
            };
        });

        let (tx_drop, rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            match rx.await {
                Ok(_) => {
                    drop(g);
                    tracing::trace!("Dropped bootstrap timer guard");

                }
                Err(_) => tracing::trace!("the sender dropped"),
            };
        });

        let (tx_drop_trigger, mut trigger_rx) = tokio::sync::mpsc::channel(10);

        tokio::spawn(async move {
            match trigger_rx.recv().await {
                Some(_) => {
                    tracing::trace!("Received a command to drop!");
                    match tx_drop.send(()) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                None => {}
            };
        });

        //drop into state monitoring loop
        loop {
            match self.ctrl_chan_rx.recv().await {
                Some(v) => {
                    match v {
                        StateMessage::INITSTATE(resp) => {
                            tracing::trace!("Received a BeaconResp: {:?}", resp);
                            if (resp.get_neighbor().get_ip() == ip
                                && resp.get_neighbor().get_port() == port as u32)
                                || (resp.get_cluster_state() as i32
                                    == RadiantOrderState::VOTING as i32
                                    || resp.get_cluster_state() as i32
                                        == RadiantOrderState::INACTIVE as i32)
                            {
                                if let Err(_) = tx_drop_trigger.send(()).await {
                                    tracing::error!("Error triggering timer drop");
                                }
                            }
                        }
                        StateMessage::SYSSTATE(s) => {
                            match s {
                                SysState::INIT => {}
                                _ => {
                                    //unimplemented
                                }
                            };
                        }
                        StateMessage::ORDERSTATE(s) => {
                            match s {
                                RadiantOrderState::INACTIVE => {}
                                _ => {
                                    //unimplemented
                                }
                            };
                        }
                        StateMessage::RADIANTSTATE(m) => {
                            match m {
                                Membership::UNASSOCIATED => {}
                                Membership::RADIANT => {}
                                _ => {
                                    //unimplemented -- here we would apply a vote change update?
                                }
                            };
                        }
                        StateMessage::ORDERHERALDSTATE(h) => {
                            match h {
                                HeraldState::VOTER => {}
                                HeraldState::CONTROLLER => {}
                            };
                        }
                        StateMessage::CONTROLLERHERALDSTATE(h) => {
                            match h {
                                HeraldState::VOTER => {}
                                HeraldState::CONTROLLER => {}
                            };
                        }
                        StateMessage::CLUSTERSTATE(s) => {
                            match s {
                                RadiantOrderState::INACTIVE => {}
                                _ => {
                                    //unimplemented
                                }
                            };
                        }
                    };
                    //   break;
                }
                None => tracing::error!("the sender dropped! oh gawd error error error"),
            };
        }
    }
}
