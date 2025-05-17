include!(concat!(env!("OUT_DIR"), "/grpc_package.rs"));

use crate::grpc_stress_simulator_service_server::GrpcStressSimulatorServiceServer;
use crate::services::StressSimulatorService;
use std::env;
use env_logger::Builder;
use log::{info, LevelFilter};
use tonic::transport::Server;

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let host = env::var("TONIC_GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("TONIC_GRPC_PORT").unwrap_or_else(|_| "50051".to_string());

    let addr = format!("{}:{}", host, port).parse()?;

    info!("Grpc service listening on {}", addr);

    let grpc_service = StressSimulatorService::default();

    Server::builder()
        .add_service(GrpcStressSimulatorServiceServer::new(grpc_service))
        .serve(addr)
        .await?;

    Ok(())
}
