use entity_value::Value;
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
        println!("Recieved: {:#?}", request.into_inner());

        let response = CreateEntityResponse {};
        Ok(Response::new(response))
    }

    async fn update_entity(
        &self,
        request: Request<UpdateEntityRequest>,
    ) -> Result<Response<UpdateEntityResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        println!("Recieved: {:#?}", request.into_inner());

        let response = UpdateEntityResponse {};
        Ok(Response::new(response))
    }

    async fn get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> Result<Response<GetEntityResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        println!("Recieved: {:#?}", request.into_inner());

        let response = GetEntityResponse {
            value: Some(EntityValue {
                value: Some(Value::Bool(true)),
            }),
        };
        Ok(Response::new(response))
    }
}
