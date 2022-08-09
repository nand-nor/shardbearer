use std::io;
use std::io::Read;
use std::sync::{Arc, Mutex};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use protobuf::Message;
use tracing::{debug, error, info, trace, warn};
//use tracing_subscriber;

use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, Bondsmith, ConfigId, ConfigSummary, HeraldInfo, JoinGroup, LeaveGroup,
    Order, OrderId, Radiant as RadiantIdMsg, Role, Roles, ShardMoveRequest,
    ShardMoveRequestResponse,
};
//use shardbearer_proto::radiant::radiant::*;
use shardbearer_core::radiant::RadiantStateMachine;

use crate::msg::*;

use crate::config::ShardbearerConfig;
//use crate::radiant::RadiantNode;
use crate::rctrl::RadiantCtrl;
use crate::rpc_cli_handler::RadiantRpcClientHandler;
use shardbearer_core::order::OrderState;

use shardbearer_core::*;

use shardbearer_core::radiant::{Radiant, RadiantNode}; // RadiantStateMachine};
use shardbearer_core::shard::{
    ShardHashMap, /*Shard, ShardEntry, ShardKey,*/ ShardMap, ShardbearerMessage,
};

use shardbearer_proto::bondsmith::bondsmith_grpc::BondsmithRpc;
use shardbearer_proto::herald::herald_grpc::HeraldRpc;
use shardbearer_proto::radiant::radiant_grpc::{create_radiant_rpc, RadiantRpc};
use std::convert::TryInto;
//use crate::server::RadiantServer;

use shardbearer_core::consensus::{ShardbearerConsensus, ShardbearerReplication};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Clone)]
pub struct RadiantService<
    C: ShardbearerConsensus,
    R: ShardbearerReplication,
    K,
    V,
    M: ShardbearerMessage,
> {
    pub radiant: Arc<Mutex<RadiantNode<K, C, R, M>>>,
    pub neighbor: RadiantIdMsg,
    setup: bool,
    pub ctrl_chan_tx: UnboundedSender<StateMessage>,
    pub herald: HeraldInfo,
    pub shard_map: Arc<Mutex<dyn ShardMap<K, V>>>,
}

impl<C: ShardbearerConsensus, R: ShardbearerReplication, K, V, M: ShardbearerMessage>
    RadiantService<C, R, K, V, M>
{
    pub fn new(
        radiant: Arc<Mutex<RadiantNode<K, C, R, M>>>,
        shard_store: Arc<Mutex<dyn ShardMap<K, V>>>,
        cfg: ShardbearerConfig,
        bootstrap: UnboundedSender<StateMessage>,
    ) -> Self {
        let mut neigh = RadiantIdMsg::default();

        neigh.set_ip(cfg.neighbor_ip());
        neigh.set_port(cfg.neighbor_port() as _);
        Self {
            radiant,
            neighbor: neigh,
            setup: false,
            ctrl_chan_tx: bootstrap,
            shard_map: shard_store,
            herald: HeraldInfo::new(),
        }
    }

    pub fn share_map(&self) -> Arc<Mutex<dyn ShardMap<K, V>>> {
        Arc::clone(&self.shard_map)
    }

    pub fn set_neighbor(&mut self, neighbor: RadiantIdMsg) {
        self.neighbor = neighbor;
    }
    pub fn set_herald(&mut self, herald: HeraldInfo) {
        self.herald = herald;
    }
}

impl<C: ShardbearerConsensus, R: ShardbearerReplication, K, V, M: ShardbearerMessage> RadiantRpc
    for RadiantService<C, R, K, V, M>
{
    fn beacon_handshake(
        &mut self,
        ctx: RpcContext<'_>,
        beacon: Beacon,
        sink: UnarySink<BeaconResponse>,
    ) {
        tracing::trace!("RadiantService: Received beacon handshake RPC");

        let mut resp = BeaconResponse::default();
        let mut neighbor = RadiantIdMsg::new();
        let mut herald = self.herald.clone(); //HeraldInfo::new();
        let (gid, mid, state) = match self.radiant.lock() {
            Ok(g) => (g.group_id(), g.member_id(), g.order_state()),
            Err(_) => (0, 0, OrderState::RESET), //ERROR),
        };

        neighbor.set_ip(self.neighbor.get_ip().to_string());
        neighbor.set_port(self.neighbor.get_port());
        //resp.set_cluster_state(state as u32 );

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
                                // g.update_cluster_state(OrderState::VOTING);
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

    fn join_system(
        &mut self,
        ctx: RpcContext<'_>,
        id: RadiantIdMsg,
        sink: UnarySink<ConfigSummary>,
    ) {
        let resp = ConfigSummary::default();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn leave_system(
        &mut self,
        ctx: RpcContext<'_>,
        id: RadiantIdMsg,
        sink: UnarySink<ConfigSummary>,
    ) {
        let resp = ConfigSummary::default();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn radiant_vote(&mut self, ctx: RpcContext<'_>, radiant: RadiantIdMsg, sink: UnarySink<Role>) {
        let resp = Role::new();

        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("RadiantService: failed to handle request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn current_roles(&mut self, ctx: RpcContext<'_>, ctrl: Bondsmith, sink: UnarySink<Roles>) {
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

        //TODO move order member list into service struct, so that `shardbearer-core` can
        // remain decoupled from `shardbearer-proto`
        /* match self.radiant.lock() {
            Ok(mut g) => {
                let radiants: Vec<RadiantIdMsg> = g.order_members.iter().map(|x| x.clone()).collect();
                resp.set_members(protobuf::RepeatedField::from_vec(radiants));
            }
            Err(_) => {}
        };*/

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
/*
impl<C: ShardbearerConsensus, R: ShardbearerReplication, K, V> HeraldRpc for RadiantService<C,R,K, V> {
    fn herald_vote(&mut self, ctx: RpcContext<'_>, radiant: RadiantIdMsg, sink: UnarySink<Roles>) {
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
*/
