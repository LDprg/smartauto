use tonic::{Request, Response, Status, metadata::MetadataValue};

use crate::smartauto::*;

pub use crate::smartauto::auth_service_server::{AuthService, AuthServiceServer};

pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "ABC".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[derive(Default)]
pub struct AuthImpl {}

#[tonic::async_trait]
impl AuthService for AuthImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let response = LoginResponse {};
        Ok(Response::new(response))
    }
}
