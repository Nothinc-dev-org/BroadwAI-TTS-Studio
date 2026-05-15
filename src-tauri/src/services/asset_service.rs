//! AssetService — biblioteca de assets de audio (RF-32, RF-33, RF-34).

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::audio_asset;
use crate::error::AppResult;

pub async fn list(
    db: &DatabaseConnection,
    project_id: &str,
) -> AppResult<Vec<audio_asset::Model>> {
    Ok(audio_asset::Entity::find()
        .filter(audio_asset::Column::ProjectId.eq(project_id))
        .all(db)
        .await?)
}
