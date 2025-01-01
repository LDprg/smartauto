use tonic::Request;
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;

use smartauto::greeter_service_client::GreeterServiceClient;
use smartauto::*;

pub mod smartauto {
    tonic::include_proto!("smartauto.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::builder("http://127.0.0.1:3000".parse().unwrap())
        .connect()
        .await?;

    let mut client = GreeterServiceClient::new(channel)
        .send_compressed(CompressionEncoding::Zstd)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Gzip);

    let request = Request::new(SayHelloRequest {
        name: "Tonic123".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:#?}", response);

    Ok(())
}
