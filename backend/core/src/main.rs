use tonic::{Request, Response, Status, transport::Server};

use hello_world::greeter_service_server::{GreeterService, GreeterServiceServer};
use hello_world::{SayHelloRequest, SayHelloResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld.v1");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl GreeterService for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let response = hello_world::SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
