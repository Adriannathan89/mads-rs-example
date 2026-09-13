use mads::{
    core::{Config, Diagnostic, Error, MADS006},
    prelude::*,
};
use sea_orm::{Database, DatabaseConnection};

#[module]
pub struct DatabaseModule;

#[derive(Configuration)]
#[config(prefix = "database")]
struct OrmDatabaseConfig {
    url: Secret<String>,
}

#[provider]
pub async fn database_connection(config: Config) -> mads::core::Result<DatabaseConnection> {
    let settings: OrmDatabaseConfig = config.parse()?;

    Database::connect(settings.url.expose().clone()).await.map_err(|error| {
        Error::with_source(
            Diagnostic::new(MADS006, "database connection failed", "SeaORM could not connect to the configured database"),
            error,
        )
    })
}
