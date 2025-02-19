// Constant values for reuse
pub const GITHUB_URL: &str = "https://github.com/LDprg/smartauto";
pub const ENTITY_ID_REGEX: &str = r"^[a-zA-Z_][a-zA-Z0-9_]{2,}$";

// Constants for env vars
pub const ENV_HOST_URI: &str = "SA_HOST";
pub const ENV_SCYLLA_URI: &str = "SA_SCYLLA";
pub const ENV_DEFAULT_USER: &str = "SA_DEFAULT_USER";
pub const ENV_DEFAULT_PASSWORD: &str = "SA_DEFAULT_PASSWORD";

// Constant default value
pub const DEFAULT_HOST_URI: &str = "127.0.0.1:3000";
pub const DEFAULT_SCYLLA_URI: &str = "127.0.0.1:9042";
pub const DEFAULT_DEFAULT_USER: &str = "admin";
pub const DEFAULT_DEFAULT_PASSWORD: &str = "smartauto";

// Constant values that are not expected to change
pub const AUTH: &str = "authorization";
pub const BEARER: &str = "Bearer ";

// Constant values FOR NOW

pub const DATABASE_NAME: &str = "smartauto_db";
