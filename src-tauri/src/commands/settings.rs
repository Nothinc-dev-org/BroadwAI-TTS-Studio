use std::collections::HashMap;

use tauri::State;

use crate::error::{AppError, AppResult};
use crate::services::credential_service::{ApiKeyStatus, Provider};
use crate::state::AppState;

#[tauri::command]
pub async fn set_api_key(
    state: State<'_, AppState>,
    provider: Provider,
    key: String,
) -> AppResult<ApiKeyStatus> {
    state.credentials.set(provider, &key)?;
    // Tras guardar no devolvemos la key, solo el estado.
    Ok(ApiKeyStatus::Configured)
}

#[tauri::command]
pub async fn delete_api_key(
    state: State<'_, AppState>,
    provider: Provider,
) -> AppResult<()> {
    state.credentials.delete(provider)
}

#[tauri::command]
pub async fn test_api_key(
    _state: State<'_, AppState>,
    _provider: Provider,
) -> AppResult<ApiKeyStatus> {
    // El test real se implementará junto con DeepSeekService / GeminiTtsService.
    Err(AppError::NotImplemented("test_api_key"))
}

#[tauri::command]
pub async fn get_api_key_status(
    state: State<'_, AppState>,
    provider: Provider,
) -> AppResult<ApiKeyStatus> {
    state.credentials.status(provider)
}

#[tauri::command]
pub async fn get_app_settings(
    _state: State<'_, AppState>,
) -> AppResult<HashMap<String, String>> {
    Ok(HashMap::new())
}

#[tauri::command]
pub async fn update_app_settings(
    _state: State<'_, AppState>,
    _settings: HashMap<String, String>,
) -> AppResult<()> {
    Err(AppError::NotImplemented("update_app_settings"))
}
