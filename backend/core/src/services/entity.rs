use tonic::{Request, Response, Status};

use crate::smartauto::*;

pub use crate::smartauto::entity_service_server::{EntityService, EntityServiceServer};

#[derive(Default)]
pub struct EntityImpl {}

#[tonic::async_trait]
impl EntityService for EntityImpl {
    async fn create_entity(
        &self,
        request: Request<CreateEntityRequest>,
    ) -> Result<Response<CreateEntityResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let response = CreateEntityResponse {};
        Ok(Response::new(response))
    }
}
