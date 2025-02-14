use std::sync::Arc;

use tonic::{Request, Response, Status};
use tracing::*;

use crate::authentication::*;
use crate::database::*;
use crate::smartauto::*;

pub use crate::smartauto::auth_service_server::{AuthService, AuthServiceServer};

pub struct AuthImpl {
    database: Arc<Database>,
}

impl AuthImpl {
    #[tracing::instrument(level = "trace", skip(database))]
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl AuthService for AuthImpl {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let user = message.user;
        let password = message.password;

        let data = self.database.get_user(&user).await?;

        validate_pwd_hash(&password, &data.password_hash)?;

        let response = LoginResponse {
            access_token: auth_keys.generate_token(&data.name)?,
        };
        Ok(Response::new(response))
    }
}
