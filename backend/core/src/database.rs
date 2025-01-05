use std::sync::Arc;

use scylla::{SessionBuilder, prepared_statement::PreparedStatement, transport::session::Session};
use tokio::task::JoinSet;
use tracing::*;
use uuid::Uuid;

use crate::constants::*;
use crate::smartauto::*;

macro_rules! execute_db_async {
    ($session:expr, $vec:expr) => {{
        let mut set: JoinSet<_> = $vec
            .into_iter()
            .map(|query| {
                let session: Arc<Session> = $session.clone();
                tokio::task::spawn(async move { session.query_unpaged(query, &[]).await })
            })
            .collect();

        while let Some(res) = set.join_next().await {
            let _ = res???;
        }
    }};
}

macro_rules! prepare_entity_data {
    ($session:expr, $query_string:expr, $($var_name:ident => $table_name:expr),*) => {
        $(
            let $var_name = $session
                .prepare(format!(
                    $query_string,
                    $table_name
                ))
                .await?;
        )*
    };
}

#[derive(Clone)]
pub struct Database {
    session: Arc<Session>,

    // INSERT
    entity_create_prepare: PreparedStatement,

    entity_data_bool_add_prepare: PreparedStatement,
    entity_data_int_add_prepare: PreparedStatement,
    entity_data_float_add_prepare: PreparedStatement,
    entity_data_string_add_prepare: PreparedStatement,

    // SELECT
    entity_get_uid_type_prepare: PreparedStatement,
    entity_get_uid_prepare: PreparedStatement,
}

impl Database {
    #[tracing::instrument(level = "trace", skip(uri))]
    pub async fn new(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        info!(%uri, "Connecting to db ...");

        let session: Session = SessionBuilder::new().known_node(uri).build().await?;
        let session: Arc<Session> = Arc::new(session);

        info!("Connected to db!");

        info!("Creating db Keyspace ...");

        session.query_unpaged(format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}}", DATABASE_NAME), &[]).await?;
        session.use_keyspace(DATABASE_NAME, false).await?;

        info!("Keyspace created!");

        info!("Creating db Types ...");
        info!("Types created!");

        info!("Creating db Tables ...");

        execute_db_async!(session, vec![
            "CREATE TABLE IF NOT EXISTS entity_register (id text, uid uuid, created timestamp, type text, PRIMARY KEY (id))",
            "CREATE TABLE IF NOT EXISTS entity_data_bool (uid uuid, timestamp timestamp, data boolean, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)",
            "CREATE TABLE IF NOT EXISTS entity_data_int (uid uuid, timestamp timestamp, data bigint, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)",
            "CREATE TABLE IF NOT EXISTS entity_data_float (uid uuid, timestamp timestamp, data double, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)",
            "CREATE TABLE IF NOT EXISTS entity_data_string (uid uuid, timestamp timestamp, data text, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)",
        ]);

        info!("Tables created!");

        info!("Preparing db Queries ...");

        // INSERT
        let entity_create_prepare = session
            .prepare("INSERT INTO entity_register (id, uid, type, created) VALUES (?, uuid(), ?, toTimestamp(now()))")
            .await?;

        prepare_entity_data!(
            session,
            "INSERT INTO {} (uid, data, timestamp) VALUES (?, ?, toTimestamp(now()))",
            entity_data_bool_add_prepare => "entity_data_bool",
            entity_data_int_add_prepare => "entity_data_int",
            entity_data_float_add_prepare => "entity_data_float",
            entity_data_string_add_prepare => "entity_data_string"
        );

        // SELECT
        let entity_get_uid_type_prepare = session
            .prepare("SELECT uid, type FROM entity_register WHERE id = ?")
            .await?;

        let entity_get_uid_prepare = session
            .prepare("SELECT uid FROM entity_register WHERE id = ?")
            .await?;

        info!("Queries prepared!");

        Ok(Self {
            session,
            entity_create_prepare,
            entity_data_bool_add_prepare,
            entity_data_int_add_prepare,
            entity_data_float_add_prepare,
            entity_data_string_add_prepare,
            entity_get_uid_type_prepare,
            entity_get_uid_prepare,
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn create_entity(
        &self,
        id: &str,
        r#type: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let exist = self
            .session
            .execute_unpaged(&self.entity_get_uid_prepare, (id,))
            .await?
            .into_rows_result()?;
        let mut exist = exist.rows::<(Uuid,)>()?;

        if exist.next().transpose()?.is_none() {
            self.session
                .execute_unpaged(&self.entity_create_prepare, (id, r#type))
                .await?;

            return Ok(());
        }
        Err("Entity already created".into())
    }

    pub async fn add_entity_data(
        &self,
        id: &str,
        value: &entity_value::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let info = self
            .session
            .execute_unpaged(&self.entity_get_uid_type_prepare, (id,))
            .await?
            .into_rows_result()?;
        let mut info = info.rows::<(Uuid, &str)>()?;

        if let Some(info) = info.next().transpose()? {
            let uid = info.0;
            if let Some(r#type) = EntityType::from_str_name(info.1) {
                let value_type = match value {
                    entity_value::Value::Bool(_) => EntityType::Bool,
                    entity_value::Value::Int(_) => EntityType::Int,
                    entity_value::Value::Float(_) => EntityType::Float,
                    entity_value::Value::String(_) => EntityType::String,
                };

                if value_type != r#type {
                    return Err(format!("Value has wrong type! Should be: {}", info.1).into());
                }

                match value {
                    entity_value::Value::Bool(value) => {
                        self.session
                            .execute_unpaged(&self.entity_data_bool_add_prepare, (uid, value))
                            .await?;
                    }
                    entity_value::Value::Int(value) => {
                        self.session
                            .execute_unpaged(&self.entity_data_int_add_prepare, (uid, value))
                            .await?;
                    }
                    entity_value::Value::Float(value) => {
                        self.session
                            .execute_unpaged(&self.entity_data_float_add_prepare, (uid, value))
                            .await?;
                    }
                    entity_value::Value::String(value) => {
                        self.session
                            .execute_unpaged(&self.entity_data_string_add_prepare, (uid, value))
                            .await?;
                    }
                }
                return Ok(());
            }
            return Err("Entity doesn't exist".into());
        }
        Err("Invalid Entity type!".into())
    }
}
