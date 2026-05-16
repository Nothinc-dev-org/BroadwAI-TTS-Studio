use std::path::PathBuf;

use tauri::State;

use crate::entities::audio_asset;
use crate::error::AppResult;
use crate::services::asset_service::{self, AssetKind};
use crate::state::AppState;

#[tauri::command]
pub async fn import_audio_asset(
    state: State<'_, AppState>,
    project_id: String,
    file_path: String,
    kind: String,
    name: Option<String>,
) -> AppResult<audio_asset::Model> {
    let current = state.current().await?;
    let parsed = AssetKind::parse(&kind)?;
    asset_service::import_from_file(
        &current.db,
        &current.paths,
        &project_id,
        &PathBuf::from(file_path),
        parsed,
        name,
    )
    .await
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
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    kind: Option<String>,
) -> AppResult<audio_asset::Model> {
    let current = state.current().await?;
    let parsed = kind.map(|k| AssetKind::parse(&k)).transpose()?;
    asset_service::rename(&current.db, &id, name, parsed).await
}

#[tauri::command]
pub async fn delete_audio_asset(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let current = state.current().await?;
    asset_service::delete(&current.db, &id).await
}

#[tauri::command]
pub async fn preview_audio_asset(state: State<'_, AppState>, id: String) -> AppResult<String> {
    let current = state.current().await?;
    let asset = asset_service::get(&current.db, &id).await?;
    Ok(asset.file_path)
}
