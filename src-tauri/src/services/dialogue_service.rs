//! DialogueService — CRUD, lista enlazada, split/merge, tags (RF-17 a RF-21).

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::{dialogue_node, dialogue_tts_tag};
use crate::error::{AppError, AppResult};
use crate::services::now;

pub async fn list_for_scene(
    db: &DatabaseConnection,
    scene_id: &str,
) -> AppResult<Vec<dialogue_node::Model>> {
    Ok(dialogue_node::Entity::find()
        .filter(dialogue_node::Column::SceneId.eq(scene_id))
        .order_by_asc(dialogue_node::Column::OrderIndex)
        .all(db)
        .await?)
}

pub async fn list_tags_for_node(
    db: &DatabaseConnection,
    node_id: &str,
) -> AppResult<Vec<dialogue_tts_tag::Model>> {
    Ok(dialogue_tts_tag::Entity::find()
        .filter(dialogue_tts_tag::Column::DialogueNodeId.eq(node_id))
        .order_by_asc(dialogue_tts_tag::Column::OrderIndex)
        .all(db)
        .await?)
}

pub async fn update(
    db: &DatabaseConnection,
    id: &str,
    text: Option<String>,
    character_id: Option<String>,
    kind: Option<String>,
    emotion: Option<String>,
    intensity: Option<i32>,
    is_enabled: Option<bool>,
    before_delay_ms: Option<i32>,
    after_delay_ms: Option<i32>,
) -> AppResult<dialogue_node::Model> {
    let model = dialogue_node::Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("dialogue_node {id}")))?;

    let mut active = model.into_active_model();

    if let Some(v) = text {
        active.text = Set(v);
    }
    if let Some(v) = character_id {
        active.character_id = Set(v);
    }
    if let Some(v) = kind {
        active.kind = Set(v);
    }
    if let Some(v) = emotion {
        active.emotion = Set(Some(v));
    }
    if let Some(v) = intensity {
        active.intensity = Set(Some(v));
    }
    if let Some(v) = is_enabled {
        active.is_enabled = Set(if v { 1 } else { 0 });
    }
    if let Some(v) = before_delay_ms {
        active.before_delay_ms = Set(Some(v));
    }
    if let Some(v) = after_delay_ms {
        active.after_delay_ms = Set(Some(v));
    }

    active.updated_at = Set(now());
    Ok(active.update(db).await?)
}
