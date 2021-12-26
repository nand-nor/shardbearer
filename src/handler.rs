use grpcio::{Channel, ChannelBuilder, Environment};

use shardbearer_proto::common::common::{
    Beacon, BeaconResponse, ConfigSummary, Controller, Radiant as RadiantID, Role, Roles,
    ShardMoveRequest, ShardMoveRequestResponse,
};
use shardbearer_proto::controller::controller_grpc::HeraldControllerClient;
use shardbearer_proto::herald::herald_grpc::HeraldClient;
use shardbearer_proto::radiant::radiant_grpc::RadiantNodeClient;

use std::sync::Arc;

pub struct RadiantRPCClientHandler {
    //herald_chan: Channel,
    //radiant_chan: Channel,
    //ctrl_chan: Channel,
    herald_cli: Option<HeraldClient>,
    radiant_cli: Option<RadiantNodeClient>,
    ctrl_cli: Option<HeraldControllerClient>,
}

impl RadiantRPCClientHandler {
    pub fn new() -> Self {
        tracing::trace!("Contructor for RadiantClientHandler called");
        Self {
            herald_cli: None,
            radiant_cli: None,
            ctrl_cli: None,
        }
    }

    pub fn setup_radiant_client(&mut self, ip: String, port: u16) {
        let env = Arc::new(Environment::new(2));
        tracing::trace!(
            "Setting up new radiant client for addr {:?}",
            format!("{}:{}", ip.clone(), port.clone()).as_str()
        );
        let channel =
            ChannelBuilder::new(env).connect(format!("{}:{}", ip.clone(), port.clone()).as_str());
        let mut client = RadiantNodeClient::new(channel);
        self.radiant_cli = Some(client);
    }

    pub fn setup_herald_client(&mut self, ip: String, port: u16) {
        let env = Arc::new(Environment::new(2));
        tracing::trace!(
            "Setting up new radiant client for addr {:?}",
            format!("{}:{}", ip.clone(), port.clone()).as_str()
        );
        let channel =
            ChannelBuilder::new(env).connect(format!("{}:{}", ip.clone(), port.clone()).as_str());
        let mut client = HeraldClient::new(channel);
        self.herald_cli = Some(client);
    }

    pub fn setup_ctrl_client(&mut self, ip: String, port: u16) {
        let env = Arc::new(Environment::new(2));
        tracing::trace!(
            "Setting up new radiant client for addr {:?}",
            format!("{}:{}", ip.clone(), port.clone()).as_str()
        );
        let channel =
            ChannelBuilder::new(env).connect(format!("{}:{}", ip.clone(), port.clone()).as_str());
        let mut client = HeraldControllerClient::new(channel);
        self.ctrl_cli = Some(client);
    }

    pub fn drop_radiant_client(&mut self) {
        self.radiant_cli = None;
    }

    pub fn drop_herald_client(&mut self) {
        self.herald_cli = None;
    }

    pub fn drop_ctrl_client(&mut self) {
        self.ctrl_cli = None;
    }
}
