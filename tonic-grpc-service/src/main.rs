mod grpc_service;
mod grpc_server;


pub mod grpc_pressure {
    tonic::include_proto!("grpc_pressure");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    grpc_server::run().await
}
