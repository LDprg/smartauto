use std::env;
use std::sync::Arc;

use database::Database;
use tonic::codec::CompressionEncoding;
use tonic::transport::Server;

mod constants;

mod database;
mod services;
mod smartauto;
mod util;

use services::*;
use tracing::level_filters::LevelFilter;
use tracing::*;
use tracing_subscriber::{
    Registry,
    filter::{self, FilterExt},
    prelude::*,
};

#[tracing::instrument(level = "trace", skip())]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::var("HOST_URI")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
        .parse()?;
    let uri = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let filter_project =
        filter::filter_fn(|metadata| metadata.target().starts_with(module_path!()));

    let default_stdout_log = tracing_subscriber::fmt::layer()
        .pretty()
        .without_time()
        .with_filter(filter_project.or(LevelFilter::INFO));

    Registry::default().with(default_stdout_log).init();

    info!(%addr, "Starting SmartAuto backend ...");

    let database = Database::new(&uri).await;

    if let Err(err) = &database {
        error!(%err, "Failed to initialize the database");
    }

    let database = Arc::new(database?);

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

    let entity = entity::EntityImpl::new(database.clone());
    let entity = entity::EntityServiceServer::with_interceptor(entity, auth::check_auth);

    info!(%addr, "SmartAuto backend ready!");

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
