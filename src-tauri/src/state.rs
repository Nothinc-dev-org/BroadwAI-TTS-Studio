//! Estado compartido de la aplicación.
//!
//! Mantiene la conexión a la base SQLite del proyecto **actualmente abierto**.
//! La aplicación es single-project en runtime: abrir otro proyecto sustituye
//! la conexión activa.

use std::path::PathBuf;
use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

use crate::db::open_project_database;
use crate::error::{AppError, AppResult};
use crate::paths::ProjectPaths;
use crate::services::credential_service::CredentialService;

pub struct OpenProject {
    pub paths: ProjectPaths,
    pub db: DatabaseConnection,
}

pub struct AppState {
    open_project: RwLock<Option<Arc<OpenProject>>>,
    pub credentials: CredentialService,
}

impl AppState {
    pub async fn initialize<R: tauri::Runtime>(_app: &tauri::AppHandle<R>) -> AppResult<Self> {
        Ok(Self {
            open_project: RwLock::new(None),
            credentials: CredentialService::new(),
        })
    }

    pub async fn open(&self, root: PathBuf) -> AppResult<()> {
        let paths = ProjectPaths::new(root);
        let db = open_project_database(&paths.database_file()).await?;
        *self.open_project.write().await = Some(Arc::new(OpenProject { paths, db }));
        Ok(())
    }

    pub async fn current(&self) -> AppResult<Arc<OpenProject>> {
        self.open_project
            .read()
            .await
            .clone()
            .ok_or_else(|| AppError::invalid("no hay un proyecto abierto"))
    }

    pub async fn close(&self) {
        *self.open_project.write().await = None;
    }
}
