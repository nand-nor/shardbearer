use std::io;
use std::io::Read;
use std::sync::{Arc, Mutex};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{
    Environment,
    RpcContext,
    ServerBuilder,
    UnarySink,
};
use protobuf;
use tracing::{debug, error, info, trace, warn};
//use tracing_subscriber;

use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, ConfigId, ConfigSummary, Controller, HeraldInfo, JoinGroup, LeaveGroup,
    Order, OrderId, Radiant as RadiantID, Role, Roles, ShardMoveRequest, ShardMoveRequestResponse,
};
//use shardbearer_proto::radiant::radiant::*;

use crate::config::ShardbearerConfig;
//use crate::radiant::RadiantNode;
use crate::rctrl::{RadiantController, StateMessage};
use crate::rhndlr::RadiantRpcClientHandler;
use shardbearer_core::order::RadiantOrderState;
use shardbearer_core::radiant::{Radiant, RadiantNode};// RadiantStateMachine};
use shardbearer_core::shard::{ShardHashMap, /*Shard, ShardEntry, ShardKey,*/ ShardMap};
//use shardbearer_core::system::RadiantSystem;

use shardbearer_proto::controller::controller_grpc::BondsmithRpc;
use shardbearer_proto::herald::herald_grpc::HeraldRpc;
use shardbearer_proto::radiant::radiant_grpc::{create_radiant_rpc, RadiantRpc};
use std::convert::TryInto;
//use crate::server::RadiantServer;


use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

use shardbearer_core::consensus::ShardbearerConsensus;
use shardbearer_core::consensus::ShardbearerReplication;

#[derive(Clone)]
pub struct RadiantService<C: ShardbearerConsensus, R: ShardbearerReplication, K, V> {
    pub radiant: Arc<Mutex<RadiantNode<K,C,R>>>,
    pub neighbor: RadiantID,
    setup: bool,
    pub ctrl_chan_tx: UnboundedSender<StateMessage>,
    pub herald: HeraldInfo,
    pub shard_map: Arc<Mutex<dyn ShardMap<K, V>>>,
}

impl<C: ShardbearerConsensus, R: ShardbearerReplication, K, V> RadiantService<C,R,K, V> {
    pub fn new(
        radiant: Arc<Mutex<RadiantNode<K,C,R>>>,
        shard_store: Arc<Mutex<dyn ShardMap<K, V>>>,
        cfg: ShardbearerConfig,
        bootstrap: UnboundedSender<StateMessage>,
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
}

impl<C: ShardbearerConsensus, R: ShardbearerReplication, K, V> RadiantRpc for RadiantService<C,R,K, V> {
    fn beacon_handshake(
        &mut self,
        ctx: RpcContext<'_>,
        beacon: Beacon,
        sink: UnarySink<BeaconResponse>,
    ) {
        tracing::trace!("RadiantService: Received beacon handshake RPC");

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

        resp.set_mid(mid);
        resp.set_gid(gid);
        resp.set_neighbor(neighbor);
        resp.set_hid(herald);

        tracing::trace!(
            "RadiantService: Setting these vals for BeaconResponse: {:?}",
            resp
        );

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
                        tracing::trace!("RadiantService: Ending bootstrap");
                        match self.radiant.lock() {
                            Ok(mut g) => {
                                //TODO make this another send on the control channel, the bondsmith
                                //should be the only thing updating state?
                                g.update_cluster_state(RadiantOrderState::VOTING);
                            }
                            Err(_) => {}
                        };

                        self.setup = false;
                    }
                    Err(_) => {
                        tracing::error!("RadiantService: Error sending on oneshot channel");
                    }
                }
            }
            false => {}
        }

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn join_system(&mut self, ctx: RpcContext<'_>, id: RadiantID, sink: UnarySink<ConfigSummary>) {
        let resp = ConfigSummary::default();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn leave_system(&mut self, ctx: RpcContext<'_>, id: RadiantID, sink: UnarySink<ConfigSummary>) {
        let resp = ConfigSummary::default();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn radiant_vote(&mut self, ctx: RpcContext<'_>, radiant: RadiantID, sink: UnarySink<Role>) {
        let resp = Role::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn current_roles(&mut self, ctx: RpcContext<'_>, ctrl: Controller, sink: UnarySink<Roles>) {
        let resp = Roles::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn current_order_membership(
        &mut self,
        ctx: RpcContext<'_>,
        gid: OrderId,
        sink: UnarySink<Order>,
    ) {
        tracing::trace!("RadiantService: received request to send current order members");
        let mut resp = Order::new();
        let gid = gid.get_gid();
        match self.radiant.lock() {
            Ok(mut g) => {
                let radiants: Vec<RadiantID> = g.order_members.iter().map(|x| x.clone()).collect();
                resp.set_members(protobuf::RepeatedField::from_vec(radiants));
            }
            Err(_) => {}
        };

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
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
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

impl<C: ShardbearerConsensus, R: ShardbearerReplication, K, V> HeraldRpc for RadiantService<C,R,K, V> {
    fn herald_vote(&mut self, ctx: RpcContext<'_>, radiant: RadiantID, sink: UnarySink<Roles>) {
        let resp = Roles::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

impl<C: ShardbearerConsensus, R: ShardbearerReplication, K, V> BondsmithRpc for RadiantService<C,R,K, V> {
    fn join(&mut self, ctx: RpcContext, req: JoinGroup, sink: UnarySink<ConfigSummary>) {
        let resp = ConfigSummary::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
    fn leave(&mut self, ctx: RpcContext, req: LeaveGroup, sink: UnarySink<ConfigSummary>) {
        let resp = ConfigSummary::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
    fn get_current_config(
        &mut self,
        ctx: RpcContext,
        req: ConfigId,
        sink: UnarySink<ConfigSummary>,
    ) {
        let resp = ConfigSummary::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
}
