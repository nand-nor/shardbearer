use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{io, thread};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{
    ChannelBuilder, Environment, ResourceQuota, RpcContext, Server, ServerBuilder,
    ServerStreamingSink, UnarySink, WriteFlags,
};

use tracing::{error, info, span, warn, Level};
//use tracing_subscriber;

use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, ConfigSummary, Controller, HeraldInfo, Radiant as RadiantID, Role,
    Roles, ShardMoveRequest, ShardMoveRequestResponse,
};
use shardbearer_proto::radiant::radiant::*;
use shardbearer_proto::radiant::radiant_grpc::RadiantNode as RadiantNodeRPC;
//use shardbearer_proto::herald::herald_grpc::Herald;

use indexmap::IndexMap;
use shardbearer_core::membership::{GroupID, MemberID};
use shardbearer_core::membership::{Membership, RadiantMembership, Role as RadiantRole};
use shardbearer_core::order::{Order, OrderShardAction, RadiantOrderState};
use shardbearer_core::radiant::{Radiant, RadiantNode};
use shardbearer_core::shard::{Shard, ShardEntry, ShardKey, ShardMap};
use shardbearer_core::system::{RadiantSystem, SysState, System};

use crate::config::ShardbearerConfig;
use crate::herald::HeraldService;
use crate::order::RadiantOrder;
use shardbearer_proto::herald::herald_grpc::Herald;

use chrono::*;
use shardbearer_proto::radiant::radiant_grpc::RadiantNodeClient;
use timer::*;

use crate::rctrl::StateMessage;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
pub struct RadiantKey {
    pub gid: GroupID,
    pub mid: MemberID,
}

#[derive(Clone)]
pub struct RadiantService<K, V> {
    pub radiant: Arc<Mutex<RadiantNode<RadiantOrder, RadiantMembership, RadiantSystem>>>,
    pub neighbor: RadiantID,
    setup: bool,
    pub ctrl_chan_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
    pub herald: HeraldInfo,
    pub shard_map: Arc<Mutex<dyn ShardMap<K, V>>>,
}

pub async fn run<K, V>(svc: RadiantService<K, V>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

impl<K, V> RadiantService<K, V> {
    pub fn new(
        radiant: Arc<Mutex<RadiantNode<RadiantOrder, RadiantMembership, RadiantSystem>>>,
        shard_store: Arc<Mutex<dyn ShardMap<K, V>>>,
        cfg: ShardbearerConfig,
        bootstrap: tokio::sync::mpsc::UnboundedSender<StateMessage>,
    ) -> Self {
        let mut neigh = RadiantID::default();

        neigh.set_ip(cfg.neighbor_ip());
        neigh.set_port(cfg.neighbor_port() as _);

        let mut tmp = Self {
            radiant,
            neighbor: neigh,
            setup: false,
            ctrl_chan_tx: bootstrap,
            shard_map: shard_store,
            herald: HeraldInfo::new(),
        };
        tmp
    }

    pub fn share_map(&self) -> Arc<Mutex<dyn ShardMap<K, V>>> {
        Arc::clone(&self.shard_map)
    }

    pub fn set_neighbor(&mut self, neighbor: RadiantID) {
        self.neighbor = neighbor;
    }
    pub fn set_herald(&mut self, herald: HeraldInfo) {
        self.herald = herald;
    }

    /*
    pub fn setup_bootstrap_client(){

    }
    pub fn setup_herald_client(){

    }
    pub fn setup_radiant_server(){

    }
    pub fn setup_herald_server(){

    }
    pub fn setup_controller_server(){

    }
    */
}
use std::convert::TryInto;

impl<K, V> RadiantNodeRPC for RadiantService<K, V> {
    fn beacon_handshake(
        &mut self,
        ctx: RpcContext<'_>,
        beacon: Beacon,
        sink: UnarySink<BeaconResponse>,
    ) {
        tracing::trace!("Received beacon handshake RPC");

        let mut resp = BeaconResponse::default();
        let mut neighbor = RadiantID::new();
        let mut herald = self.herald.clone(); //HeraldInfo::new();
        let (gid, mid, state) = match self.radiant.lock() {
            Ok(g) => (g.group_id(), g.member_id(), g.order_state()),
            Err(_) => (0, 0, RadiantOrderState::INACTIVE),
        };

        neighbor.set_ip(self.neighbor.get_ip().to_string());
        neighbor.set_port(self.neighbor.get_port());
        resp.set_cluster_state(state as _);

        /*
             pub gid: u64,
        pub mid: u64,
        pub hid: u64,
        pub ip: ::std::string::String,
        pub port: u32,
            */

        resp.set_mid(mid);
        resp.set_gid(gid);
        resp.set_neighbor(neighbor);
        resp.set_hid(herald);

        tracing::trace!("Setting these vals for BeaconResponse: {:?}", resp);

        //this is not at all graceful :(
        //Check if this is a bootstrap beacon. If it is, we need to make sure the
        //guard for the timer gets dropped, and we need to transition to the next state
        //for the cluster and the order (TODO need better way to do this)
        match self.setup {
            true => {
                match self
                    .ctrl_chan_tx
                    .send(StateMessage::INITSTATE(resp.clone()))
                {
                    Ok(_) => {
                        tracing::trace!("End bootstrap");
                        match self.radiant.lock() {
                            Ok(mut g) => {
                                //TODO make this another send on the control channel, the controller
                                //should be the only thing updating state?
                                g.update_cluster_state(RadiantOrderState::VOTING);
                            }
                            Err(_) => {}
                        };

                        self.setup = false;
                    }
                    Err(_) => {
                        tracing::error!("Error sending on oneshot channel");
                    }
                }
            }
            false => {}
        }

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn join_system(&mut self, ctx: RpcContext<'_>, id: RadiantID, sink: UnarySink<ConfigSummary>) {
        let resp = ConfigSummary::default();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn leave_system(&mut self, ctx: RpcContext<'_>, id: RadiantID, sink: UnarySink<ConfigSummary>) {
        let resp = ConfigSummary::default();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn radiant_vote(&mut self, ctx: RpcContext<'_>, radiant: RadiantID, sink: UnarySink<Role>) {
        let resp = Role::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn current_roles(&mut self, ctx: RpcContext<'_>, ctrl: Controller, sink: UnarySink<Roles>) {
        let resp = Roles::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn move_shard_request(
        &mut self,
        ctx: RpcContext<'_>,
        request: ShardMoveRequest,
        sink: UnarySink<ShardMoveRequestResponse>,
    ) {
        let resp = ShardMoveRequestResponse::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
}
