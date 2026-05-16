//! TtsOptimizationService — RF-22.
//!
//! Reemplaza los `dialogue_tts_tags` de los nodos de una escena por las
//! sugerencias de DeepSeek. Nunca toca `text`, `speaker` ni `order_index`.
//! Marca como `outdated` cualquier `generated_audio` de un nodo modificado,
//! porque el `tag_signature` cambia y por tanto el `input_hash` del caché
//! deja de ser válido (RF-30).

use std::collections::{HashMap, HashSet};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};

use crate::entities::{character, dialogue_node, dialogue_tts_tag, generated_audio};
use crate::error::{AppError, AppResult};
use crate::services::credential_service::CredentialService;
use crate::services::deepseek_service::{DeepSeekService, OptimizationInputBlock, TagsUpdate};
use crate::services::{new_id, now};

pub async fn optimize_scene_tags(
    db: &DatabaseConnection,
    credentials: &CredentialService,
    scene_id: &str,
) -> AppResult<Vec<TagsUpdate>> {
    let nodes = dialogue_node::Entity::find()
        .filter(dialogue_node::Column::SceneId.eq(scene_id))
        .filter(dialogue_node::Column::IsEnabled.eq(1))
        .order_by_asc(dialogue_node::Column::OrderIndex)
        .all(db)
        .await?;
    if nodes.is_empty() {
        return Err(AppError::invalid(
            "la escena no tiene diálogos para optimizar",
        ));
    }

    let character_ids: HashSet<String> = nodes.iter().map(|n| n.character_id.clone()).collect();
    let characters: HashMap<String, String> = character::Entity::find()
        .filter(character::Column::Id.is_in(character_ids.into_iter().collect::<Vec<_>>()))
        .all(db)
        .await?
        .into_iter()
        .map(|c| (c.id, c.name))
        .collect();

    let node_ids: Vec<String> = nodes.iter().map(|n| n.id.clone()).collect();
    let tags_by_node = load_tags_grouped(db, &node_ids).await?;

    let blocks: Vec<OptimizationInputBlock> = nodes
        .iter()
        .map(|node| OptimizationInputBlock {
            id: node.id.clone(),
            speaker: characters
                .get(&node.character_id)
                .cloned()
                .unwrap_or_else(|| "Desconocido".into()),
            kind: node.kind.clone(),
            text: node.text.clone(),
            tags: tags_by_node.get(&node.id).cloned().unwrap_or_default(),
        })
        .collect();

    let service = DeepSeekService::new(credentials);
    let updates = service.optimize_tts_tags(&blocks).await?;
    if updates.is_empty() {
        return Ok(updates);
    }

    let txn = db.begin().await?;
    let now_ts = now();
    for update in &updates {
        dialogue_tts_tag::Entity::delete_many()
            .filter(dialogue_tts_tag::Column::DialogueNodeId.eq(&update.id))
            .exec(&txn)
            .await?;
        for (index, tag) in update.tags.iter().enumerate() {
            let trimmed = tag.trim();
            if trimmed.is_empty() {
                continue;
            }
            dialogue_tts_tag::ActiveModel {
                id: Set(new_id()),
                dialogue_node_id: Set(update.id.clone()),
                tag: Set(trimmed.to_owned()),
                position: Set("prefix".into()),
                order_index: Set(index as i32),
                source: Set("ai".into()),
            }
            .insert(&txn)
            .await?;
        }
        invalidate_cached_audio(&txn, &update.id, &now_ts).await?;
    }
    txn.commit().await?;
    Ok(updates)
}

async fn load_tags_grouped(
    db: &DatabaseConnection,
    node_ids: &[String],
) -> AppResult<HashMap<String, Vec<String>>> {
    let rows = dialogue_tts_tag::Entity::find()
        .filter(dialogue_tts_tag::Column::DialogueNodeId.is_in(node_ids.to_vec()))
        .order_by_asc(dialogue_tts_tag::Column::OrderIndex)
        .all(db)
        .await?;
    let mut grouped: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        grouped
            .entry(row.dialogue_node_id)
            .or_default()
            .push(row.tag);
    }
    Ok(grouped)
}

async fn invalidate_cached_audio<C>(txn: &C, dialogue_node_id: &str, now_ts: &str) -> AppResult<()>
where
    C: sea_orm::ConnectionTrait,
{
    let cached = generated_audio::Entity::find()
        .filter(generated_audio::Column::DialogueNodeId.eq(dialogue_node_id))
        .filter(generated_audio::Column::Status.eq("generated"))
        .all(txn)
        .await?;
    for audio in cached {
        let mut active = audio.into_active_model();
        active.status = Set("outdated".into());
        active.error_message = Set(Some(format!("tags actualizados {now_ts}")));
        active.update(txn).await?;
    }
    Ok(())
}
