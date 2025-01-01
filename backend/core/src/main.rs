use tonic::codec::CompressionEncoding;
use tonic::transport::Server;

mod services;
mod smartauto;

use services::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let service_reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(smartauto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let greeter = echo::EchoImpl::default();
    let greeter = echo::EchoServiceServer::new(greeter)
        .send_compressed(CompressionEncoding::Zstd)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Gzip);

    let auth = auth::AuthImpl::default();
    let auth = auth::AuthServiceServer::new(auth);

    let entity = entity::EntityImpl::default();
    let entity = entity::EntityServiceServer::with_interceptor(entity, auth::check_auth);

    println!("SmartAuto backend listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(service_reflection)
        .add_service(tonic_web::enable(greeter))
        .add_service(tonic_web::enable(auth))
        .add_service(tonic_web::enable(entity))
        .serve(addr)
        .await?;

    Ok(())
}
