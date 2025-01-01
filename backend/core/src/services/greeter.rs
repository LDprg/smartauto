use tonic::{Request, Response, Status};

use crate::smartauto::*;

pub use crate::smartauto::greeter_service_server::{GreeterService, GreeterServiceServer};

#[derive(Default)]
pub struct GreeterImpl {}

#[tonic::async_trait]
impl GreeterService for GreeterImpl {
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
