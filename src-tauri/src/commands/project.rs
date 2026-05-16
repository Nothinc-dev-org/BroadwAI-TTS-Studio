use std::path::PathBuf;

use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use tauri::State;

use crate::entities::project;
use crate::error::{AppError, AppResult};
use crate::paths::ProjectPaths;
use crate::services::project_io_service;
use crate::services::{now, project_service};
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
    let project = project_service::create(
        &current.db,
        project_service::CreateProjectInput {
            title,
            description,
            language,
            root_path: paths.root.clone(),
        },
    )
    .await?;
    state.remember_project(&project)?;
    Ok(project)
}

#[tauri::command]
pub async fn open_project(state: State<'_, AppState>, root_path: String) -> AppResult<()> {
    let root = PathBuf::from(root_path);
    let paths = ProjectPaths::new(root.clone());
    paths.validate()?;
    state.open(root).await?;
    let current = state.current().await?;
    if let Some(project) = project_service::list(&current.db).await?.into_iter().next() {
        state.remember_project(&project)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn list_recent_projects(state: State<'_, AppState>) -> AppResult<Vec<project::Model>> {
    state.list_recent_projects()
}

#[tauri::command]
pub async fn update_project(
    state: State<'_, AppState>,
    id: String,
    title: Option<String>,
    description: Option<String>,
    language: Option<String>,
) -> AppResult<project::Model> {
    let current = state.current().await?;
    let model = project::Entity::find_by_id(id.clone())
        .one(&current.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("project {id}")))?;
    let mut active = model.into_active_model();
    if let Some(value) = title {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(AppError::invalid("el título no puede estar vacío"));
        }
        active.title = Set(trimmed.to_owned());
    }
    if let Some(value) = description {
        active.description = Set(Some(value));
    }
    if let Some(value) = language {
        active.language = Set(value);
    }
    active.updated_at = Set(now());
    let updated = active.update(&current.db).await?;
    state.remember_project(&updated)?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_project(_state: State<'_, AppState>, _id: String) -> AppResult<()> {
    // El proyecto es una carpeta en disco propiedad del usuario; eliminarlo
    // desde la app borraría datos fuera del scope SQLite. Por ahora lo
    // dejamos como acción manual; un sprint posterior puede ofrecerlo con
    // confirmación explícita.
    Err(AppError::NotImplemented("delete_project"))
}

#[tauri::command]
pub async fn export_project(
    state: State<'_, AppState>,
    id: String,
    target_path: String,
) -> AppResult<String> {
    let current = state.current().await?;
    let target = PathBuf::from(target_path);
    let written = project_io_service::export_to_file(&current.db, &id, &target).await?;
    Ok(written.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn import_project(
    state: State<'_, AppState>,
    source_path: String,
    target_root_path: String,
) -> AppResult<project::Model> {
    let source = PathBuf::from(source_path);
    let target_root = PathBuf::from(target_root_path);
    let (project_model, _paths, _db) =
        project_io_service::import_from_file(&source, &target_root).await?;
    // Abrir el proyecto recién importado como el activo.
    state.open(target_root).await?;
    state.remember_project(&project_model)?;
    Ok(project_model)
}
