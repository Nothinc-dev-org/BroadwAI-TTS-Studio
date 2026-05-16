use tauri::State;

use crate::entities::{character, character_alias};
use crate::error::{AppError, AppResult};
use crate::services::character_service;
use crate::state::AppState;

#[tauri::command]
pub async fn create_character(
    state: State<'_, AppState>,
    project_id: String,
    name: String,
    role: String,
    description: Option<String>,
    color: Option<String>,
    voice_provider: Option<String>,
    voice_id: Option<String>,
    default_style_prompt: Option<String>,
) -> AppResult<character::Model> {
    let current = state.current().await?;
    character_service::create(
        &current.db,
        character_service::CreateCharacterInput {
            project_id,
            name,
            role,
            description,
            color,
            voice_provider,
            voice_id,
            default_style_prompt,
        },
    )
    .await
}

#[tauri::command]
pub async fn list_characters(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<character::Model>> {
    let current = state.current().await?;
    character_service::list(&current.db, &project_id).await
}

#[tauri::command]
pub async fn update_character(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    role: Option<String>,
    description: Option<String>,
    color: Option<String>,
) -> AppResult<()> {
    let current = state.current().await?;
    character_service::update(
        &current.db,
        &id,
        character_service::UpdateCharacterInput {
            name,
            role,
            description,
            color,
        },
    )
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_character(_state: State<'_, AppState>, _id: String) -> AppResult<()> {
    Err(AppError::NotImplemented("delete_character"))
}

#[tauri::command]
pub async fn add_character_alias(
    state: State<'_, AppState>,
    character_id: String,
    alias: String,
) -> AppResult<character_alias::Model> {
    let current = state.current().await?;
    character_service::add_alias(&current.db, &character_id, &alias).await
}

#[tauri::command]
pub async fn remove_character_alias(
    _state: State<'_, AppState>,
    _alias_id: String,
) -> AppResult<()> {
    Err(AppError::NotImplemented("remove_character_alias"))
}

#[tauri::command]
pub async fn assign_character_voice(
    state: State<'_, AppState>,
    character_id: String,
    voice_provider: String,
    voice_id: String,
    default_style_prompt: Option<String>,
) -> AppResult<()> {
    let current = state.current().await?;
    character_service::assign_voice(
        &current.db,
        &character_id,
        &voice_provider,
        &voice_id,
        default_style_prompt,
    )
    .await?;
    Ok(())
}
