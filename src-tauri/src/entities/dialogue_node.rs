use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dialogue_nodes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub scene_id: String,
    pub character_id: String,
    pub previous_id: Option<String>,
    pub next_id: Option<String>,
    pub order_index: i32,
    #[sea_orm(column_name = "type")]
    pub kind: String,
    pub text: String,
    pub raw_text: Option<String>,
    pub emotion: Option<String>,
    pub intensity: Option<i32>,
    pub is_enabled: i32,
    pub before_delay_ms: Option<i32>,
    pub after_delay_ms: Option<i32>,
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
        belongs_to = "super::character::Entity",
        from = "Column::CharacterId",
        to = "super::character::Column::Id",
        on_delete = "Restrict"
    )]
    Character,
    #[sea_orm(has_many = "super::dialogue_tts_tag::Entity")]
    TtsTags,
    #[sea_orm(has_many = "super::generated_audio::Entity")]
    GeneratedAudio,
}

impl Related<super::scene::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scene.def()
    }
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

impl Related<super::dialogue_tts_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TtsTags.def()
    }
}

impl Related<super::generated_audio::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GeneratedAudio.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
