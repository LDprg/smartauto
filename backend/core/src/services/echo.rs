use tonic::{Request, Response, Status};

use tracing::*;

use crate::smartauto::*;

pub use crate::smartauto::echo_service_server::{EchoService, EchoServiceServer};

#[derive(Debug, Default)]
pub struct EchoImpl {}

#[tonic::async_trait]
impl EchoService for EchoImpl {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let response = SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(response))
    }
}
