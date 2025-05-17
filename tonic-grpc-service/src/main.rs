include!(concat!(env!("OUT_DIR"), "/grpc_package.rs"));

use crate::services::StressSimulatorService;
use tonic::transport::Server;
use crate::grpc_stress_simulator_service_server::GrpcStressSimulatorServiceServer;

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    // let reflection_service = ReflectionBuilder::configure()
    //     .register_encoded_file_descriptor_set(grpc_burst_service_server::FILE_DESCRIPTOR_SET)
    //     .build()?;

    println!("Grpc service listening on {}", addr);

    Server::builder()
        .add_service(GrpcStressSimulatorServiceServer::new(StressSimulatorService))
        .serve(addr)
        .await?;

    Ok(())
}
