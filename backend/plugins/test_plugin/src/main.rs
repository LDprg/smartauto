use hello_world::SayHelloRequest;
use hello_world::greeter_service_client::GreeterServiceClient;

pub mod hello_world {
    tonic::include_proto!("helloworld.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterServiceClient::connect("http://127.0.0.1:3000").await?;

    let request = tonic::Request::new(SayHelloRequest {
        name: "Tonic123".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
