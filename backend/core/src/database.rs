use scylla::{
    SessionBuilder, macros::DeserializeRow, prepared_statement::PreparedStatement,
    transport::session::Session,
};
use tonic::Status;
use tracing::*;
use uuid::Uuid;

use crate::smartauto::*;
use crate::util::*;
use crate::*;
use crate::{authentication::generate_pwd_hash, constants::*};

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
    entity_get_count_prepare: PreparedStatement,

    entity_data_bool_get_count_prepare: PreparedStatement,
    entity_data_int_get_count_prepare: PreparedStatement,
    entity_data_double_get_count_prepare: PreparedStatement,
    entity_data_string_get_count_prepare: PreparedStatement,
}

#[derive(DeserializeRow)]
pub struct User {
    pub name: String,
    pub password_hash: String,
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

        // TODO: Add needed types

        info!("Types created!");

        info!("Creating db Tables ...");

        execute_db!(
            session.query_unpaged("CREATE TABLE IF NOT EXISTS users (name text, uid uuid, created timestamp, disabled boolean, password_hash text, PRIMARY KEY (name))", &[]),
            session.query_unpaged("CREATE TABLE IF NOT EXISTS entity_register (id text, uid uuid, created timestamp, type text, PRIMARY KEY (id))", &[]),
            session.query_unpaged("CREATE TABLE IF NOT EXISTS entity_data (uid uuid, timestamp timestamp, data_bool boolean, data_int bigint, data_double double, data_string text, PRIMARY KEY ((uid), timestamp)) WITH CLUSTERING ORDER BY (timestamp DESC)", &[]),
        );

        info!("Tables created!");

        info!("Preparing db Queries ...");

        execute_db!(
            // INSERT
            session.prepare("INSERT INTO users (name, password_hash, disabled, uid, created) VALUES (?, ?, ?, uuid(), toTimestamp(now()))") => user_create_prepare,
            session.prepare("INSERT INTO entity_register (id, uid, type, created) VALUES (?, uuid(), ?, toTimestamp(now()))") => entity_create_prepare,
            session.prepare("INSERT INTO entity_data (uid, data_bool, timestamp) VALUES (?, ?, toTimestamp(now()))") => entity_data_bool_add_prepare,
            session.prepare("INSERT INTO entity_data (uid, data_int, timestamp) VALUES (?, ?, toTimestamp(now()))") => entity_data_int_add_prepare,
            session.prepare("INSERT INTO entity_data (uid, data_double, timestamp) VALUES (?, ?, toTimestamp(now()))") => entity_data_double_add_prepare,
            session.prepare("INSERT INTO entity_data (uid, data_string, timestamp) VALUES (?, ?, toTimestamp(now()))") => entity_data_string_add_prepare,
            // SELECT
            session.prepare("SELECT name, password_hash FROM users WHERE name = ?") => user_get_prepare,
            session.prepare("SELECT uid, type FROM entity_register WHERE id = ?") => entity_get_uid_type_prepare,
            session.prepare("SELECT COUNT(*) FROM entity_register WHERE id = ?") => entity_get_count_prepare,
            session.prepare("SELECT data_bool FROM entity_data WHERE uid = ?") => entity_data_bool_get_count_prepare,
            session.prepare("SELECT data_int FROM entity_data WHERE uid = ?") => entity_data_int_get_count_prepare,
            session.prepare("SELECT data_double FROM entity_data WHERE uid = ?") => entity_data_double_get_count_prepare,
            session.prepare("SELECT data_string FROM entity_data WHERE uid = ?") => entity_data_string_get_count_prepare,
        );

        info!("Queries prepared!");

        info!("Check users");

        let result = session
            .query_unpaged("SELECT COUNT(*) FROM users;", &[])
            .await?
            .into_rows_result()?
            .single_row::<(i64,)>()?;
        let user_cnt = result.0;

        if user_cnt == 0 {
            info!("No users found creating default ones!");

            let user =
                env::var(ENV_DEFAULT_USER).unwrap_or_else(|_| DEFAULT_DEFAULT_USER.to_string());
            let password = env::var(ENV_DEFAULT_PASSWORD)
                .unwrap_or_else(|_| DEFAULT_DEFAULT_PASSWORD.to_string());

            let password_hash = generate_pwd_hash(&password).map_err(|_| {
                Box::<dyn std::error::Error>::from("Generate password_hash failed!")
            })?;

            session
                .execute_unpaged(&user_create_prepare, (user, password_hash, false))
                .await?;

            info!("Default user has been created, change the password as soon as possible!");
        } else {
            info!("Found {} users", user_cnt);
        }

        info!("Users checked!");

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
            entity_get_count_prepare,

