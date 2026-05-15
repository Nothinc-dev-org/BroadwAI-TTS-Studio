//! CharacterService — CRUD de personajes, alias y voz (RF-06, RF-07, RF-08, RF-23).

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::entities::{character, character_alias};
use crate::error::AppResult;
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
