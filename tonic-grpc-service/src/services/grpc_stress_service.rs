use crate::grpc_stress_simulator_service_server::GrpcStressSimulatorService;
use crate::{BurstRequest, BurstResponse, MemoryRequest, MemoryResponse};
use log::info;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tonic::{Request, Response, Status};

pub mod grpc_stress {
    tonic::include_proto!("grpc_package");
}

#[derive(Debug, Default)]
pub struct StressSimulatorService {
    memory_hog: Arc<Mutex<Vec<Vec<u8>>>>,
}

#[tonic::async_trait]
impl GrpcStressSimulatorService for StressSimulatorService {
    async fn simulate_burst(
        &self,
        request: Request<BurstRequest>,
    ) -> Result<Response<BurstResponse>, Status> {
        let request_inner = request.into_inner();
        let burst_time = Duration::from_millis(request_inner.duration_ms as u64);
        let upper_bound = request_inner.upper_bound;

        info!(
            "Starting CPU burst: duration = {} ms, upper_bound = {}",
            request_inner.duration_ms, upper_bound
        );

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            while start.elapsed() < burst_time {
                let mut acc = 0u64;
                for i in 0..upper_bound {
                    acc = acc.wrapping_add(i.wrapping_mul(i));
                }
                tokio::task::yield_now().await;
            }

            info!(
                "Finished CPU burst after {:?}, upper_bound = {}",
                burst_time, upper_bound
            );
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
        let inner_request = request.into_inner();
        let duration = Duration::from_millis(inner_request.duration_ms as u64);
        let memory_bytes = inner_request.memory_bytes as usize;

        info!(
            "Allocating {} bytes of memory for {} ms",
            memory_bytes, inner_request.duration_ms
        );

        let mut hog = vec![0u8; memory_bytes];
        for i in 0..memory_bytes {
            hog[i] = (i % 256) as u8;
        }

        let mut store = self.memory_hog.lock().await;
        store.push(hog);

        let handle = {
            let memory_hog = Arc::clone(&self.memory_hog);
            tokio::spawn(async move {
                sleep(duration).await;
                let mut mem = memory_hog.lock().await;
                mem.clear(); // release memory

                info!(
                    "Released {} bytes of memory after {} ms",
                    memory_bytes, inner_request.duration_ms
                );
            })
        };

        Ok(Response::new(MemoryResponse {
            message: format!(
                "Allocated {} bytes of memory for {} ms",
                memory_bytes, inner_request.duration_ms
            ),
        }))
    }
}
