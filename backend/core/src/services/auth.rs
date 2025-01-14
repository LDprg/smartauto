use tonic::{Request, Response, Status, metadata::MetadataValue};

use tracing::*;

use crate::smartauto::*;

pub use crate::smartauto::auth_service_server::{AuthService, AuthServiceServer};

#[tracing::instrument(level = "trace", skip(req))]
pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer ABC".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[derive(Debug, Default)]
pub struct AuthImpl {}

#[tonic::async_trait]
impl AuthService for AuthImpl {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let response = LoginResponse {
            access_token: "".to_string(),
        };
        Ok(Response::new(response))
    }
}
