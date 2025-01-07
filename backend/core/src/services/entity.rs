use std::sync::Arc;

use tonic::{Request, Response, Status};
use tonic_types::BadRequest;

use tracing::*;

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
        info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let mut bad_request = BadRequest::new(vec![]);

        bad_request.add_required("id", &message.id);
        if let Some(id) = message.id.as_ref() {
            bad_request.add_not_valid_id("id", &id.id);
        }

        bad_request.add_not_valid_type("type", message.r#type);

        bad_request.has_violation()?;

        debug!("Recieved:\n{:#?}", message);

        let id = message.id.unwrap();
        let r#type = EntityType::try_from(message.r#type)
            .map_err(|err| Status::from_error(Box::new(err)))?
            .as_str_name();

        self.database
            .create_entity(&id.id, r#type)
            .await
            .map_err(Status::from_error)?;

        Ok(Response::new(CreateEntityResponse {}))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn update_entity(
        &self,
        request: Request<UpdateEntityRequest>,
    ) -> Result<Response<UpdateEntityResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let mut bad_request = BadRequest::new(vec![]);

        bad_request.add_required("id", &message.id);
        bad_request.add_required("value", &message.value);

        if let Some(id) = message.id.as_ref() {
            bad_request.add_not_valid_id("id", &id.id);
        }
        if let Some(value) = message.value.as_ref() {
            bad_request.add_required("value.value", &value.value);
        }

        bad_request.has_violation()?;

        debug!("Recieved:\n{:#?}", message);

        let id = message.id.unwrap();
        let value = message.value.unwrap().value.unwrap();

        self.database
            .add_entity_data(&id.id, &value)
            .await
            .map_err(Status::from_error)?;

        Ok(Response::new(UpdateEntityResponse {}))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> Result<Response<GetEntityResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let mut bad_request = BadRequest::new(vec![]);

        bad_request.add_required("id", &message.id);

        if let Some(id) = message.id.as_ref() {
            bad_request.add_not_valid_id("id", &id.id);
        }

        bad_request.has_violation()?;

        debug!("Recieved:\n{:#?}", message);

        let response = GetEntityResponse {
            value: Some(EntityValue {
                value: Some(entity_value::Value::Bool(true)),
            }),
        };
        Ok(Response::new(response))
    }
}
