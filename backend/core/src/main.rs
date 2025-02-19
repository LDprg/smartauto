use std::env;
use std::sync::Arc;

use database::Database;
use tonic::transport::Server;
use tonic::{codec::CompressionEncoding, service::interceptor::InterceptedService};

mod constants;

mod authentication;
mod database;
mod services;
mod smartauto;
mod util;

use constants::*;
use services::*;
use tracing::level_filters::LevelFilter;
use tracing::*;
use tracing_subscriber::{
    Registry,
    filter::{self, FilterExt},
    prelude::*,
};

macro_rules! new_service {
    ($e:expr) => {{
        $e.send_compressed(CompressionEncoding::Zstd)
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Zstd)
            .accept_compressed(CompressionEncoding::Gzip)
    }};
}

#[tracing::instrument(level = "trace", skip())]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::var(ENV_HOST_URI)
        .unwrap_or_else(|_| DEFAULT_HOST_URI.to_string())
        .parse()?;
    let uri = env::var(ENV_SCYLLA_URI).unwrap_or_else(|_| DEFAULT_SCYLLA_URI.to_string());

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
    let greeter = new_service!(echo::EchoServiceServer::new(greeter));

    let auth = auth::AuthImpl::new(database.clone());
    let auth = new_service!(auth::AuthServiceServer::new(auth));

    let admin = admin::AdminImpl::new(database.clone());
    let admin = new_service!(admin::AdminServiceServer::new(admin));
    let admin = InterceptedService::new(admin, authentication::check_auth);

    let entity = entity::EntityImpl::new(database.clone());
    let entity = new_service!(entity::EntityServiceServer::new(entity));
    let entity = InterceptedService::new(entity, authentication::check_auth);

    info!(%addr, "SmartAuto backend ready!");

    Server::builder()
        .accept_http1(true)
        .add_service(service_reflection)
        .add_service(tonic_web::enable(greeter))
        .add_service(tonic_web::enable(auth))
        .add_service(tonic_web::enable(admin))
        .add_service(tonic_web::enable(entity))
        .serve(addr)
        .await?;

    Ok(())
}
