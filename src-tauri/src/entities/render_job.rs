use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "render_jobs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub scene_id: Option<String>,
    pub dialogue_node_id: Option<String>,
    #[sea_orm(column_name = "type")]
    pub kind: String,
    pub provider: String,
    pub model: Option<String>,
    pub status: String,
    pub input_payload: String,
    pub output_path: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::scene::Entity",
        from = "Column::SceneId",
        to = "super::scene::Column::Id",
        on_delete = "Cascade"
    )]
    Scene,
    #[sea_orm(
        belongs_to = "super::dialogue_node::Entity",
        from = "Column::DialogueNodeId",
        to = "super::dialogue_node::Column::Id",
        on_delete = "Cascade"
    )]
    DialogueNode,
}

impl ActiveModelBehavior for ActiveModel {}
