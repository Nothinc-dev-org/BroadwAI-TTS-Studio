//! CharacterService — CRUD de personajes, alias y voz (RF-06, RF-07, RF-08, RF-23).

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

use crate::entities::{character, character_alias, dialogue_node, generated_audio};
use crate::error::{AppError, AppResult};
use crate::services::{new_id, now};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCharacterInput {
    pub project_id: String,
    pub name: String,
    pub role: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub voice_provider: Option<String>,
    pub voice_id: Option<String>,
    pub default_style_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCharacterInput {
    pub name: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

pub async fn create(
    db: &DatabaseConnection,
    input: CreateCharacterInput,
) -> AppResult<character::Model> {
    let now_ts = now();
    let model = character::ActiveModel {
        id: Set(new_id()),
        project_id: Set(input.project_id),
        name: Set(input.name),
        role: Set(input.role),
        description: Set(input.description),
        color: Set(input.color),
        voice_provider: Set(input.voice_provider),
        voice_id: Set(input.voice_id),
        default_style_prompt: Set(input.default_style_prompt),
        created_at: Set(now_ts.clone()),
        updated_at: Set(now_ts),
    };
    Ok(model.insert(db).await?)
}

pub async fn list(db: &DatabaseConnection, project_id: &str) -> AppResult<Vec<character::Model>> {
    Ok(character::Entity::find()
        .filter(character::Column::ProjectId.eq(project_id))
        .all(db)
        .await?)
}

pub async fn update(
    db: &DatabaseConnection,
    id: &str,
    input: UpdateCharacterInput,
) -> AppResult<character::Model> {
    let model = character::Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("character {id}")))?;
    let mut active = model.into_active_model();
    if let Some(name) = input.name {
        active.name = Set(name);
    }
    if let Some(role) = input.role {
        active.role = Set(role);
    }
    if let Some(description) = input.description {
        active.description = Set(Some(description));
    }
    if let Some(color) = input.color {
        active.color = Set(Some(color));
    }
    active.updated_at = Set(now());
    Ok(active.update(db).await?)
}

pub async fn assign_voice(
    db: &DatabaseConnection,
    character_id: &str,
    voice_provider: &str,
    voice_id: &str,
    default_style_prompt: Option<String>,
) -> AppResult<character::Model> {
    if voice_provider.trim().is_empty() {
        return Err(AppError::invalid("el proveedor de voz es obligatorio"));
    }
    if voice_id.trim().is_empty() {
        return Err(AppError::invalid("el voice_id es obligatorio"));
    }

    let model = character::Entity::find_by_id(character_id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("character {character_id}")))?;
    let mut active = model.into_active_model();
    active.voice_provider = Set(Some(voice_provider.trim().to_owned()));
    active.voice_id = Set(Some(voice_id.trim().to_owned()));
    active.default_style_prompt = Set(default_style_prompt);
    active.updated_at = Set(now());
    let updated = active.update(db).await?;
    mark_character_audio_outdated(db, character_id).await?;
    Ok(updated)
}

pub async fn add_alias(
    db: &DatabaseConnection,
    character_id: &str,
    alias: &str,
) -> AppResult<character_alias::Model> {
    let model = character_alias::ActiveModel {
        id: Set(new_id()),
        character_id: Set(character_id.to_owned()),
        alias: Set(alias.to_owned()),
    };
    Ok(model.insert(db).await?)
}

pub async fn list_aliases(
    db: &DatabaseConnection,
    character_id: &str,
) -> AppResult<Vec<character_alias::Model>> {
    Ok(character_alias::Entity::find()
        .filter(character_alias::Column::CharacterId.eq(character_id))
        .all(db)
        .await?)
}

async fn mark_character_audio_outdated(
    db: &DatabaseConnection,
    character_id: &str,
) -> AppResult<()> {
    let nodes = dialogue_node::Entity::find()
        .filter(dialogue_node::Column::CharacterId.eq(character_id))
        .all(db)
        .await?;

    for node in nodes {
        let audios = generated_audio::Entity::find()
            .filter(generated_audio::Column::DialogueNodeId.eq(node.id))
            .filter(generated_audio::Column::Status.eq("generated"))
            .all(db)
            .await?;

        for audio in audios {
            let mut active = audio.into_active_model();
            active.status = Set("outdated".to_owned());
            active.update(db).await?;
        }
    }

    Ok(())
}
