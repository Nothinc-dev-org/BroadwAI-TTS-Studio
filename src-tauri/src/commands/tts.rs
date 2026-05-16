use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use tauri::State;

use crate::entities::{dialogue_node, generated_audio};
use crate::error::{AppError, AppResult};
use crate::services::audio_mixer::ExportFormat;
use crate::services::deepseek_service::TagsUpdate;
use crate::services::scene_mix_service::{self, SceneMixResult};
use crate::services::{tts_optimization_service, tts_service};
use crate::state::AppState;

#[tauri::command]
pub async fn generate_dialogue_audio(
    state: State<'_, AppState>,
    dialogue_node_id: String,
) -> AppResult<generated_audio::Model> {
    let current = state.current().await?;
    tts_service::synthesize_dialogue(
        &current.db,
        &current.paths,
        &state.credentials,
        &dialogue_node_id,
        true,
    )
    .await
}

#[tauri::command]
pub async fn play_dialogue_audio(
    state: State<'_, AppState>,
    dialogue_node_id: String,
) -> AppResult<generated_audio::Model> {
    let current = state.current().await?;
    tts_service::synthesize_dialogue(
        &current.db,
        &current.paths,
        &state.credentials,
        &dialogue_node_id,
        false,
    )
    .await
}

#[tauri::command]
pub async fn regenerate_outdated_audio(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<Vec<generated_audio::Model>> {
    let current = state.current().await?;
    tts_service::regenerate_outdated_in_scene(
        &current.db,
        &current.paths,
        &state.credentials,
        &scene_id,
    )
    .await
}

#[tauri::command]
pub async fn list_generated_audio_for_scene(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<Vec<generated_audio::Model>> {
    let current = state.current().await?;
    tts_service::list_for_scene(&current.db, &scene_id).await
}

#[tauri::command]
pub async fn generated_audio_bytes(
    state: State<'_, AppState>,
    generated_audio_id: String,
) -> AppResult<Vec<u8>> {
    let current = state.current().await?;
    let audio = generated_audio::Entity::find_by_id(generated_audio_id.clone())
        .one(&current.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("generated_audio {generated_audio_id}")))?;
    Ok(std::fs::read(audio.file_path)?)
}

#[tauri::command]
pub async fn preview_voice_sample(
    state: State<'_, AppState>,
    voice_provider: String,
    voice_id: String,
    sample_text: String,
) -> AppResult<String> {
    let current = state.current().await?;
    tts_service::synthesize_voice_sample(
        &current.paths,
        &state.credentials,
        &voice_provider,
        &voice_id,
        &sample_text,
    )
    .await
}

#[tauri::command]
pub async fn preview_voice_sample_bytes(
    state: State<'_, AppState>,
    voice_provider: String,
    voice_id: String,
    sample_text: String,
) -> AppResult<Vec<u8>> {
    let current = state.current().await?;
    let path = tts_service::synthesize_voice_sample(
        &current.paths,
        &state.credentials,
        &voice_provider,
        &voice_id,
        &sample_text,
    )
    .await?;
    Ok(std::fs::read(path)?)
}

#[tauri::command]
pub async fn generate_scene_audio(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<Vec<generated_audio::Model>> {
    let current = state.current().await?;
    let nodes = dialogue_node::Entity::find()
        .filter(dialogue_node::Column::SceneId.eq(&scene_id))
        .filter(dialogue_node::Column::IsEnabled.eq(1))
        .order_by_asc(dialogue_node::Column::OrderIndex)
        .all(&current.db)
        .await?;
    let mut results = Vec::with_capacity(nodes.len());
    for node in nodes {
        let audio = tts_service::synthesize_dialogue(
            &current.db,
            &current.paths,
            &state.credentials,
            &node.id,
            false,
        )
        .await?;
        results.push(audio);
    }
    Ok(results)
}

#[tauri::command]
pub async fn optimize_scene_tts_tags(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<Vec<TagsUpdate>> {
    let current = state.current().await?;
    tts_optimization_service::optimize_scene_tags(&current.db, &state.credentials, &scene_id).await
}

#[tauri::command]
pub async fn play_scene_audio(
    state: State<'_, AppState>,
    scene_id: String,
) -> AppResult<SceneMixResult> {
    let current = state.current().await?;
    let output = scene_mix_service::default_export_path(&current.paths, &scene_id);
    scene_mix_service::render_scene(
        &current.db,
        &current.paths,
        &state.credentials,
        &scene_id,
        &output,
        ExportFormat::Wav,
    )
    .await
}
