//! DialogueService — CRUD, lista enlazada, split/merge, tags (RF-17 a RF-21).

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::entities::{dialogue_node, dialogue_tts_tag};
use crate::error::AppResult;

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
