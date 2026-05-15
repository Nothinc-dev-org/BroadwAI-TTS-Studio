use tauri::State;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

#[tauri::command]
pub async fn create_timeline_track(
    _state: State<'_, AppState>,
    _scene_id: String,
    _name: String,
    _kind: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("create_timeline_track"))
}

#[tauri::command]
pub async fn update_timeline_track(
    _state: State<'_, AppState>,
    _id: String,
    _name: Option<String>,
    _volume: Option<f64>,
    _muted: Option<bool>,
    _solo: Option<bool>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_timeline_track"))
}

#[tauri::command]
pub async fn delete_timeline_track(
    _state: State<'_, AppState>,
    _id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("delete_timeline_track"))
}

#[tauri::command]
pub async fn create_timeline_event(
    _state: State<'_, AppState>,
    _scene_id: String,
    _track_id: String,
    _start_ms: i32,
) -> AppResult<()> {
    Err(AppError::NotImplemented("create_timeline_event"))
}

#[tauri::command]
pub async fn update_timeline_event(
    _state: State<'_, AppState>,
    _id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_timeline_event"))
}

#[tauri::command]
pub async fn delete_timeline_event(
    _state: State<'_, AppState>,
    _id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("delete_timeline_event"))
}

#[tauri::command]
pub async fn render_timeline(
    _state: State<'_, AppState>,
    _scene_id: String,
    _output_path: String,
    _format: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("render_timeline"))
}
