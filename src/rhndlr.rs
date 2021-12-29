use grpcio::{ChannelBuilder, Environment};

use shardbearer_proto::controller::controller_grpc::HeraldControllerRpcClient;
use shardbearer_proto::herald::herald_grpc::HeraldRpcClient;
use shardbearer_proto::radiant::radiant_grpc::RadiantRpcClient;
use tracing;

use crate::herald::{CtrlHeraldMsg, HeraldMsg};
use crate::radiant::RadiantMsg;
use crate::rctrl::StateMessage;

use std::sync::{Arc,Mutex};
use crate::radiant::RadiantNode;
use raft::eraftpb::{Message, MessageType};

pub enum ClientCommand {
    PEER(RadiantMsg),
    HERALD(HeraldMsg),
    CTRL(CtrlHeraldMsg),
}
//#[tracing::instrument]
pub struct RadiantRpcClientHandler {
    cmd_rx: tokio::sync::mpsc::UnboundedReceiver<ClientCommand>,
    state_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
    pub radiant: Arc<Mutex<RadiantNode>>,

    herald_cli: Option<HeraldRpcClient>,
    radiant_cli: Option<RadiantRpcClient>,
    ctrl_cli: Option<HeraldControllerRpcClient>,
}

impl RadiantRpcClientHandler {
    pub fn new(
        cmd_rx: tokio::sync::mpsc::UnboundedReceiver<ClientCommand>,
        state_tx: tokio::sync::mpsc::UnboundedSender<StateMessage>,
        radiant: Arc<Mutex<RadiantNode>>,
    ) -> Self {
        tracing::trace!("Constructor for RadiantRpcClientHandler called");
        Self {
            cmd_rx,
            state_tx,
            radiant,
            herald_cli: None,
            radiant_cli: None,
            ctrl_cli: None,
        }
    }
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::trace!("Dropping into RadiantRpcClientHandler event loop");

        loop {

            match self.cmd_rx.recv().await {
                Some(v) => {
                    match v {
                        ClientCommand::PEER(/*RadiantMsg*/ msg) => {
                            tracing::trace!("RadiantRpcClientHandler: received peer message!");
                            match msg.msg.get_msg_type(){
                                MessageType::MsgRequestVote=>{
                                    tracing::trace!("RadiantRpcClientHandler: request vote message!");

                                }
                                _=>{
                                    tracing::trace!("RadiantRpcClientHandler: other message type!");

                                }
                            }
                        }
                        ClientCommand::HERALD(/*HeraldMsg*/ msg) => {
                            tracing::trace!("RadiantRpcClientHandler: received herald message!");

                        }
                        ClientCommand::CTRL(/*CtrlHeraldMsg*/ msg) => {
                            tracing::trace!("RadiantRpcClientHandler: received controller message!");

                        }
                    }
                }
                None => {}
            }
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
        let mut client = RadiantRpcClient::new(channel);
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
        let mut client = HeraldRpcClient::new(channel);
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
        let mut client = HeraldControllerRpcClient::new(channel);
        self.ctrl_cli = Some(client);
    }

    pub fn drop_all(&mut self) {
        self.drop_radiant_client();
        self.drop_ctrl_client();
        self.drop_herald_client();
    }

    pub fn drop_radiant_client(&mut self) {
        if let Some(cli) = &self.radiant_cli {
            drop(cli);
        }
        self.radiant_cli = None;
    }

    pub fn drop_herald_client(&mut self) {
        if let Some(cli) = &self.herald_cli {
            drop(cli);
        }
        self.herald_cli = None;
    }

    pub fn drop_ctrl_client(&mut self) {
        if let Some(cli) = &self.ctrl_cli {
            drop(cli);
        }
        self.ctrl_cli = None;
    }

    pub fn reset(
        &mut self,
        rip: String,
        rport: u16,
        hip: String,
        hport: u16,
        cip: String,
        cport: u16,
    ) {
        self.reset_radiant(rip, rport);
        self.reset_herald(hip, hport);
        self.reset_controller(cip, cport);
    }

    pub fn reset_radiant(&mut self, ip: String, port: u16) {
        self.drop_radiant_client();
        self.setup_radiant_client(ip, port);
    }
    pub fn reset_herald(&mut self, ip: String, port: u16) {
        self.drop_herald_client();
        self.setup_herald_client(ip, port);
    }
    pub fn reset_controller(&mut self, ip: String, port: u16) {
        self.drop_ctrl_client();
        self.setup_ctrl_client(ip, port);
    }
}

/*
#[cfg(feature = "client_tests")]
pub mod tests {
    #[cfg(feature = "client_tests")]
    pub fn setup_tests() {
        tracing::info!("RadiantRpcClientHandler: running basic setup test");
        // assert_eq!(2 + 2, 4);
        let mut cli = super::RadiantRpcClientHandler::new();
        cli.setup_ctrl_client("127.0.0.1".to_string(), 50501);
        cli.setup_radiant_client("127.0.0.1".to_string(), 50502);
        cli.setup_herald_client("127.0.0.1".to_string(), 50503);
        drop(cli);
    }

    #[cfg(feature = "client_tests")]
    pub fn drop_tests() {
        // assert_eq!(2 + 2, 4);
        tracing::info!("RadiantRpcClientHandler: running setup and drop test");
        let mut cli = super::RadiantRpcClientHandler::new();
        cli.setup_ctrl_client("127.0.0.1".to_string(), 50501);
        cli.setup_radiant_client("127.0.0.1".to_string(), 50502);
        cli.setup_herald_client("127.0.0.1".to_string(), 50503);

        cli.drop_ctrl_client();
        cli.drop_radiant_client();
        cli.drop_herald_client();
        drop(cli);
    }

    pub fn reset_tests() {
        // assert_eq!(2 + 2, 4);
        tracing::info!("RadiantRpcClientHandler: running setup and drop test");
        let mut cli = super::RadiantRpcClientHandler::new();
        cli.setup_ctrl_client("127.0.0.1".to_string(), 50501);
        cli.setup_radiant_client("127.0.0.1".to_string(), 50502);
        cli.setup_herald_client("127.0.0.1".to_string(), 50503);

        cli.drop_ctrl_client();
        cli.drop_radiant_client();
        cli.drop_herald_client();
        cli.reset(
            "127.0.0.1".to_string(),
            50511,
            "127.0.0.1".to_string(),
            50512,
            "127.0.0.1".to_string(),
            50513,
        );
        drop(cli);
        let mut cli2 = super::RadiantRpcClientHandler::new();
        cli2.setup_ctrl_client("127.0.0.1".to_string(), 50501);
        cli2.setup_radiant_client("127.0.0.1".to_string(), 50502);
        cli2.setup_herald_client("127.0.0.1".to_string(), 50503);

        cli2.reset(
            "127.0.0.1".to_string(),
            50511,
            "127.0.0.1".to_string(),
            50512,
            "127.0.0.1".to_string(),
            50513,
        );
        cli2.drop_ctrl_client();
        cli2.drop_radiant_client();
        cli2.drop_herald_client();
        drop(cli2);
    }
}
*/