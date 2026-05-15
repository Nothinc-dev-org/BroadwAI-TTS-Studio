//! ProjectService — crear/abrir/actualizar/exportar proyectos (RF-01, RF-02, RF-39, RF-40).

use std::path::PathBuf;

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::entities::project;
use crate::error::AppResult;
use crate::services::{new_id, now};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectInput {
    pub title: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub root_path: PathBuf,
}

pub async fn create(db: &DatabaseConnection, input: CreateProjectInput) -> AppResult<project::Model> {
    let now_ts = now();
    let model = project::ActiveModel {
        id: Set(new_id()),
        title: Set(input.title),
        description: Set(input.description),
        language: Set(input.language.unwrap_or_else(|| "es-MX".into())),
        root_path: Set(input
            .root_path
            .to_str()
            .unwrap_or_default()
            .to_string()),
        created_at: Set(now_ts.clone()),
        updated_at: Set(now_ts),
    };
    Ok(model.insert(db).await?)
}

pub async fn list(db: &DatabaseConnection) -> AppResult<Vec<project::Model>> {
    Ok(project::Entity::find().all(db).await?)
}

pub async fn get(db: &DatabaseConnection, id: &str) -> AppResult<Option<project::Model>> {
    Ok(project::Entity::find_by_id(id.to_owned()).one(db).await?)
}
