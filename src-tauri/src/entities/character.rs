use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "characters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub role: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub voice_provider: Option<String>,
    pub voice_id: Option<String>,
    pub default_style_prompt: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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
    #[sea_orm(has_many = "super::character_alias::Entity")]
    Aliases,
    #[sea_orm(has_many = "super::dialogue_node::Entity")]
    DialogueNodes,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::character_alias::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Aliases.def()
    }
}

impl Related<super::dialogue_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DialogueNodes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
