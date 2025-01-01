use tonic::{Request, Response, Status};

use crate::smartauto::*;

pub use crate::smartauto::echo_service_server::{EchoService, EchoServiceServer};

#[derive(Default)]
pub struct EchoImpl {}

#[tonic::async_trait]
impl EchoService for EchoImpl {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let response = SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(response))
    }
}
