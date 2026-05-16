use std::path::PathBuf;

use serde::Serialize;
use tauri::{Emitter, State, Window};

use crate::entities::raw_import;
use crate::error::AppResult;
use crate::services::deepseek_service::DeepSeekResult;
use crate::services::import_service::{self, ImportSourceType, ImportTextInput};
use crate::state::AppState;

#[derive(Clone, Serialize)]
struct DeepSeekImportProgress {
    raw_import_id: String,
    completed: usize,
    total: usize,
}

#[tauri::command]
pub async fn import_text(
    state: State<'_, AppState>,
    project_id: String,
    text: String,
) -> AppResult<raw_import::Model> {
    let current = state.current().await?;
    import_service::create_raw_import(
        &current.db,
        ImportTextInput {
            project_id,
            text,
            source_file_path: None,
            source_type: ImportSourceType::Paste,
        },
    )
    .await
}

#[tauri::command]
pub async fn import_file(
    state: State<'_, AppState>,
    project_id: String,
    file_path: String,
) -> AppResult<raw_import::Model> {
    let current = state.current().await?;
    let path = PathBuf::from(&file_path);
    let text = import_service::read_file(&path)?;
    import_service::create_raw_import(
        &current.db,
        ImportTextInput {
            project_id,
            text,
            source_file_path: Some(file_path),
            source_type: ImportSourceType::File,
        },
    )
    .await
}

#[tauri::command]
pub async fn process_import_with_deepseek(
    state: State<'_, AppState>,
    window: Window,
    raw_import_id: String,
) -> AppResult<DeepSeekResult> {
    let current = state.current().await?;
    let progress_import_id = raw_import_id.clone();
    import_service::process_with_deepseek_progress(
        &current.db,
        &state.credentials,
        &raw_import_id,
        move |completed, total| {
            let _ = window.emit(
                "import://deepseek-progress",
                DeepSeekImportProgress {
                    raw_import_id: progress_import_id.clone(),
                    completed,
                    total,
                },
            );
        },
    )
    .await
}

#[tauri::command]
pub async fn validate_import_result(
    state: State<'_, AppState>,
    raw_import_id: String,
) -> AppResult<DeepSeekResult> {
    let current = state.current().await?;
    import_service::validate_processed_result(&current.db, &raw_import_id).await
}

#[tauri::command]
pub async fn create_scene_from_import(
    state: State<'_, AppState>,
    raw_import_id: String,
) -> AppResult<crate::entities::scene::Model> {
    let current = state.current().await?;
    import_service::create_scene_from_import(&current.db, &raw_import_id).await
}