            entity_data_bool_get_count_prepare,
            entity_data_int_get_count_prepare,
            entity_data_double_get_count_prepare,
            entity_data_string_get_count_prepare,
        })
    }

    #[tracing::instrument(level = "trace", skip(self, user, password, disabled))]
    pub async fn create_user(
        &self,
        user: &str,
        password: &str,
        disabled: bool,
    ) -> Result<(), Status> {
        let password_hash = generate_pwd_hash(password)?;

        resolve_error!(
            self.session
                .execute_unpaged(&self.user_create_prepare, (user, password_hash, disabled),)
                .await
        )?;

        Ok(())
    }

    pub async fn get_user(&self, user: &str) -> Result<User, Status> {
        let result = resolve_error!(
            self.session
                .execute_unpaged(&self.user_get_prepare, (&user,))
                .await
        )?;
        let result = resolve_error!(result.into_rows_result())?;

        let data = resolve_error!(result.single_row::<User>())?;

        Ok(data)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn create_entity(&self, id: &str, r#type: &str) -> Result<(), Status> {
        let count = resolve_error!(
            self.session
                .execute_unpaged(&self.entity_get_count_prepare, (id,))
                .await
        )?;
        let count = resolve_error!(count.into_rows_result())?;
        let count = resolve_error!(count.single_row::<(i64,)>())?;

        if count.0 == 0 {
            resolve_error!(
                self.session
                    .execute_unpaged(&self.entity_create_prepare, (id, r#type))
                    .await
            )?;

            return Ok(());
        }
        Err(Status::invalid_argument("Entity already exists")) // TODO: Replace with proper error handling
    }

    #[tracing::instrument(level = "trace", skip(self, id, value))]
    pub async fn add_entity_data(
        &self,
        id: &str,
        value: &entity_value::Value,
    ) -> Result<(), Status> {
        let info = resolve_error!(
            self.session
                .execute_unpaged(&self.entity_get_uid_type_prepare, (id,))
                .await
        )?;

        let info = resolve_error!(info.into_rows_result())?;
        let info = resolve_error!(info.single_row::<(Uuid, &str)>())?;

        let uid = info.0;
        if let Some(r#type) = EntityType::from_str_name(info.1) {
            if value.as_type() != r#type {
                return Err(Status::invalid_argument(format!(
                    "Value has wrong type! Should be: {}",
                    info.1
                ))); // TODO: Replace with proper error handling
            }

            match value {
                entity_value::Value::Bool(value) => {
                    resolve_error!(
                        self.session
                            .execute_unpaged(&self.entity_data_bool_add_prepare, (uid, value))
                            .await
                    )?;
                }
                entity_value::Value::Int(value) => {
                    resolve_error!(
                        self.session
                            .execute_unpaged(&self.entity_data_int_add_prepare, (uid, value))
                            .await
                    )?;
                }
                entity_value::Value::Double(value) => {
                    resolve_error!(
                        self.session
                            .execute_unpaged(&self.entity_data_double_add_prepare, (uid, value))
                            .await
                    )?;
                }
                entity_value::Value::String(value) => {
                    resolve_error!(
                        self.session
                            .execute_unpaged(&self.entity_data_string_add_prepare, (uid, value))
                            .await
                    )?;
                }
            }
            return Ok(());
        }
        Err(Status::invalid_argument("Invalid entity type")) // TODO: Replace with proper error handling
    }

    pub async fn get_entity_data(&self, id: &str) -> Result<entity_value::Value, Status> {
        let info = resolve_error!(
            self.session
                .execute_unpaged(&self.entity_get_uid_type_prepare, (id,))
                .await
        )?;

        let info = resolve_error!(info.into_rows_result())?;
        let (uid, r#type) = resolve_error!(info.single_row::<(Uuid, &str)>())?;

        if let Some(r#type) = EntityType::from_str_name(r#type) {
            let statement = match r#type {
                EntityType::Bool => &self.entity_data_bool_get_count_prepare,
                EntityType::Int => &self.entity_data_int_get_count_prepare,
                EntityType::Double => &self.entity_data_double_get_count_prepare,
                EntityType::String => &self.entity_data_string_get_count_prepare,
                EntityType::Unspecified => return Err(Status::internal("Database corruped!")),
            };

            let data = resolve_error!(self.session.execute_unpaged(statement, (uid,)).await)?;
            let data = resolve_error!(data.into_rows_result())?;
            let value = match r#type {
                EntityType::Bool => {
                    entity_value::Value::Bool(resolve_error!(data.first_row::<(bool,)>())?.0)
                }
                EntityType::Int => {
                    entity_value::Value::Int(resolve_error!(data.first_row::<(i64,)>())?.0)
                }
                EntityType::Double => {
                    entity_value::Value::Double(resolve_error!(data.first_row::<(f64,)>())?.0)
                }
                EntityType::String => {
                    entity_value::Value::String(resolve_error!(data.first_row::<(String,)>())?.0)
                }
                EntityType::Unspecified => return Err(Status::internal("Database corruped!")),
            };

            return Ok(value);
        }
        error!("Database corruped!");
        Err(Status::internal("Database corruped")) // TODO: Replace with proper error handling
    }
}
