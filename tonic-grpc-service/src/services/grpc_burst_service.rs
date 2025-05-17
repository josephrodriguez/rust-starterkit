use crate::{BurstRequest, BurstResponse, MemoryRequest, MemoryResponse};
use std::time::Duration;
use tonic::{Request, Response, Status};
use crate::grpc_stress_simulator_service_server::GrpcStressSimulatorService;

pub mod grpc_stress {
    tonic::include_proto!("grpc_package");
}

#[derive(Default)]
pub struct StressSimulatorService;

#[tonic::async_trait]
impl GrpcStressSimulatorService for StressSimulatorService {
    async fn simulate_burst(
        &self,
        request: Request<BurstRequest>,
    ) -> Result<Response<BurstResponse>, Status> {
        let request_inner = request.into_inner();
        let burst_time = Duration::from_millis(request_inner.duration_ms as u64);
        let upper_bound = request_inner.upper_bound as u64;

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            while start.elapsed() < burst_time {
                let mut acc = 0u64;
                for i in 0..upper_bound {
                    acc = acc.wrapping_add(i.wrapping_mul(i));
                }
                tokio::task::yield_now().await;
            }
        });

        Ok(Response::new(BurstResponse {
            message: format!(
                "Simulating CPU burst for {}ms with upper bound {}",
                request_inner.duration_ms, upper_bound
            ),
        }))
    }

    async fn simulate_memory_leak(
        &self,
        request: Request<MemoryRequest>,
    ) -> Result<Response<MemoryResponse>, Status> {
        todo!()
    }
}
