use tauri::State;

use crate::entities::{dialogue_node, dialogue_tts_tag};
use crate::error::{AppError, AppResult};
use crate::services::dialogue_service;
use crate::state::AppState;

#[tauri::command]
pub async fn create_dialogue_node(
    _state: State<'_, AppState>,
    _scene_id: String,
    _character_id: String,
    _kind: String,
    _text: String,
    _order_index: i32,
) -> AppResult<dialogue_node::Model> {
    Err(AppError::NotImplemented("create_dialogue_node"))
}

#[tauri::command]
pub async fn list_dialogue_nodes(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<Vec<dialogue_node::Model>> {
    let current = state.current().await?;
    dialogue_service::list_for_scene(&current.db, &scene_id).await
}

#[tauri::command]
pub async fn update_dialogue_node(
    _state: State<'_, AppState>,
    _id: String,
    _text: Option<String>,
    _character_id: Option<String>,
    _kind: Option<String>,
    _emotion: Option<String>,
    _intensity: Option<i32>,
    _is_enabled: Option<bool>,
    _before_delay_ms: Option<i32>,
    _after_delay_ms: Option<i32>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_dialogue_node"))
}

#[tauri::command]
pub async fn delete_dialogue_node(_state: State<'_, AppState>, _id: String) -> AppResult<()> {
    Err(AppError::NotImplemented("delete_dialogue_node"))
}

#[tauri::command]
pub async fn split_dialogue_node(
    _state: State<'_, AppState>,
    _id: String,
    _split_at: i32,
) -> AppResult<()> {
    Err(AppError::NotImplemented("split_dialogue_node"))
}

#[tauri::command]
pub async fn merge_dialogue_nodes(
    _state: State<'_, AppState>,
    _first_id: String,
    _second_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("merge_dialogue_nodes"))
}

#[tauri::command]
pub async fn reorder_dialogue_nodes(
    _state: State<'_, AppState>,
    _scene_id: String,
    _ordered_ids: Vec<String>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("reorder_dialogue_nodes"))
}

#[tauri::command]
pub async fn update_dialogue_tts_tags(
    _state: State<'_, AppState>,
    _dialogue_node_id: String,
    _tags: Vec<dialogue_tts_tag::Model>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_dialogue_tts_tags"))
}
