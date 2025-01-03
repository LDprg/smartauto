use scylla::{SessionBuilder, prepared_statement::PreparedStatement, transport::session::Session};

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

        session.query_unpaged("CREATE KEYSPACE IF NOT EXISTS examples_ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}", &[]).await?;
        session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS examples_ks.basic (a int, b int, c text, primary key (a, b))",
            &[],
        )
        .await?;

        tracing::info!("Default structure created!");

        tracing::info!("Prepare CQL Queries ...");

        let create_entity_prepare = session
            .prepare("INSERT INTO examples_ks.basic (a, b, c) VALUES (?, ?, ?)")
            .await?;

        tracing::info!("Queries prepared!");

        Ok(Self {
            session,
            create_entity_prepare,
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn create_client(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.session
            .execute_unpaged(&self.create_entity_prepare, (3, 5, "def1"))
            .await?;

        Ok(())
    }
}
