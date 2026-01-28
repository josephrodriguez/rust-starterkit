use crate::grpc_pressure::grpc_stress_simulator_service_server::GrpcStressSimulatorService;
use crate::grpc_pressure::{
    CpuLoadRequest, CpuLoadResponse, MemoryPressureRequest, MemoryPressureResponse,
};
use log::info;
use std::time::Duration;
use tokio::time::sleep;
use tonic::{Request, Response, Status};

pub mod grpc_service {
    tonic::include_proto!("grpc_pressure");
}

#[derive(Debug, Default)]
pub struct PressureService;

#[tonic::async_trait]
impl GrpcStressSimulatorService for PressureService {
    async fn simulate_cpu_load(
        &self,
        request: Request<CpuLoadRequest>,
    ) -> Result<Response<CpuLoadResponse>, Status> {
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

        Ok(Response::new(CpuLoadResponse {
            message: format!(
                "Simulating CPU burst for {}ms with upper bound {}",
                request_inner.duration_ms, upper_bound
            ),
        }))
    }

    async fn simulate_memory_pressure(
        &self,
        request: Request<MemoryPressureRequest>,
    ) -> Result<Response<MemoryPressureResponse>, Status> {
        let inner_request = request.into_inner();
        let duration = Duration::from_millis(inner_request.duration_ms as u64);
        let memory_bytes = inner_request.memory_bytes as usize;

        info!(
            "Allocating {} bytes of memory for {} ms",
            memory_bytes, inner_request.duration_ms
        );

        {
            let mut hog = vec![0u8; memory_bytes];
            for i in 0..memory_bytes {
                hog[i] = (i % 256) as u8;
            }

            sleep(duration).await;

            drop(hog);
        }

        info!(
            "Released {} bytes of memory after {} ms",
            memory_bytes, inner_request.duration_ms
        );

        Ok(Response::new(MemoryPressureResponse {
            message: format!(
                "Allocated {} bytes of memory for {} ms",
                memory_bytes, inner_request.duration_ms
            ),
        }))
    }
}
