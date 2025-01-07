use tonic::{Request, Response, Status};

use tracing::*;

use crate::smartauto::*;

pub use crate::smartauto::admin_service_server::{AdminService, AdminServiceServer};

#[derive(Debug, Default)]
pub struct AdminImpl {}

#[tonic::async_trait]
impl AdminService for AdminImpl {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn add_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<AddUserResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let response = AddUserResponse {};
        Ok(Response::new(response))
    }
}
