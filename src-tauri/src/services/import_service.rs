//! ImportService — copy-paste / archivos (RF-09, RF-10, RF-16).

use std::collections::{BTreeMap, BTreeSet};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::entities::{
    character, character_alias, dialogue_node, dialogue_tts_tag, raw_import, scene, timeline_track,
};
use crate::error::{AppError, AppResult};
use crate::services::credential_service::CredentialService;
use crate::services::deepseek_service::{self, DeepSeekResult, DeepSeekService};
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

pub async fn process_with_deepseek(
    db: &DatabaseConnection,
    credentials: &CredentialService,
    raw_import_id: &str,
) -> AppResult<DeepSeekResult> {
    process_with_deepseek_progress(db, credentials, raw_import_id, |_, _| {}).await
}

pub async fn process_with_deepseek_progress<F>(
    db: &DatabaseConnection,
    credentials: &CredentialService,
    raw_import_id: &str,
    progress: F,
) -> AppResult<DeepSeekResult>
where
    F: FnMut(usize, usize),
{
    let import = get_raw_import(db, raw_import_id).await?;
    let service = DeepSeekService::new(credentials);

    match service
        .structure_script_with_progress(&import.original_text, progress)
        .await
    {
        Ok(result) => {
            let status = if result.warnings.is_empty() {
                "processed"
            } else {
                "needs_review"
            };
            update_processed_result(db, import, &result, status, None).await?;
            Ok(result)
        }
        Err(error) => {
            update_failed_import(db, import, &error.to_string()).await?;
            Err(error)
        }
    }
}

pub async fn validate_processed_result(
    db: &DatabaseConnection,
    raw_import_id: &str,
) -> AppResult<DeepSeekResult> {
    let import = get_raw_import(db, raw_import_id).await?;
    let processed_json = import
        .processed_json
        .as_deref()
        .ok_or_else(|| AppError::invalid("la importación todavía no tiene JSON procesado"))?;
    let mut result: DeepSeekResult = serde_json::from_str(processed_json)?;
    result.warnings = deepseek_service::validate_result(&import.original_text, &result.scene)?;
    let status = if result.warnings.is_empty() {
        "processed"
    } else {
        "needs_review"
    };
    update_processed_result(db, import, &result, status, None).await?;
    Ok(result)
}

