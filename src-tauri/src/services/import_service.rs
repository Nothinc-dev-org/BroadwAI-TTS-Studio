//! ImportService — copy-paste / archivos (RF-09, RF-10, RF-16).

use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::entities::raw_import;
use crate::error::{AppError, AppResult};
use crate::services::{new_id, now};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportSourceType {
    Paste,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTextInput {
    pub project_id: String,
    pub text: String,
    pub source_file_path: Option<String>,
    pub source_type: ImportSourceType,
}

/// Persiste el texto crudo (RF-04.4 / RF-10) sin tocar al LLM todavía.
pub async fn create_raw_import(
    db: &DatabaseConnection,
    input: ImportTextInput,
) -> AppResult<raw_import::Model> {
    if input.text.trim().is_empty() {
        return Err(AppError::invalid("el texto importado está vacío"));
    }

    let source_type = match input.source_type {
        ImportSourceType::Paste => "paste",
        ImportSourceType::File => "file",
    };

    let model = raw_import::ActiveModel {
        id: Set(new_id()),
        project_id: Set(input.project_id),
        scene_id: Set(None),
        source_type: Set(source_type.into()),
        source_file_path: Set(input.source_file_path),
        original_text: Set(input.text),
        processed_json: Set(None),
        status: Set("pending".into()),
        error_message: Set(None),
        created_at: Set(now()),
    };
    Ok(model.insert(db).await?)
}

pub fn read_file(path: &std::path::Path) -> AppResult<String> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());
    match ext.as_deref() {
        Some("txt") | Some("md") => Ok(std::fs::read_to_string(path)?),
        other => Err(AppError::invalid(format!(
            "extensión de archivo no soportada: {other:?}"
        ))),
    }
}
