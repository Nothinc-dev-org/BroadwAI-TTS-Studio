use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub language: String,
    pub root_path: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::scene::Entity")]
    Scenes,
    #[sea_orm(has_many = "super::character::Entity")]
    Characters,
    #[sea_orm(has_many = "super::audio_asset::Entity")]
    AudioAssets,
    #[sea_orm(has_many = "super::raw_import::Entity")]
    RawImports,
}

impl Related<super::scene::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scenes.def()
    }
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Characters.def()
    }
}

impl Related<super::audio_asset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AudioAssets.def()
    }
}

impl Related<super::raw_import::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RawImports.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
