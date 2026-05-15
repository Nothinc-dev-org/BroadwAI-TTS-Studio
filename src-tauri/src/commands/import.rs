use std::path::PathBuf;

use tauri::State;

use crate::entities::raw_import;
use crate::error::{AppError, AppResult};
use crate::services::import_service::{self, ImportSourceType, ImportTextInput};
use crate::state::AppState;

#[tauri::command]
pub async fn import_text(
    state: State<'_, AppState>,
    project_id: String,
    text: String,
) -> AppResult<raw_import::Model> {
    let current = state.current().await?;
    import_service::create_raw_import(
        &current.db,
        ImportTextInput {
            project_id,
            text,
            source_file_path: None,
            source_type: ImportSourceType::Paste,
        },
    )
    .await
}

#[tauri::command]
pub async fn import_file(
    state: State<'_, AppState>,
    project_id: String,
    file_path: String,
) -> AppResult<raw_import::Model> {
    let current = state.current().await?;
    let path = PathBuf::from(&file_path);
    let text = import_service::read_file(&path)?;
    import_service::create_raw_import(
        &current.db,
        ImportTextInput {
            project_id,
            text,
            source_file_path: Some(file_path),
            source_type: ImportSourceType::File,
        },
    )
    .await
}

#[tauri::command]
pub async fn process_import_with_deepseek(
    _state: State<'_, AppState>,
    _raw_import_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("process_import_with_deepseek"))
}

#[tauri::command]
pub async fn validate_import_result(
    _state: State<'_, AppState>,
    _raw_import_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("validate_import_result"))
}

#[tauri::command]
pub async fn create_scene_from_import(
    _state: State<'_, AppState>,
    _raw_import_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("create_scene_from_import"))
}
