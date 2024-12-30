use tonic::{Request, Response, Status, transport::Server};

use smartauto::greeter_service_server::{GreeterService, GreeterServiceServer};
use smartauto::{SayHelloRequest, SayHelloResponse};

pub mod smartauto {
    tonic::include_proto!("smartauto.v1");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("smartauto_v1_descriptor");
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

        let response = smartauto::SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let service_reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(smartauto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let greeter = MyGreeter::default();
    let greeter = GreeterServiceServer::new(greeter);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(service_reflection)
        .add_service(tonic_web::enable(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
