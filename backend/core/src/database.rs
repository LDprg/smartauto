use scylla::{SessionBuilder, prepared_statement::PreparedStatement, transport::session::Session};

use crate::constants::*;

pub struct Database {
    session: Session,

    create_entity_prepare: PreparedStatement,
}

impl Database {
    #[tracing::instrument(level = "trace", skip(uri))]
    pub async fn new(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        tracing::info!("Connecting to db {} ...", uri);

        let session: Session = SessionBuilder::new().known_node(uri).build().await?;

        tracing::info!("Connected to db!");

        tracing::info!("Creating default db structure ...");

        session.query_unpaged(format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}}", DATABASE_NAME), &[]).await?;

        session
        .query_unpaged(
            format!("CREATE TABLE IF NOT EXISTS {}.entity_register (id text, type text, primary key (id))", DATABASE_NAME),
            &[],
        )
        .await?;

        tracing::info!("Default structure created!");

        tracing::info!("Prepare CQL Queries ...");

        let create_entity_prepare = session
            .prepare(format!(
                "INSERT INTO {}.entity_register (id, type) VALUES (?, ?)",
                DATABASE_NAME
            ))
            .await?;

        tracing::info!("Queries prepared!");

        Ok(Self {
            session,
            create_entity_prepare,
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn create_entity(
        &self,
        id: &str,
        r#type: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.session
            .execute_unpaged(&self.create_entity_prepare, (id, r#type))
            .await?;

        Ok(())
    }
}
