use tonic::Request;
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;

use tracing::level_filters::LevelFilter;
use tracing::*;
use tracing_subscriber::{
    Registry,
    filter::{self, FilterExt},
    prelude::*,
};

use smartauto::echo_service_client::EchoServiceClient;
use smartauto::*;

pub mod smartauto {
    tonic::include_proto!("proto.smartauto.v1");
}

#[tracing::instrument(level = "trace", skip())]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter_project =
        filter::filter_fn(|metadata| metadata.target().starts_with(module_path!()))
            .and(LevelFilter::TRACE);

    let default_stdout_log = tracing_subscriber::fmt::layer()
        .pretty()
        .without_time()
        .with_filter(filter_project.or(LevelFilter::INFO));

    Registry::default().with(default_stdout_log).init();

    let channel = Channel::builder("http://127.0.0.1:3000".parse().unwrap())
        .connect()
        .await?;

    let mut client = EchoServiceClient::new(channel)
        .send_compressed(CompressionEncoding::Zstd)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Gzip);

    let request = Request::new(SayHelloRequest {
        name: "Tonic123".into(),
    });

    let response = client.say_hello(request).await?;

    debug!("RESPONSE={:#?}", response);

    Ok(())
}
