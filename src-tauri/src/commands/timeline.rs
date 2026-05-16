use std::path::PathBuf;

use tauri::State;

use crate::entities::{timeline_event, timeline_track};
use crate::error::AppResult;
use crate::services::audio_mixer::ExportFormat;
use crate::services::scene_mix_service::{self, SceneMixResult};
use crate::services::timeline_service::{self, EventUpdate};
use crate::state::AppState;

#[tauri::command]
pub async fn create_timeline_track(
    state: State<'_, AppState>,
    scene_id: String,
    name: String,
    kind: String,
) -> AppResult<timeline_track::Model> {
    let current = state.current().await?;
    timeline_service::create_track(&current.db, &scene_id, name, kind).await
}

#[tauri::command]
pub async fn list_timeline_tracks(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<Vec<timeline_track::Model>> {
    let current = state.current().await?;
    timeline_service::list_tracks(&current.db, &scene_id).await
}

#[tauri::command]
pub async fn update_timeline_track(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    volume: Option<f64>,
    muted: Option<bool>,
    solo: Option<bool>,
) -> AppResult<timeline_track::Model> {
    let current = state.current().await?;
    timeline_service::update_track(&current.db, &id, name, volume, muted, solo).await
}

#[tauri::command]
pub async fn delete_timeline_track(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let current = state.current().await?;
    timeline_service::delete_track(&current.db, &id).await
}

#[tauri::command]
pub async fn list_timeline_events(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<Vec<timeline_event::Model>> {
    let current = state.current().await?;
    timeline_service::list_events_for_scene(&current.db, &scene_id).await
}

#[tauri::command]
pub async fn create_timeline_event(
    state: State<'_, AppState>,
    scene_id: String,
    audio_asset_id: String,
    start_ms: i32,
) -> AppResult<timeline_event::Model> {
    let current = state.current().await?;
    timeline_service::create_asset_event(&current.db, &scene_id, &audio_asset_id, start_ms).await
}

#[tauri::command]
pub async fn update_timeline_event(
    state: State<'_, AppState>,
    id: String,
    update: EventUpdate,
) -> AppResult<timeline_event::Model> {
    let current = state.current().await?;
    timeline_service::update_event(&current.db, &id, update).await
}

#[tauri::command]
pub async fn delete_timeline_event(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let current = state.current().await?;
    timeline_service::delete_event(&current.db, &id).await
}

#[tauri::command]
pub async fn render_timeline(
    state: State<'_, AppState>,
    scene_id: String,
    output_path: String,
    format: String,
) -> AppResult<SceneMixResult> {
    let current = state.current().await?;
    let format = ExportFormat::parse(&format)?;
    let output = PathBuf::from(output_path);
    scene_mix_service::render_scene(
        &current.db,
        &current.paths,
        &state.credentials,
        &scene_id,
        &output,
        format,
    )
    .await
}
