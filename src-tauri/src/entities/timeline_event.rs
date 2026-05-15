use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "timeline_events")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub scene_id: String,
    pub track_id: String,
    pub dialogue_node_id: Option<String>,
    pub audio_asset_id: Option<String>,
    pub generated_audio_id: Option<String>,
    pub start_ms: i32,
    pub duration_ms: Option<i32>,
    pub offset_ms: Option<i32>,
    pub volume: f64,
    pub fade_in_ms: Option<i32>,
    pub fade_out_ms: Option<i32>,
    pub playback_rate: f64,
    #[sea_orm(column_name = "loop")]
    pub looping: i32,
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
        belongs_to = "super::timeline_track::Entity",
        from = "Column::TrackId",
        to = "super::timeline_track::Column::Id",
        on_delete = "Cascade"
    )]
    Track,
    #[sea_orm(
        belongs_to = "super::dialogue_node::Entity",
        from = "Column::DialogueNodeId",
        to = "super::dialogue_node::Column::Id",
        on_delete = "SetNull"
    )]
    DialogueNode,
    #[sea_orm(
        belongs_to = "super::audio_asset::Entity",
        from = "Column::AudioAssetId",
        to = "super::audio_asset::Column::Id",
        on_delete = "SetNull"
    )]
    AudioAsset,
    #[sea_orm(
        belongs_to = "super::generated_audio::Entity",
        from = "Column::GeneratedAudioId",
        to = "super::generated_audio::Column::Id",
        on_delete = "SetNull"
    )]
    GeneratedAudio,
}

impl Related<super::scene::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scene.def()
    }
}

impl Related<super::timeline_track::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Track.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
