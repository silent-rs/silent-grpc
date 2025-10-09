mod client;
mod handler;
mod proto;
mod service;

use std::{env, error::Error, net::SocketAddr};

use proto::helloworld::greeter_server::GreeterServer;
use service::GreeterService;
use silent::prelude::{Level, Route, Server, info, logger};
use silent::{GrpcRegister, Request};
use tonic_reflection::server::Builder as ReflectionBuilder;

const FILE_DESCRIPTOR_SET: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/helloworld_descriptor.bin"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if env::args().any(|arg| arg == "--client") {
        client::run_demo().await?;
        return Ok(());
    }

    logger::fmt().with_max_level(Level::INFO).init();
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;
    let reflection = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .map_err(|err| -> Box<dyn Error> { Box::new(err) })?;
    let route = Route::new("")
        .get(|_req: Request| async { Ok("silent grpc server is running") })
        .append(GreeterServer::new(GreeterService::new()).service())
        .append(reflection.service());
    info!("gRPC 服务启动，监听地址 {}", addr);
    Server::new().bind(addr).serve(route).await;
    Ok(())
}