pub async fn create_scene_from_import(
    db: &DatabaseConnection,
    raw_import_id: &str,
) -> AppResult<scene::Model> {
    let import = get_raw_import(db, raw_import_id).await?;
    let processed_json = import
        .processed_json
        .as_deref()
        .ok_or_else(|| AppError::invalid("la importación todavía no tiene JSON procesado"))?;
    let mut result: DeepSeekResult = serde_json::from_str(processed_json)?;
    result.warnings = deepseek_service::validate_result(&import.original_text, &result.scene)?;

    let txn = db.begin().await?;
    let now_ts = now();
    let scene_id = new_id();
    let scene_model = scene::ActiveModel {
        id: Set(scene_id.clone()),
        project_id: Set(import.project_id.clone()),
        title: Set(result.scene.title.trim().to_owned()),
        description: Set(result.scene.description.clone()),
        order_index: Set(next_scene_order_index(db, &import.project_id).await?),
        created_at: Set(now_ts.clone()),
        updated_at: Set(now_ts.clone()),
    }
    .insert(&txn)
    .await?;

    let mut characters_by_name = BTreeMap::new();
    for structured in &result.scene.characters {
        let name = structured.name.trim();
        if name.is_empty() || characters_by_name.contains_key(name) {
            continue;
        }
        let character_id = new_id();
        character::ActiveModel {
            id: Set(character_id.clone()),
            project_id: Set(import.project_id.clone()),
            name: Set(name.to_owned()),
            role: Set(normalize_character_role(&structured.role, name)),
            description: Set(structured.description.clone()),
            color: Set(None),
            voice_provider: Set(None),
            voice_id: Set(None),
            default_style_prompt: Set(None),
            created_at: Set(now_ts.clone()),
            updated_at: Set(now_ts.clone()),
        }
        .insert(&txn)
        .await?;
        characters_by_name.insert(name.to_owned(), character_id.clone());

        let mut aliases = BTreeSet::new();
        for alias in &structured.aliases {
            let alias = alias.trim();
            if alias.is_empty() || alias == name || !aliases.insert(alias.to_owned()) {
                continue;
            }
            character_alias::ActiveModel {
                id: Set(new_id()),
                character_id: Set(character_id.clone()),
                alias: Set(alias.to_owned()),
            }
            .insert(&txn)
            .await?;
        }
    }

    for dialogue in &result.scene.dialogues {
        let speaker = dialogue.speaker.trim();
        if characters_by_name.contains_key(speaker) {
            continue;
        }
        let character_id = new_id();
        character::ActiveModel {
            id: Set(character_id.clone()),
            project_id: Set(import.project_id.clone()),
            name: Set(speaker.to_owned()),
            role: Set(normalize_character_role("character", speaker)),
            description: Set(None),
            color: Set(None),
            voice_provider: Set(None),
            voice_id: Set(None),
            default_style_prompt: Set(None),
            created_at: Set(now_ts.clone()),
            updated_at: Set(now_ts.clone()),
        }
        .insert(&txn)
        .await?;
        characters_by_name.insert(speaker.to_owned(), character_id);
    }

    let node_ids: Vec<String> = result.scene.dialogues.iter().map(|_| new_id()).collect();
    for (index, dialogue) in result.scene.dialogues.iter().enumerate() {
        let speaker = dialogue.speaker.trim();
        let character_id = characters_by_name
            .get(speaker)
            .ok_or_else(|| AppError::internal("speaker validado sin personaje asociado"))?;
        dialogue_node::ActiveModel {
            id: Set(node_ids[index].clone()),
            scene_id: Set(scene_id.clone()),
            character_id: Set(character_id.clone()),
            previous_id: Set(index
                .checked_sub(1)
                .map(|previous| node_ids[previous].clone())),
            next_id: Set(node_ids.get(index + 1).cloned()),
            order_index: Set(index as i32),
            kind: Set(dialogue.kind.clone()),
            text: Set(dialogue.text.clone()),
            raw_text: Set(dialogue.original_excerpt.clone()),
            emotion: Set(None),
            intensity: Set(None),
            is_enabled: Set(1),
            before_delay_ms: Set(Some(0)),
            after_delay_ms: Set(Some(0)),
            created_at: Set(now_ts.clone()),
            updated_at: Set(now_ts.clone()),
        }
        .insert(&txn)
        .await?;

        for (tag_index, tag) in dialogue.tts_tags.iter().enumerate() {
            dialogue_tts_tag::ActiveModel {
                id: Set(new_id()),
                dialogue_node_id: Set(node_ids[index].clone()),
                tag: Set(tag.trim().to_owned()),
                position: Set("prefix".into()),
                order_index: Set(tag_index as i32),
                source: Set("ai".into()),
            }
            .insert(&txn)
            .await?;
        }
    }

    timeline_track::ActiveModel {
        id: Set(new_id()),
        scene_id: Set(scene_id.clone()),
        name: Set("Voces".into()),
        kind: Set("voice".into()),
        order_index: Set(0),
        volume: Set(1.0),
        muted: Set(0),
        solo: Set(0),
    }
    .insert(&txn)
    .await?;

    let mut active_import = import.into_active_model();
    active_import.scene_id = Set(Some(scene_id));
    active_import.status = Set("scene_created".into());
    active_import.error_message = Set(None);
    active_import.update(&txn).await?;

    txn.commit().await?;
    Ok(scene_model)
}

async fn get_raw_import(db: &DatabaseConnection, id: &str) -> AppResult<raw_import::Model> {
    raw_import::Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("raw_import {id}")))
}

async fn update_processed_result(
    db: &DatabaseConnection,
    import: raw_import::Model,
    result: &DeepSeekResult,
    status: &str,
    error_message: Option<String>,
) -> AppResult<raw_import::Model> {
    let mut active = import.into_active_model();
    active.processed_json = Set(Some(serde_json::to_string(result)?));
    active.status = Set(status.to_owned());
    active.error_message = Set(error_message);
    Ok(active.update(db).await?)
}

async fn update_failed_import(
    db: &DatabaseConnection,
    import: raw_import::Model,
    message: &str,
) -> AppResult<raw_import::Model> {
    let mut active = import.into_active_model();
    active.status = Set("failed".into());
    active.error_message = Set(Some(message.to_owned()));
    Ok(active.update(db).await?)
}

async fn next_scene_order_index(db: &DatabaseConnection, project_id: &str) -> AppResult<i32> {
    let last = scene::Entity::find()
        .filter(scene::Column::ProjectId.eq(project_id))
        .order_by_desc(scene::Column::OrderIndex)
        .one(db)
        .await?;
    Ok(last.map(|scene| scene.order_index + 1).unwrap_or(0))
}

fn normalize_character_role(role: &str, name: &str) -> String {
    match role.trim() {
        "narrator" | "character" | "system" => role.trim().to_owned(),
        _ if name.eq_ignore_ascii_case("Narrador") => "narrator".into(),
        _ => "character".into(),
    }
}
