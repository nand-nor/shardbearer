use crate::config::ShardbearerConfig;
use crate::radiant::{RadiantNode, RadiantMsg};
use crate::rhndlr::{ClientCommand, RadiantRpcClientHandler};
use shardbearer_core::order::RadiantOrderState;
use shardbearer_core::radiant::{HeraldRole, RadiantRole, RadiantState, RadiantStateMachine};
use std::sync::{Arc, Mutex};

use shardbearer_core::raft::RaftCfg;
use shardbearer_core::system::{RadiantSystem, SysState};
use shardbearer_proto::common::common::{BeaconResponse, Radiant as RadiantId};
use timer::{Guard, Timer};

use raft::Config;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use crate::herald::HeraldMsg;
use raft::eraftpb::{Message, MessageType};

pub struct RadiantController<'a> {
    ctrl_chan_rx: tokio::sync::mpsc::UnboundedReceiver<StateMessage>,
    ctrl_chan_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
    cmd_tx: tokio::sync::mpsc::UnboundedSender<ClientCommand>,
    _lifetime: std::marker::PhantomData<&'a ()>,
    pub radiant: Arc<Mutex<RadiantNode>>,
}

pub struct Vote {}

pub enum StateMessage {
    INITSTATE(BeaconResponse),
    VOTESTATE(Vote),
    SYSSTATE(SysState),
    ORDERSTATE(RadiantOrderState),
    RADIANTSTATE(RadiantState),
    RADIANTROLE(RadiantRole),
    ORDERHERALDSTATE(HeraldRole),
    CONTROLLERHERALDSTATE(HeraldRole),
    CLUSTERSTATE(RadiantOrderState), //TODO have dedicated cluster state
}

