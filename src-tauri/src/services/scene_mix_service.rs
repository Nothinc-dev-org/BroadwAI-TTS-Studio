//! SceneMixService — RF-26, RF-27, RF-28, RF-34, RF-36, RF-37, RF-38.
//!
//! Orquesta:
//!   1. Asegura que cada dialogue_node enabled tenga audio TTS vigente.
//!   2. Resuelve los timeline_events (assets/generated_audio) por pista.
//!   3. Construye el `MixRequest` (voces + N pistas de assets) y delega al
//!      mixer.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::entities::{
    audio_asset, dialogue_node, generated_audio, timeline_event, timeline_track,
};
use crate::error::AppResult;
use crate::paths::ProjectPaths;
use crate::services::audio_mixer::{
    self, AssetClip, AssetTrack, DialogueClip, ExportFormat, MixRequest, VoiceTrack,
};
use crate::services::credential_service::CredentialService;
use crate::services::timeline_service;
use crate::services::tts_service;

const DEFAULT_DIALOGUE_GAP_MS: i32 = 250;
const VOICE_TRACK_KIND: &str = "voice";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMixResult {
    pub output_path: String,
    pub duration_ms: u64,
}

pub async fn render_scene(
    db: &DatabaseConnection,
    paths: &ProjectPaths,
    credentials: &CredentialService,
    scene_id: &str,
    output: &Path,
    format: ExportFormat,
) -> AppResult<SceneMixResult> {
    // --- 1. Voces -----------------------------------------------------------
    let nodes = dialogue_node::Entity::find()
        .filter(dialogue_node::Column::SceneId.eq(scene_id))
        .filter(dialogue_node::Column::IsEnabled.eq(1))
        .order_by_asc(dialogue_node::Column::OrderIndex)
        .all(db)
        .await?;

    let mut voice_paths: Vec<PathBuf> = Vec::with_capacity(nodes.len());
    for node in &nodes {
        let audio =
            tts_service::synthesize_dialogue(db, paths, credentials, &node.id, false).await?;
        voice_paths.push(PathBuf::from(audio.file_path));
    }
    let voice_clips: Vec<DialogueClip<'_>> = nodes
        .iter()
        .zip(voice_paths.iter())
        .map(|(node, path)| DialogueClip {
            source_path: path.as_path(),
            before_delay_ms: node.before_delay_ms.unwrap_or(0),
            after_delay_ms: node.after_delay_ms.unwrap_or(0),
            volume: 1.0,
            fade_in_ms: 0,
            fade_out_ms: 0,
        })
        .collect();

    // --- 2. Eventos de timeline (assets / generated_audio) -----------------
    let tracks = timeline_service::list_tracks(db, scene_id).await?;
    let events = timeline_service::list_events_for_scene(db, scene_id).await?;
    let asset_paths = resolve_event_paths(db, &events).await?;

    let voice_track_model = tracks.iter().find(|t| t.kind == VOICE_TRACK_KIND);
    let voice_track = if voice_clips.is_empty() {
        None
    } else {
        Some(VoiceTrack {
            clips: &voice_clips,
            default_gap_ms: DEFAULT_DIALOGUE_GAP_MS,
            volume: voice_track_model.map(|t| t.volume as f32).unwrap_or(1.0),
            muted: voice_track_model.map(|t| t.muted != 0).unwrap_or(false),
            solo: voice_track_model.map(|t| t.solo != 0).unwrap_or(false),
        })
    };

    // Mantenemos los Vec<AssetClip> vivos durante la mezcla:
    let mut asset_track_clips: Vec<Vec<AssetClip<'_>>> = Vec::with_capacity(tracks.len());
    let mut asset_tracks_meta: Vec<&timeline_track::Model> = Vec::with_capacity(tracks.len());
    for track in tracks.iter().filter(|t| t.kind != VOICE_TRACK_KIND) {
        let mut clips: Vec<AssetClip<'_>> = Vec::new();
        for event in events.iter().filter(|e| e.track_id == track.id) {
            let Some(path) = asset_paths.get(&event.id) else {
                continue;
            };
            clips.push(AssetClip {
                source_path: path.as_path(),
                start_ms: event.start_ms,
                volume: event.volume as f32,
                fade_in_ms: event.fade_in_ms.unwrap_or(0),
                fade_out_ms: event.fade_out_ms.unwrap_or(0),
                looping: event.looping != 0,
                duration_ms: event.duration_ms,
            });
        }
        asset_tracks_meta.push(track);
        asset_track_clips.push(clips);
    }
    let asset_tracks: Vec<AssetTrack<'_>> = asset_tracks_meta
        .iter()
        .zip(asset_track_clips.iter())
        .map(|(track, clips)| AssetTrack {
            clips,
            volume: track.volume as f32,
            muted: track.muted != 0,
            solo: track.solo != 0,
        })
        .collect();

    // --- 3. Mezcla ---------------------------------------------------------
    let summary = audio_mixer::render_mix(MixRequest {
        voice_track,
        asset_tracks: &asset_tracks,
        output,
        format,
    })?;

    tracing::info!(
        target: "mixer",
        scene_id = scene_id,
        voice_clips = nodes.len(),
        asset_tracks = asset_tracks.len(),
        duration_ms = summary.duration_ms,
        output = %summary.output_path.display(),
        "scene rendered"
    );

    Ok(SceneMixResult {
        output_path: summary.output_path.to_string_lossy().into_owned(),
        duration_ms: summary.duration_ms,
    })
}

pub fn default_export_path(paths: &ProjectPaths, scene_id: &str) -> PathBuf {
    let stamp = chrono::Utc::now().format("%Y%m%dT%H%M%S");
    paths.exports_dir().join(format!("{scene_id}-{stamp}.wav"))
}

async fn resolve_event_paths(
    db: &DatabaseConnection,
    events: &[timeline_event::Model],
) -> AppResult<HashMap<String, PathBuf>> {
    let mut out: HashMap<String, PathBuf> = HashMap::new();
    let asset_ids: Vec<String> = events
        .iter()
        .filter_map(|e| e.audio_asset_id.clone())
        .collect();
    let generated_ids: Vec<String> = events
        .iter()
        .filter_map(|e| e.generated_audio_id.clone())
        .collect();

    let assets: HashMap<String, String> = if asset_ids.is_empty() {
        HashMap::new()
    } else {
        audio_asset::Entity::find()
            .filter(audio_asset::Column::Id.is_in(asset_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|a| (a.id, a.file_path))
            .collect()
    };
    let generated: HashMap<String, String> = if generated_ids.is_empty() {
        HashMap::new()
    } else {
        generated_audio::Entity::find()
            .filter(generated_audio::Column::Id.is_in(generated_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|g| (g.id, g.file_path))
            .collect()
    };

    for event in events {
        let path = event
            .audio_asset_id
            .as_ref()
            .and_then(|id| assets.get(id))
            .or_else(|| {
                event
                    .generated_audio_id
                    .as_ref()
                    .and_then(|id| generated.get(id))
            });
        if let Some(path) = path {
            out.insert(event.id.clone(), PathBuf::from(path));
        }
    }
    Ok(out)
}
