//! TimelineService — tracks y eventos del timeline (RF-34, RF-36, RF-37, RF-38).
//!
//! El timeline vive en SQLite (`timeline_tracks`, `timeline_events`).
//! El renderizado consume la combinación de los dialogue_nodes (voces) y los
//! timeline_events asociados a audio_assets/generated_audio.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::entities::{audio_asset, timeline_event, timeline_track};
use crate::error::{AppError, AppResult};
use crate::services::asset_service::AssetKind;
use crate::services::{new_id, now};

const VOICE_KIND: &str = "voice";
const SFX_KIND: &str = "sfx";
const MUSIC_KIND: &str = "music";
const AMBIENCE_KIND: &str = "ambience";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventUpdate {
    pub start_ms: Option<i32>,
    pub duration_ms: Option<i32>,
    pub offset_ms: Option<i32>,
    pub volume: Option<f64>,
    pub fade_in_ms: Option<i32>,
    pub fade_out_ms: Option<i32>,
    pub playback_rate: Option<f64>,
    pub looping: Option<bool>,
}

pub async fn list_tracks(
    db: &DatabaseConnection,
    scene_id: &str,
) -> AppResult<Vec<timeline_track::Model>> {
    Ok(timeline_track::Entity::find()
        .filter(timeline_track::Column::SceneId.eq(scene_id))
        .order_by_asc(timeline_track::Column::OrderIndex)
        .all(db)
        .await?)
}

pub async fn list_events_for_scene(
    db: &DatabaseConnection,
    scene_id: &str,
) -> AppResult<Vec<timeline_event::Model>> {
    Ok(timeline_event::Entity::find()
        .filter(timeline_event::Column::SceneId.eq(scene_id))
        .order_by_asc(timeline_event::Column::StartMs)
        .all(db)
        .await?)
}

pub async fn create_track(
    db: &DatabaseConnection,
    scene_id: &str,
    name: String,
    kind: String,
) -> AppResult<timeline_track::Model> {
    let order_index = next_order_index(db, scene_id).await?;
    let inserted = timeline_track::ActiveModel {
        id: Set(new_id()),
        scene_id: Set(scene_id.to_owned()),
        name: Set(name),
        kind: Set(kind),
        order_index: Set(order_index),
        volume: Set(1.0),
        muted: Set(0),
        solo: Set(0),
    }
    .insert(db)
    .await?;
    Ok(inserted)
}

pub async fn ensure_track_for_kind(
    db: &DatabaseConnection,
    scene_id: &str,
    track_kind: &str,
) -> AppResult<timeline_track::Model> {
    if let Some(existing) = timeline_track::Entity::find()
        .filter(timeline_track::Column::SceneId.eq(scene_id))
        .filter(timeline_track::Column::Kind.eq(track_kind))
        .one(db)
        .await?
    {
        return Ok(existing);
    }
    let name = default_track_name(track_kind);
    create_track(db, scene_id, name.to_owned(), track_kind.to_owned()).await
}

pub async fn update_track(
    db: &DatabaseConnection,
    id: &str,
    name: Option<String>,
    volume: Option<f64>,
    muted: Option<bool>,
    solo: Option<bool>,
) -> AppResult<timeline_track::Model> {
    let model = timeline_track::Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("timeline_track {id}")))?;
    let mut active = model.into_active_model();
    if let Some(name) = name {
        active.name = Set(name);
    }
    if let Some(volume) = volume {
        active.volume = Set(volume);
    }
    if let Some(muted) = muted {
        active.muted = Set(if muted { 1 } else { 0 });
    }
    if let Some(solo) = solo {
        active.solo = Set(if solo { 1 } else { 0 });
    }
    Ok(active.update(db).await?)
}

pub async fn delete_track(db: &DatabaseConnection, id: &str) -> AppResult<()> {
    timeline_track::Entity::delete_by_id(id.to_owned())
        .exec(db)
        .await?;
    Ok(())
}

pub async fn create_asset_event(
    db: &DatabaseConnection,
    scene_id: &str,
    audio_asset_id: &str,
    start_ms: i32,
) -> AppResult<timeline_event::Model> {
    let asset = audio_asset::Entity::find_by_id(audio_asset_id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("audio_asset {audio_asset_id}")))?;
    let kind = AssetKind::parse(&asset.kind)?;
    let track_kind = track_kind_for_asset(kind);
    let track = ensure_track_for_kind(db, scene_id, track_kind).await?;

    let now_ts = now();
    let inserted = timeline_event::ActiveModel {
        id: Set(new_id()),
        scene_id: Set(scene_id.to_owned()),
        track_id: Set(track.id),
        dialogue_node_id: Set(None),
        audio_asset_id: Set(Some(asset.id)),
        generated_audio_id: Set(None),
        start_ms: Set(start_ms.max(0)),
        duration_ms: Set(asset.duration_ms),
        offset_ms: Set(Some(0)),
        volume: Set(1.0),
        fade_in_ms: Set(None),
        fade_out_ms: Set(None),
        playback_rate: Set(1.0),
        looping: Set(if matches!(kind, AssetKind::Ambience) {
            1
        } else {
            0
        }),
        created_at: Set(now_ts.clone()),
        updated_at: Set(now_ts),
    }
    .insert(db)
    .await?;
    Ok(inserted)
}

pub async fn update_event(
    db: &DatabaseConnection,
    id: &str,
    update: EventUpdate,
) -> AppResult<timeline_event::Model> {
    let model = timeline_event::Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("timeline_event {id}")))?;
    let mut active = model.into_active_model();
    if let Some(value) = update.start_ms {
        active.start_ms = Set(value.max(0));
    }
    if let Some(value) = update.duration_ms {
        active.duration_ms = Set(Some(value.max(0)));
    }
    if let Some(value) = update.offset_ms {
        active.offset_ms = Set(Some(value));
    }
    if let Some(value) = update.volume {
        active.volume = Set(value);
    }
    if let Some(value) = update.fade_in_ms {
        active.fade_in_ms = Set(Some(value.max(0)));
    }
    if let Some(value) = update.fade_out_ms {
        active.fade_out_ms = Set(Some(value.max(0)));
    }
    if let Some(value) = update.playback_rate {
        active.playback_rate = Set(value.max(0.01));
    }
    if let Some(value) = update.looping {
        active.looping = Set(if value { 1 } else { 0 });
    }
    active.updated_at = Set(now());
    Ok(active.update(db).await?)
}

pub async fn delete_event(db: &DatabaseConnection, id: &str) -> AppResult<()> {
    timeline_event::Entity::delete_by_id(id.to_owned())
        .exec(db)
        .await?;
    Ok(())
}

async fn next_order_index(db: &DatabaseConnection, scene_id: &str) -> AppResult<i32> {
    let last = timeline_track::Entity::find()
        .filter(timeline_track::Column::SceneId.eq(scene_id))
        .order_by_desc(timeline_track::Column::OrderIndex)
        .one(db)
        .await?;
    Ok(last.map(|t| t.order_index + 1).unwrap_or(0))
}

fn track_kind_for_asset(kind: AssetKind) -> &'static str {
    match kind {
        AssetKind::SoundEffect => SFX_KIND,
        AssetKind::Music => MUSIC_KIND,
        AssetKind::Ambience => AMBIENCE_KIND,
        AssetKind::Voice | AssetKind::Generated => VOICE_KIND,
    }
}

fn default_track_name(kind: &str) -> &'static str {
    match kind {
        VOICE_KIND => "Voces",
        SFX_KIND => "SFX",
        MUSIC_KIND => "Música",
        AMBIENCE_KIND => "Ambiente",
        _ => "Pista",
    }
}
