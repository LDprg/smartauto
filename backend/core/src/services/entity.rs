use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};
use tonic_types::BadRequest;

use tracing::*;

use crate::{database::Database, services::util::*};
use crate::{smartauto::*, try_required};

pub use crate::smartauto::entity_service_server::{EntityService, EntityServiceServer};

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

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

        let id = try_required!(message.id, "id")?;

        let mut bad_request = BadRequest::new(vec![]);
        bad_request.validate_id("id", &id.id);
        bad_request.validate_type("type", message.r#type);
        bad_request.has_violation()?;

        debug!("Recieved:\n{:#?}", message);

        let r#type = EntityType::try_from(message.r#type)
            .map_err(|err| Status::from_error(Box::new(err)))?
            .as_str_name();

        self.database.create_entity(&id.id, r#type).await?;

        Ok(Response::new(CreateEntityResponse {}))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn update_entity(
        &self,
        request: Request<UpdateEntityRequest>,
    ) -> Result<Response<UpdateEntityResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let id = try_required!(message.id, "id")?;
        let value = try_required!(message.value, "value")?;
        let value = try_required!(value.value, "value.value")?;

        let mut bad_request = BadRequest::new(vec![]);
        bad_request.validate_id("id", &id.id);
        bad_request.has_violation()?;

        debug!("Recieved:\n{:#?}", message);

        self.database.add_entity_data(&id.id, value).await?;

        Ok(Response::new(UpdateEntityResponse {}))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> Result<Response<GetEntityResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        let message = request.into_inner();

        let id = try_required!(&message.id, "id")?;

        let mut bad_request = BadRequest::new(vec![]);
        bad_request.validate_id("id", &id.id);
        bad_request.has_violation()?;

        debug!("Recieved:\n{:#?}", message);

        let value = self.database.get_entity_data(&id.id).await?;

        let response = GetEntityResponse {
            value: Some(EntityValue { value: Some(value) }),
        };

        Ok(Response::new(response))
    }

    type GetEntityStreamStream = ResponseStream<GetEntityStreamResponse>;
    async fn get_entity_stream(
        &self,
        request: Request<Streaming<GetEntityStreamRequest>>,
    ) -> Result<Response<Self::GetEntityStreamStream>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            let res = tx
                .send(Ok(GetEntityStreamResponse {
                    id: None,
                    value: None,
                }))
                .await;

            if let Err(err) = res {
                error!(%err);
            }
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::GetEntityStreamStream
        ))
    }
}
