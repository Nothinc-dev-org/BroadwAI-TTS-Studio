use tauri::State;

use crate::entities::audio_asset;
use crate::error::{AppError, AppResult};
use crate::services::asset_service;
use crate::state::AppState;

#[tauri::command]
pub async fn import_audio_asset(
    _state: State<'_, AppState>,
    _project_id: String,
    _file_path: String,
    _kind: String,
    _name: Option<String>,
) -> AppResult<audio_asset::Model> {
    Err(AppError::NotImplemented("import_audio_asset"))
}

#[tauri::command]
pub async fn list_audio_assets(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<audio_asset::Model>> {
    let current = state.current().await?;
    asset_service::list(&current.db, &project_id).await
}

#[tauri::command]
pub async fn update_audio_asset(
    _state: State<'_, AppState>,
    _id: String,
    _name: Option<String>,
    _kind: Option<String>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_audio_asset"))
}

#[tauri::command]
pub async fn delete_audio_asset(
    _state: State<'_, AppState>,
    _id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("delete_audio_asset"))
}

#[tauri::command]
pub async fn preview_audio_asset(
    _state: State<'_, AppState>,
    _id: String,
) -> AppResult<String> {
    Err(AppError::NotImplemented("preview_audio_asset"))
}
