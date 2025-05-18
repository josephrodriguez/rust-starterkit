use std::env;
use std::error::Error;
use std::net::SocketAddr;
use env_logger::Env;
use log::info;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use crate::grpc_pressure::grpc_stress_simulator_service_server::GrpcStressSimulatorServiceServer;
use crate::grpc_service::{PressureService};

pub mod grpc_service {
    tonic::include_proto!("grpc_pressure");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("descriptor");
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    init_logger();

    let addr = load_grpc_addr()?;
    info!("Starting gRPC service on {}", addr);

    start_grpc_server(addr).await
}

fn init_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
}

fn load_grpc_addr() -> Result<SocketAddr, Box<dyn Error>> {
    let host = env::var("GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("{}:{}", host, port).parse()?;
    Ok(addr)
}

async fn start_grpc_server(
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let grpc_service = PressureService::default();

    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(grpc_service::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    Server::builder()
        .add_service(GrpcStressSimulatorServiceServer::new(grpc_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}