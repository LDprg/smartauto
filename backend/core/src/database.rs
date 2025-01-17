use scylla::{SessionBuilder, prepared_statement::PreparedStatement, transport::session::Session};
use tracing::*;
use uuid::Uuid;

use crate::constants::*;
use crate::smartauto::*;
use crate::util::*;

macro_rules! execute_db {
    ($($e:expr),+ $(,)?) => {{
        tokio::try_join!{
            $($e),*
        }?
    }};

    ($($e:expr => $v:ident),+ $(,)?) => {
        let ( $($v),* ) = execute_db!{
            $($e),*
        };
    };
}

#[allow(dead_code)]
pub struct Database {
    session: Session,

    // INSERT
    user_create_prepare: PreparedStatement,

    entity_create_prepare: PreparedStatement,

    entity_data_bool_add_prepare: PreparedStatement,
    entity_data_int_add_prepare: PreparedStatement,
    entity_data_double_add_prepare: PreparedStatement,
    entity_data_string_add_prepare: PreparedStatement,

    // SELECT
    user_get_prepare: PreparedStatement,

    entity_get_uid_type_prepare: PreparedStatement,
    entity_get_uid_prepare: PreparedStatement,
}

impl Database {
    #[tracing::instrument(level = "trace", skip(uri))]
    pub async fn new(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        info!(%uri, "Connecting to db ...");

        let session: Session = SessionBuilder::new().known_node(uri).build().await?;

        info!("Connected to db!");

        info!("Creating db Keyspace ...");

        session.query_unpaged(format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}}", DATABASE_NAME), &[]).await?;
        session.use_keyspace(DATABASE_NAME, false).await?;

        info!("Keyspace created!");

        info!("Creating db Types ...");
        info!("Types created!");

        info!("Creating db Tables ...");

        execute_db!(
            session.query_unpaged("CREATE TABLE IF NOT EXISTS users (name text, uid uuid, created timestamp, disabled boolean, password_hash text, PRIMARY KEY (name))", &[]),
            session.query_unpaged("CREATE TABLE IF NOT EXISTS entity_register (id text, uid uuid, created timestamp, type text, PRIMARY KEY (id))", &[]),
            session.query_unpaged("CREATE TABLE IF NOT EXISTS entity_data_bool (uid uuid, timestamp timestamp, data boolean, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)", &[]),
            session.query_unpaged("CREATE TABLE IF NOT EXISTS entity_data_int (uid uuid, timestamp timestamp, data bigint, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)", &[]),
            session.query_unpaged("CREATE TABLE IF NOT EXISTS entity_data_float (uid uuid, timestamp timestamp, data double, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)", &[]),
            session.query_unpaged("CREATE TABLE IF NOT EXISTS entity_data_string (uid uuid, timestamp timestamp, data text, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)", &[]),
        );

        info!("Tables created!");

        info!("Preparing db Queries ...");

        let entity_data_add_str =
            "INSERT INTO {} (uid, data, timestamp) VALUES (?, ?, toTimestamp(now()))";
        execute_db!(
            // INSERT
            session.prepare("INSERT INTO users (name, password_hash, disabled, uid, created) VALUES (?, ?, ?, uuid(), toTimestamp(now()))") => user_create_prepare,
            session.prepare("INSERT INTO entity_register (id, uid, type, created) VALUES (?, uuid(), ?, toTimestamp(now()))") => entity_create_prepare,
            session.prepare(entity_data_add_str.replace("{}", "entity_data_bool")) => entity_data_bool_add_prepare,
            session.prepare(entity_data_add_str.replace("{}", "entity_data_int")) => entity_data_int_add_prepare,
            session.prepare(entity_data_add_str.replace("{}", "entity_data_float")) => entity_data_double_add_prepare,
            session.prepare(entity_data_add_str.replace("{}", "entity_data_string")) => entity_data_string_add_prepare,
            // SELECT
            session.prepare("SELECT name, password_hash, uid FROM users WHERE name = ?") => user_get_prepare,
            session.prepare("SELECT uid, type FROM entity_register WHERE id = ?") => entity_get_uid_type_prepare,
            session.prepare("SELECT uid FROM entity_register WHERE id = ?") => entity_get_uid_prepare,
        );

        info!("Queries prepared!");

        Ok(Self {
            session,
            user_create_prepare,
            entity_create_prepare,
            entity_data_bool_add_prepare,
            entity_data_int_add_prepare,
            entity_data_double_add_prepare,
            entity_data_string_add_prepare,
            user_get_prepare,
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
        Err("Entity already exists".into())
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
                if value.as_type() != r#type {
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
                    entity_value::Value::Double(value) => {
                        self.session
                            .execute_unpaged(&self.entity_data_double_add_prepare, (uid, value))
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
