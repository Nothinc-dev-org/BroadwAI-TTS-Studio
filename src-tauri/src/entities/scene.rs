use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "scenes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
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
    #[sea_orm(has_many = "super::dialogue_node::Entity")]
    DialogueNodes,
    #[sea_orm(has_many = "super::timeline_track::Entity")]
    TimelineTracks,
    #[sea_orm(has_many = "super::timeline_event::Entity")]
    TimelineEvents,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::dialogue_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DialogueNodes.def()
    }
}

impl Related<super::timeline_track::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimelineTracks.def()
    }
}

impl Related<super::timeline_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimelineEvents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
