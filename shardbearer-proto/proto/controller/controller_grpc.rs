// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_HERALD_CONTROLLER_RPC_JOIN: ::grpcio::Method<super::common::JoinGroup, super::common::ConfigSummary> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/HeraldControllerRPC/Join",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_HERALD_CONTROLLER_RPC_LEAVE: ::grpcio::Method<super::common::LeaveGroup, super::common::ConfigSummary> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/HeraldControllerRPC/Leave",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_HERALD_CONTROLLER_RPC_GET_CURRENT_CONFIG: ::grpcio::Method<super::common::ConfigId, super::common::ConfigSummary> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/HeraldControllerRPC/GetCurrentConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct HeraldControllerRpcClient {
    client: ::grpcio::Client,
}

impl HeraldControllerRpcClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        HeraldControllerRpcClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn join_opt(&self, req: &super::common::JoinGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::ConfigSummary> {
        self.client.unary_call(&METHOD_HERALD_CONTROLLER_RPC_JOIN, req, opt)
    }

    pub fn join(&self, req: &super::common::JoinGroup) -> ::grpcio::Result<super::common::ConfigSummary> {
        self.join_opt(req, ::grpcio::CallOption::default())
    }

    pub fn join_async_opt(&self, req: &super::common::JoinGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ConfigSummary>> {
        self.client.unary_call_async(&METHOD_HERALD_CONTROLLER_RPC_JOIN, req, opt)
    }

    pub fn join_async(&self, req: &super::common::JoinGroup) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ConfigSummary>> {
        self.join_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn leave_opt(&self, req: &super::common::LeaveGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::ConfigSummary> {
        self.client.unary_call(&METHOD_HERALD_CONTROLLER_RPC_LEAVE, req, opt)
    }

    pub fn leave(&self, req: &super::common::LeaveGroup) -> ::grpcio::Result<super::common::ConfigSummary> {
        self.leave_opt(req, ::grpcio::CallOption::default())
    }

    pub fn leave_async_opt(&self, req: &super::common::LeaveGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ConfigSummary>> {
        self.client.unary_call_async(&METHOD_HERALD_CONTROLLER_RPC_LEAVE, req, opt)
    }

    pub fn leave_async(&self, req: &super::common::LeaveGroup) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ConfigSummary>> {
        self.leave_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_current_config_opt(&self, req: &super::common::ConfigId, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::ConfigSummary> {
        self.client.unary_call(&METHOD_HERALD_CONTROLLER_RPC_GET_CURRENT_CONFIG, req, opt)
    }

    pub fn get_current_config(&self, req: &super::common::ConfigId) -> ::grpcio::Result<super::common::ConfigSummary> {
        self.get_current_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_current_config_async_opt(&self, req: &super::common::ConfigId, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ConfigSummary>> {
        self.client.unary_call_async(&METHOD_HERALD_CONTROLLER_RPC_GET_CURRENT_CONFIG, req, opt)
    }

    pub fn get_current_config_async(&self, req: &super::common::ConfigId) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ConfigSummary>> {
        self.get_current_config_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait HeraldControllerRpc {
    fn join(&mut self, ctx: ::grpcio::RpcContext, req: super::common::JoinGroup, sink: ::grpcio::UnarySink<super::common::ConfigSummary>);
    fn leave(&mut self, ctx: ::grpcio::RpcContext, req: super::common::LeaveGroup, sink: ::grpcio::UnarySink<super::common::ConfigSummary>);
    fn get_current_config(&mut self, ctx: ::grpcio::RpcContext, req: super::common::ConfigId, sink: ::grpcio::UnarySink<super::common::ConfigSummary>);
}

pub fn create_herald_controller_rpc<S: HeraldControllerRpc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_HERALD_CONTROLLER_RPC_JOIN, move |ctx, req, resp| {
        instance.join(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_HERALD_CONTROLLER_RPC_LEAVE, move |ctx, req, resp| {
        instance.leave(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_HERALD_CONTROLLER_RPC_GET_CURRENT_CONFIG, move |ctx, req, resp| {
        instance.get_current_config(ctx, req, resp)
    });
    builder.build()
}
