use tonic::{Request, Response, Status};
use tracing::*;

use crate::smartauto::*;
use crate::*;

pub use crate::smartauto::admin_service_server::{AdminService, AdminServiceServer};

pub struct AdminImpl {
    database: Arc<Database>,
}

impl AdminImpl {
    #[tracing::instrument(level = "trace", skip(database))]
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl AdminService for AdminImpl {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn add_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<AddUserResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let user = message.user;
        let password = message.password;

        self.database.create_user(&user, &password, false).await?;

        let response = AddUserResponse {};
        Ok(Response::new(response))
    }
}
