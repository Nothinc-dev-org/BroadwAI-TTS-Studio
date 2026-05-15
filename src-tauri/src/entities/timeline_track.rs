use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "timeline_tracks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub scene_id: String,
    pub name: String,
    #[sea_orm(column_name = "type")]
    pub kind: String,
    pub order_index: i32,
    pub volume: f64,
    pub muted: i32,
    pub solo: i32,
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
    #[sea_orm(has_many = "super::timeline_event::Entity")]
    Events,
}

impl Related<super::scene::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scene.def()
    }
}

impl Related<super::timeline_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Events.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
