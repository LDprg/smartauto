use std::sync::Arc;

use tonic::{Request, Response, Status};
use tonic_types::BadRequest;

use crate::smartauto::*;
use crate::{database::Database, services::util::*};

pub use crate::smartauto::entity_service_server::{EntityService, EntityServiceServer};

pub struct EntityImpl {
    database: Arc<Database>,
}

impl EntityImpl {
    #[tracing::instrument(level = "trace", skip(database))]
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl EntityService for EntityImpl {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn create_entity(
        &self,
        request: Request<CreateEntityRequest>,
    ) -> Result<Response<CreateEntityResponse>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let mut bad_request = BadRequest::new(vec![]);

        bad_request.add_required_cond(message.id.is_none(), "id");

        if let Some(id) = message.id.as_ref() {
            bad_request.add_not_empty("id.id", &id.id)
        }

        if let Some(status) = bad_request.has_violation() {
            return Err(status);
        }

        tracing::debug!("Recieved:\n{:#?}", message);

        if let Err(err) = self.database.create_client().await {
            return Err(Status::from_error(err));
        }

        let response = CreateEntityResponse {};
        Ok(Response::new(response))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn update_entity(
        &self,
        request: Request<UpdateEntityRequest>,
    ) -> Result<Response<UpdateEntityResponse>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let mut bad_request = BadRequest::new(vec![]);

        bad_request.add_required_cond(message.id.is_none(), "id");
        bad_request.add_required_cond(message.value.is_none(), "value");

        if let Some(id) = message.id.as_ref() {
            bad_request.add_not_empty("id.id", &id.id)
        }

        if let Some(status) = bad_request.has_violation() {
            return Err(status);
        }

        tracing::debug!("Recieved:\n{:#?}", message);

        let response = UpdateEntityResponse {};
        Ok(Response::new(response))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> Result<Response<GetEntityResponse>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let mut bad_request = BadRequest::new(vec![]);

        bad_request.add_required_cond(message.id.is_none(), "id");

        if let Some(id) = message.id.as_ref() {
            bad_request.add_not_empty("id.id", &id.id)
        }

        if let Some(status) = bad_request.has_violation() {
            return Err(status);
        }

        tracing::debug!("Recieved:\n{:#?}", message);

        let response = GetEntityResponse {
            value: Some(EntityValue {
                value: Some(entity_value::Value::Bool(true)),
            }),
        };
        Ok(Response::new(response))
    }
}
