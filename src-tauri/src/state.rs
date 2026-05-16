//! Estado compartido de la aplicación.
//!
//! Mantiene la conexión a la base SQLite del proyecto **actualmente abierto**.
//! La aplicación es single-project en runtime: abrir otro proyecto sustituye
//! la conexión activa.

use std::path::PathBuf;
use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::db::open_project_database;
use crate::entities::project;
use crate::error::{AppError, AppResult};
use crate::paths::ProjectPaths;
use crate::services::credential_service::CredentialService;

const RECENT_PROJECTS_FILE: &str = "recent-projects.json";
const MAX_RECENT_PROJECTS: usize = 20;

#[derive(Default, Serialize, Deserialize)]
struct RecentProjects {
    projects: Vec<project::Model>,
}

pub struct OpenProject {
    pub paths: ProjectPaths,
    pub db: DatabaseConnection,
}

pub struct AppState {
    open_project: RwLock<Option<Arc<OpenProject>>>,
    pub credentials: CredentialService,
    recent_projects_path: PathBuf,
}

impl AppState {
    pub async fn initialize<R: tauri::Runtime>(_app: &tauri::AppHandle<R>) -> AppResult<Self> {
        Ok(Self {
            open_project: RwLock::new(None),
            credentials: CredentialService::new(),
            recent_projects_path: recent_projects_path()?,
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

    pub fn list_recent_projects(&self) -> AppResult<Vec<project::Model>> {
        Ok(read_recent_projects(&self.recent_projects_path)?.projects)
    }

    pub fn remember_project(&self, project: &project::Model) -> AppResult<()> {
        let mut recent = read_recent_projects(&self.recent_projects_path)?;
        recent.projects.retain(|item| item.id != project.id);
        recent.projects.insert(0, project.clone());
        recent.projects.truncate(MAX_RECENT_PROJECTS);
        write_recent_projects(&self.recent_projects_path, &recent)
    }
}

fn recent_projects_path() -> AppResult<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::internal("no se pudo resolver el directorio de configuración"))?
        .join("broadwai-tts-studio");
    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join(RECENT_PROJECTS_FILE))
}

fn read_recent_projects(path: &PathBuf) -> AppResult<RecentProjects> {
    if !path.exists() {
        return Ok(RecentProjects::default());
    }
    let content = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

fn write_recent_projects(path: &PathBuf, recent: &RecentProjects) -> AppResult<()> {
    let content = serde_json::to_string_pretty(recent)?;
    std::fs::write(path, content)?;
    Ok(())
}