impl RadiantController<'static> {
    pub fn new(
        ctrl_chan_rx: tokio::sync::mpsc::UnboundedReceiver<StateMessage>,
        ctrl_chan_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
        cmd_tx: tokio::sync::mpsc::UnboundedSender<ClientCommand>,
        radiant: Arc<Mutex<RadiantNode>>,
    ) -> Self {
        Self {
            ctrl_chan_rx,
            ctrl_chan_tx,
            cmd_tx,
            _lifetime: std::marker::PhantomData,
            radiant,
        }
    }

    pub async fn bootstrap(
        guard: Guard,
        mut trigger_rx: tokio::sync::mpsc::Receiver<()>,
    ) -> Result<(), Box<dyn std::error::Error>> {

        let (tx_drop, mut rx) = tokio::sync::oneshot::channel();
        RadiantController::initial_association(
            guard,
            rx,
        )
        .await?;

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
        }).await;
        Ok(())
    }

    pub async fn initial_association(
        guard: Guard,
        mut rx: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::spawn(async move {
            match rx.await {
                Ok(_) => {
                    drop(guard);
                    tracing::trace!("Dropped bootstrap timer guard");
                }
                Err(_) => tracing::error!("the sender dropped"),
            };
        });

        Ok(())
    }

    pub async fn run(
        &'static mut self,
        cfg: ShardbearerConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let radiant_ip = cfg.my_ip();
        let radiant_port = cfg.my_port();
        let neighbor_ip = cfg.neighbor_ip();
        let neighbor_port = cfg.neighbor_port();
        let bootstrap_backoff = cfg.bootstrap_backoff();

        let rcfg = cfg.raft_cfg();

        let (tx_drop_trigger, mut trigger_rx) = tokio::sync::mpsc::channel(10);

        let ip = radiant_ip.clone();
        let port = radiant_port.clone();
        let tx_init = self.ctrl_chan_tx.clone();
        let association_tx = self.cmd_tx.clone();
        let mut backoff: u64 = StdRng::seed_from_u64(bootstrap_backoff).gen();
        let timer = Timer::new();

        backoff = backoff % 500;
        tracing::trace!(
            "Bootstrap backoff time: {:?} milliseconds, senging to neighbor at: ip {:?} port {:?}",
            backoff,
            neighbor_ip,
            neighbor_port
        );

        let closure_tx = tx_init.clone();

        let guard = timer.schedule_repeating(chrono::Duration::milliseconds(backoff as _), move || {
            match crate::client::handshake(
                neighbor_ip.clone(),
                neighbor_port.clone(),
                closure_tx.clone(),
            ) {
                Ok(_) => {}
                Err(_) => {}
            };
        });

        tokio::spawn(async move {
            if let Err(_) = RadiantController::bootstrap(
                guard,
                trigger_rx,
            )
                .await {
                tracing::error!( "Error running bootstrap setup steps");
            }
        });

        tracing::trace!(
            "Dropping into RadiantController state monitoring loop"
        );

        //drop into state monitoring loop
        loop {
            match self.ctrl_chan_rx.recv().await {
                Some(v) => {
                    match v {
                        StateMessage::INITSTATE(resp) => {
                            tracing::trace!("Received a BeaconResp: {:?}", resp);
                            if (resp.get_neighbor().get_ip() == ip
                                && resp.get_neighbor().get_port() == port as u32
                                && resp.get_cluster_state() as i32
                                    != RadiantOrderState::VOTING as i32
                                 &&  resp.get_cluster_state() as i32
                                    != RadiantOrderState::RESETLOCK as i32)
                                || resp.get_cluster_state() as i32
                                    == RadiantOrderState::ACTIVE as i32
                                || resp.get_cluster_state() as i32
                                    == RadiantOrderState::INACTIVE as i32
                            {
                                tracing::trace!("Triggering timer drop");

                                if let Err(_) = tx_drop_trigger.send(()).await {
                                    //tracing::error!("Error triggering timer drop");
                                }
                            }

                            if resp.get_cluster_state() as i32
                                == RadiantOrderState::VOTING as i32
                                || resp.get_cluster_state() as i32
                                == RadiantOrderState::RESETLOCK as i32 {
                                tracing::trace!(
                                    "System is locked, keep trying system state changes"
                                );
                            } else if resp.get_cluster_state() as i32
                                == RadiantOrderState::INACTIVE as i32
                            {
                                // let assoc = Association{
                               /*  match self.radiant.lock() {
                                    Ok(mut g) => (g.update_order_state(RadiantOrderState::VOTING)),
                                    Err(_) => {
                                        tracing::error!("System error getting mutex guard, failure to update state to voting");
                                    },
                                };
                                // };*/

                                let mut msg = Message::default();
                                msg.set_msg_type(MessageType::MsgRequestVote);
                                let new_msg = ClientCommand::PEER(RadiantMsg{
                                    rid: RadiantId::default(),
                                    msg,
                                });

                                if let Err(_) = association_tx.send(new_msg) {
                                //    tracing::error!("Error initiating next association step");
                                } else {
                                    tracing::trace!("Send voting message to rpc client handler");

                                }
                            } else if resp.get_cluster_state() as i32
                                == RadiantOrderState::ACTIVE as i32
                            {
                                //&& resp.get_join_success() {
                                tracing::trace!(
                                    "System is active, make request to join the system"
                                );
                                //TODO here we trigger the client handler to send the relevant
                                // client RPC request to the herald?

                                let herald_info = resp.get_hid().clone();

                                let new_msg = ClientCommand::HERALD(HeraldMsg{
                                    hid: herald_info,
                                    msg: Message::default(),
                                });
                                if let Err(_) = association_tx.send(new_msg) {
                                    //    tracing::error!("Error initiating next association step");
                                } else {
                                    tracing::trace!("Send join message to herald (will relay to controller if not already incontroller state)");

                                }
                            } else {
                                tracing::trace!("Unhandled case");
                            }
                        }
                        StateMessage::VOTESTATE(resp) => {}
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
                                RadiantState::UNASSOCIATED => {}
                                RadiantState::BOOTSTRAP => {}
                                RadiantState::VOTING => {}
                                RadiantState::ACTIVE => {}
                                RadiantState::LOCKED => {}
                                RadiantState::ERROR(state_error) => {}
                                _ => {
                                    //unimplemented
                                }
                            };
                        }
                        StateMessage::RADIANTROLE(m) => {
                            match m {
                                RadiantRole::UNASSOCIATED => {}
                                RadiantRole::MEMBER(order_role) => {}
                                _ => {
                                    //unimplemented
                                }
                            };
                        }
                        StateMessage::ORDERHERALDSTATE(h) => {
                            match h {
                                HeraldRole::VOTER => {}
                                HeraldRole::CONTROLLER => {}
                            };
                        }
                        StateMessage::CONTROLLERHERALDSTATE(h) => {
                            match h {
                                HeraldRole::VOTER => {}
                                HeraldRole::CONTROLLER => {}
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
                None => tracing::error!("sthe sender dropped! oh gawd error error error"),
            };
        }
        Ok(())
    }
}
