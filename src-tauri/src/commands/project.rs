use std::path::PathBuf;

use tauri::State;

use crate::entities::project;
use crate::error::{AppError, AppResult};
use crate::paths::ProjectPaths;
use crate::services::project_service;
use crate::state::AppState;

#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    title: String,
    description: Option<String>,
    language: Option<String>,
    root_path: String,
) -> AppResult<project::Model> {
    let root = PathBuf::from(&root_path);
    let paths = ProjectPaths::new(root.clone());
    paths.create_all()?;
    state.open(root).await?;

    let current = state.current().await?;
    project_service::create(
        &current.db,
        project_service::CreateProjectInput {
            title,
            description,
            language,
            root_path: paths.root.clone(),
        },
    )
    .await
}

#[tauri::command]
pub async fn open_project(state: State<'_, AppState>, root_path: String) -> AppResult<()> {
    let root = PathBuf::from(root_path);
    let paths = ProjectPaths::new(root.clone());
    paths.validate()?;
    state.open(root).await
}

#[tauri::command]
pub async fn list_recent_projects(
    state: State<'_, AppState>,
) -> AppResult<Vec<project::Model>> {
    let current = state.current().await?;
    project_service::list(&current.db).await
}

#[tauri::command]
pub async fn update_project(
    _state: State<'_, AppState>,
    _id: String,
    _title: Option<String>,
    _description: Option<String>,
    _language: Option<String>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_project"))
}

#[tauri::command]
pub async fn delete_project(_state: State<'_, AppState>, _id: String) -> AppResult<()> {
    Err(AppError::NotImplemented("delete_project"))
}

#[tauri::command]
pub async fn export_project(
    _state: State<'_, AppState>,
    _id: String,
    _target_path: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("export_project"))
}

#[tauri::command]
pub async fn import_project(
    _state: State<'_, AppState>,
    _source_path: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("import_project"))
}
