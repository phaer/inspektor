// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_INSPEKTOR_AUTH: ::grpcio::Method<super::api::AuthRequest, super::api::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/api.Inspektor/Auth",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INSPEKTOR_POLICY: ::grpcio::Method<super::api::Empty, super::api::InspektorPolicy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/api.Inspektor/Policy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct InspektorClient {
    client: ::grpcio::Client,
}

impl InspektorClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        InspektorClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn auth_opt(&self, req: &super::api::AuthRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::api::Empty> {
        self.client.unary_call(&METHOD_INSPEKTOR_AUTH, req, opt)
    }

    pub fn auth(&self, req: &super::api::AuthRequest) -> ::grpcio::Result<super::api::Empty> {
        self.auth_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_async_opt(&self, req: &super::api::AuthRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Empty>> {
        self.client.unary_call_async(&METHOD_INSPEKTOR_AUTH, req, opt)
    }

    pub fn auth_async(&self, req: &super::api::AuthRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::api::Empty>> {
        self.auth_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn policy_opt(&self, req: &super::api::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::api::InspektorPolicy>> {
        self.client.server_streaming(&METHOD_INSPEKTOR_POLICY, req, opt)
    }

    pub fn policy(&self, req: &super::api::Empty) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::api::InspektorPolicy>> {
        self.policy_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Inspektor {
    fn auth(&mut self, ctx: ::grpcio::RpcContext, req: super::api::AuthRequest, sink: ::grpcio::UnarySink<super::api::Empty>);
    fn policy(&mut self, ctx: ::grpcio::RpcContext, req: super::api::Empty, sink: ::grpcio::ServerStreamingSink<super::api::InspektorPolicy>);
}

pub fn create_inspektor<S: Inspektor + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INSPEKTOR_AUTH, move |ctx, req, resp| {
        instance.auth(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_INSPEKTOR_POLICY, move |ctx, req, resp| {
        instance.policy(ctx, req, resp)
    });
    builder.build()
}