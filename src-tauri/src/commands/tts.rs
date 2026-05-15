use tauri::State;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

#[tauri::command]
pub async fn generate_dialogue_audio(
    _state: State<'_, AppState>,
    _dialogue_node_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("generate_dialogue_audio"))
}

#[tauri::command]
pub async fn generate_scene_audio(
    _state: State<'_, AppState>,
    _scene_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("generate_scene_audio"))
}

#[tauri::command]
pub async fn regenerate_outdated_audio(
    _state: State<'_, AppState>,
    _scene_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("regenerate_outdated_audio"))
}

#[tauri::command]
pub async fn play_dialogue_audio(
    _state: State<'_, AppState>,
    _dialogue_node_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("play_dialogue_audio"))
}

#[tauri::command]
pub async fn play_scene_audio(
    _state: State<'_, AppState>,
    _scene_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("play_scene_audio"))
}
