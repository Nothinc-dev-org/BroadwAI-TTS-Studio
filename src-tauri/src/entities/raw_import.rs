use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "raw_imports")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub scene_id: Option<String>,
    pub source_type: String,
    pub source_file_path: Option<String>,
    pub original_text: String,
    pub processed_json: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id",
        on_delete = "Cascade"
    )]
    Project,
    #[sea_orm(
        belongs_to = "super::scene::Entity",
        from = "Column::SceneId",
        to = "super::scene::Column::Id",
        on_delete = "SetNull"
    )]
    Scene,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::scene::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scene.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
