use tauri::State;

use crate::entities::scene;
use crate::error::{AppError, AppResult};
use crate::services::scene_service;
use crate::state::AppState;

#[tauri::command]
pub async fn create_scene(
    state: State<'_, AppState>,
    project_id: String,
    title: String,
    description: Option<String>,
    order_index: Option<i32>,
) -> AppResult<scene::Model> {
    let current = state.current().await?;
    scene_service::create(
        &current.db,
        scene_service::CreateSceneInput {
            project_id,
            title,
            description,
            order_index,
        },
    )
    .await
}

#[tauri::command]
pub async fn get_scene(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Option<scene::Model>> {
    let current = state.current().await?;
    scene_service::get(&current.db, &id).await
}

#[tauri::command]
pub async fn list_scenes(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<scene::Model>> {
    let current = state.current().await?;
    scene_service::list(&current.db, &project_id).await
}

#[tauri::command]
pub async fn update_scene(
    _state: State<'_, AppState>,
    _id: String,
    _title: Option<String>,
    _description: Option<String>,
    _order_index: Option<i32>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_scene"))
}

#[tauri::command]
pub async fn delete_scene(_state: State<'_, AppState>, _id: String) -> AppResult<()> {
    Err(AppError::NotImplemented("delete_scene"))
}

#[tauri::command]
pub async fn reorder_scenes(
    _state: State<'_, AppState>,
    _project_id: String,
    _ordered_ids: Vec<String>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("reorder_scenes"))
}
