//! Conexión y bootstrap de SQLite por proyecto.

use std::path::Path;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::error::AppResult;
use crate::migrations::Migrator;

/// Abre (o crea) la base SQLite de un proyecto, aplica migraciones y devuelve la conexión.
pub async fn open_project_database(database_file: &Path) -> AppResult<DatabaseConnection> {
    if let Some(parent) = database_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let url = format!(
        "sqlite://{}?mode=rwc",
        database_file
            .to_str()
            .ok_or_else(|| crate::error::AppError::invalid("ruta de DB inválida"))?
    );

    let mut opts = ConnectOptions::new(url);
    opts.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(false);

    let conn = Database::connect(opts).await?;
    Migrator::up(&conn, None).await?;
    Ok(conn)
}
