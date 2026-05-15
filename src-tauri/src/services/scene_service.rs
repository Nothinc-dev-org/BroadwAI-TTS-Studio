//! SceneService — CRUD de escenas (RF-04, RF-05).

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};

use crate::entities::scene;
use crate::error::AppResult;
use crate::services::{new_id, now};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSceneInput {
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub order_index: Option<i32>,
}

pub async fn create(db: &DatabaseConnection, input: CreateSceneInput) -> AppResult<scene::Model> {
    let now_ts = now();
    let model = scene::ActiveModel {
        id: Set(new_id()),
        project_id: Set(input.project_id),
        title: Set(input.title),
        description: Set(input.description),
        order_index: Set(input.order_index.unwrap_or(0)),
        created_at: Set(now_ts.clone()),
        updated_at: Set(now_ts),
    };
    Ok(model.insert(db).await?)
}

pub async fn list(db: &DatabaseConnection, project_id: &str) -> AppResult<Vec<scene::Model>> {
    Ok(scene::Entity::find()
        .filter(scene::Column::ProjectId.eq(project_id))
        .order_by_asc(scene::Column::OrderIndex)
        .all(db)
        .await?)
}

pub async fn get(db: &DatabaseConnection, id: &str) -> AppResult<Option<scene::Model>> {
    Ok(scene::Entity::find_by_id(id.to_owned()).one(db).await?)
}
