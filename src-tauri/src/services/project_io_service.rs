//! ProjectIoService — exportar/importar proyectos en JSON (RF-39, RF-40).
//!
//! El snapshot contiene únicamente datos de BD: los archivos binarios
//! (assets, audios generados, exports) siguen referenciados por su `file_path`
//! original. La importación crea un proyecto nuevo con IDs remapeados; los
//! `file_path` se preservan tal cual y el usuario es responsable de mover los
//! archivos si trasladó el proyecto a otra máquina.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::db::open_project_database;
use crate::entities::{
    app_setting, audio_asset, character, character_alias, dialogue_node, dialogue_tts_tag,
    generated_audio, project, raw_import, scene, timeline_event, timeline_track,
};
use crate::error::{AppError, AppResult};
use crate::paths::ProjectPaths;
use crate::services::{new_id, now};

const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub schema_version: u32,
    pub exported_at: String,
    pub project: project::Model,
    pub scenes: Vec<scene::Model>,
    pub characters: Vec<character::Model>,
    pub character_aliases: Vec<character_alias::Model>,
    pub raw_imports: Vec<raw_import::Model>,
    pub dialogue_nodes: Vec<dialogue_node::Model>,
    pub dialogue_tts_tags: Vec<dialogue_tts_tag::Model>,
    pub audio_assets: Vec<audio_asset::Model>,
    pub generated_audio: Vec<generated_audio::Model>,
    pub timeline_tracks: Vec<timeline_track::Model>,
    pub timeline_events: Vec<timeline_event::Model>,
    pub app_settings: Vec<app_setting::Model>,
}

pub async fn export_to_file(
    db: &DatabaseConnection,
    project_id: &str,
    target: &Path,
) -> AppResult<PathBuf> {
    let snapshot = build_snapshot(db, project_id).await?;
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&snapshot)?;
    fs::write(target, json)?;
    Ok(target.to_path_buf())
}

pub async fn import_from_file(
    source: &Path,
    target_root: &Path,
) -> AppResult<(project::Model, ProjectPaths, DatabaseConnection)> {
    let bytes = fs::read(source)?;
    let snapshot: ProjectSnapshot = serde_json::from_slice(&bytes)?;
    if snapshot.schema_version != SCHEMA_VERSION {
        return Err(AppError::invalid(format!(
            "schema_version {} no soportada; esperado {SCHEMA_VERSION}",
            snapshot.schema_version
        )));
    }

    let paths = ProjectPaths::new(target_root.to_path_buf());
    paths.create_all()?;
    let db = open_project_database(&paths.database_file()).await?;
    let inserted = restore_snapshot(&db, snapshot, &paths).await?;
    Ok((inserted, paths, db))
}

async fn build_snapshot(db: &DatabaseConnection, project_id: &str) -> AppResult<ProjectSnapshot> {
    let project_model = project::Entity::find_by_id(project_id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("project {project_id}")))?;

    let scenes = scene::Entity::find()
        .filter(scene::Column::ProjectId.eq(project_id))
        .all(db)
        .await?;
    let scene_ids: Vec<String> = scenes.iter().map(|s| s.id.clone()).collect();

    let characters = character::Entity::find()
        .filter(character::Column::ProjectId.eq(project_id))
        .all(db)
        .await?;
    let character_ids: Vec<String> = characters.iter().map(|c| c.id.clone()).collect();

    let character_aliases = if character_ids.is_empty() {
        Vec::new()
    } else {
        character_alias::Entity::find()
            .filter(character_alias::Column::CharacterId.is_in(character_ids))
            .all(db)
            .await?
    };

    let raw_imports = raw_import::Entity::find()
        .filter(raw_import::Column::ProjectId.eq(project_id))
        .all(db)
        .await?;

    let dialogue_nodes = if scene_ids.is_empty() {
        Vec::new()
    } else {
        dialogue_node::Entity::find()
            .filter(dialogue_node::Column::SceneId.is_in(scene_ids.clone()))
            .all(db)
            .await?
    };
    let node_ids: Vec<String> = dialogue_nodes.iter().map(|n| n.id.clone()).collect();

    let dialogue_tts_tags = if node_ids.is_empty() {
        Vec::new()
    } else {
        dialogue_tts_tag::Entity::find()
            .filter(dialogue_tts_tag::Column::DialogueNodeId.is_in(node_ids.clone()))
            .all(db)
            .await?
    };

    let audio_assets = audio_asset::Entity::find()
        .filter(audio_asset::Column::ProjectId.eq(project_id))
        .all(db)
        .await?;

    let generated_audio = if node_ids.is_empty() {
        Vec::new()
    } else {
        generated_audio::Entity::find()
            .filter(generated_audio::Column::DialogueNodeId.is_in(node_ids))
            .all(db)
            .await?
    };

    let timeline_tracks = if scene_ids.is_empty() {
        Vec::new()
    } else {
        timeline_track::Entity::find()
            .filter(timeline_track::Column::SceneId.is_in(scene_ids.clone()))
            .all(db)
            .await?
    };

    let timeline_events = if scene_ids.is_empty() {
        Vec::new()
    } else {
        timeline_event::Entity::find()
            .filter(timeline_event::Column::SceneId.is_in(scene_ids))
            .all(db)
            .await?
    };

    let app_settings = app_setting::Entity::find().all(db).await?;

    Ok(ProjectSnapshot {
        schema_version: SCHEMA_VERSION,
        exported_at: now(),
        project: project_model,
        scenes,
        characters,
        character_aliases,
        raw_imports,
        dialogue_nodes,
        dialogue_tts_tags,
        audio_assets,
        generated_audio,
        timeline_tracks,
        timeline_events,
        app_settings,
    })
}

async fn restore_snapshot(
    db: &DatabaseConnection,
    snapshot: ProjectSnapshot,
    paths: &ProjectPaths,
) -> AppResult<project::Model> {
    let txn = db.begin().await?;
    let now_ts = now();

    let new_project_id = new_id();
    let project_active = project::ActiveModel {
        id: Set(new_project_id.clone()),
        title: Set(snapshot.project.title),
        description: Set(snapshot.project.description),
        language: Set(snapshot.project.language),
        root_path: Set(paths.root.to_string_lossy().into_owned()),
        created_at: Set(now_ts.clone()),
        updated_at: Set(now_ts.clone()),
    };
    let project_model = project_active.insert(&txn).await?;

    let scene_id_map: HashMap<String, String> = snapshot
        .scenes
        .iter()
        .map(|s| (s.id.clone(), new_id()))
        .collect();
    for scene_model in snapshot.scenes {
        let new_id = scene_id_map[&scene_model.id].clone();
        scene::ActiveModel {
            id: Set(new_id),
            project_id: Set(new_project_id.clone()),
            title: Set(scene_model.title),
            description: Set(scene_model.description),
            order_index: Set(scene_model.order_index),
            created_at: Set(scene_model.created_at),
            updated_at: Set(scene_model.updated_at),
        }
        .insert(&txn)
        .await?;
    }

    let character_id_map: HashMap<String, String> = snapshot
        .characters
        .iter()
        .map(|c| (c.id.clone(), new_id()))
        .collect();
    for character_model in snapshot.characters {
        let new_id = character_id_map[&character_model.id].clone();
        character::ActiveModel {
            id: Set(new_id),
            project_id: Set(new_project_id.clone()),
            name: Set(character_model.name),
            role: Set(character_model.role),
            description: Set(character_model.description),
            color: Set(character_model.color),
            voice_provider: Set(character_model.voice_provider),
            voice_id: Set(character_model.voice_id),
            default_style_prompt: Set(character_model.default_style_prompt),
            created_at: Set(character_model.created_at),
            updated_at: Set(character_model.updated_at),
        }
        .insert(&txn)
        .await?;
    }

    for alias in snapshot.character_aliases {
        let Some(character_id) = character_id_map.get(&alias.character_id) else {
            continue;
        };
        character_alias::ActiveModel {
            id: Set(new_id()),
            character_id: Set(character_id.clone()),
            alias: Set(alias.alias),
        }
        .insert(&txn)
        .await?;
    }

    let raw_import_id_map: HashMap<String, String> = snapshot
        .raw_imports
        .iter()
        .map(|r| (r.id.clone(), new_id()))
        .collect();
    for raw in snapshot.raw_imports {
        let new_raw_id = raw_import_id_map[&raw.id].clone();
        let mapped_scene = raw
            .scene_id
            .as_ref()
            .and_then(|s| scene_id_map.get(s).cloned());
        raw_import::ActiveModel {
            id: Set(new_raw_id),
            project_id: Set(new_project_id.clone()),
            scene_id: Set(mapped_scene),
            source_type: Set(raw.source_type),
            source_file_path: Set(raw.source_file_path),
            original_text: Set(raw.original_text),
            processed_json: Set(raw.processed_json),
            status: Set(raw.status),
            error_message: Set(raw.error_message),
            created_at: Set(raw.created_at),
        }
        .insert(&txn)
        .await?;
    }

    let node_id_map: HashMap<String, String> = snapshot
        .dialogue_nodes
        .iter()
        .map(|n| (n.id.clone(), new_id()))
        .collect();
    for node in snapshot.dialogue_nodes {
        let Some(scene_id) = scene_id_map.get(&node.scene_id) else {
            continue;
        };
        let Some(character_id) = character_id_map.get(&node.character_id) else {
            continue;
        };
        let new_node_id = node_id_map[&node.id].clone();
        dialogue_node::ActiveModel {
            id: Set(new_node_id),
            scene_id: Set(scene_id.clone()),
            character_id: Set(character_id.clone()),
            previous_id: Set(node.previous_id.and_then(|p| node_id_map.get(&p).cloned())),
            next_id: Set(node.next_id.and_then(|p| node_id_map.get(&p).cloned())),
            order_index: Set(node.order_index),
            kind: Set(node.kind),
            text: Set(node.text),
            raw_text: Set(node.raw_text),
            emotion: Set(node.emotion),
            intensity: Set(node.intensity),
            is_enabled: Set(node.is_enabled),
            before_delay_ms: Set(node.before_delay_ms),
            after_delay_ms: Set(node.after_delay_ms),
            created_at: Set(node.created_at),
            updated_at: Set(node.updated_at),
        }
        .insert(&txn)
        .await?;
    }

    for tag in snapshot.dialogue_tts_tags {
        let Some(node_id) = node_id_map.get(&tag.dialogue_node_id) else {
            continue;
        };
        dialogue_tts_tag::ActiveModel {
            id: Set(new_id()),
            dialogue_node_id: Set(node_id.clone()),
            tag: Set(tag.tag),
            position: Set(tag.position),
            order_index: Set(tag.order_index),
            source: Set(tag.source),
        }
        .insert(&txn)
        .await?;
    }

    let asset_id_map: HashMap<String, String> = snapshot
        .audio_assets
        .iter()
        .map(|a| (a.id.clone(), new_id()))
        .collect();
    for asset in snapshot.audio_assets {
        let new_asset_id = asset_id_map[&asset.id].clone();
        audio_asset::ActiveModel {
            id: Set(new_asset_id),
            project_id: Set(new_project_id.clone()),
            name: Set(asset.name),
            kind: Set(asset.kind),
            file_path: Set(asset.file_path),
            duration_ms: Set(asset.duration_ms),
            original_file_name: Set(asset.original_file_name),
            mime_type: Set(asset.mime_type),
            created_at: Set(asset.created_at),
        }
        .insert(&txn)
        .await?;
    }

    let generated_id_map: HashMap<String, String> = snapshot
        .generated_audio
        .iter()
        .map(|g| (g.id.clone(), new_id()))
        .collect();
    for audio in snapshot.generated_audio {
        let Some(node_id) = node_id_map.get(&audio.dialogue_node_id) else {
            continue;
        };
        let new_audio_id = generated_id_map[&audio.id].clone();
        generated_audio::ActiveModel {
            id: Set(new_audio_id),
            dialogue_node_id: Set(node_id.clone()),
            provider: Set(audio.provider),
            model: Set(audio.model),
            voice_id: Set(audio.voice_id),
            input_hash: Set(audio.input_hash),
            file_path: Set(audio.file_path),
            duration_ms: Set(audio.duration_ms),
            status: Set(audio.status),
            error_message: Set(audio.error_message),
            created_at: Set(audio.created_at),
        }
        .insert(&txn)
        .await?;
    }

    let track_id_map: HashMap<String, String> = snapshot
        .timeline_tracks
        .iter()
        .map(|t| (t.id.clone(), new_id()))
        .collect();
    for track in snapshot.timeline_tracks {
        let Some(scene_id) = scene_id_map.get(&track.scene_id) else {
            continue;
        };
        let new_track_id = track_id_map[&track.id].clone();
        timeline_track::ActiveModel {
            id: Set(new_track_id),
            scene_id: Set(scene_id.clone()),
            name: Set(track.name),
            kind: Set(track.kind),
            order_index: Set(track.order_index),
            volume: Set(track.volume),
            muted: Set(track.muted),
            solo: Set(track.solo),
        }
        .insert(&txn)
        .await?;
    }

    for event in snapshot.timeline_events {
        let Some(scene_id) = scene_id_map.get(&event.scene_id) else {
            continue;
        };
        let Some(track_id) = track_id_map.get(&event.track_id) else {
            continue;
        };
        timeline_event::ActiveModel {
            id: Set(new_id()),
            scene_id: Set(scene_id.clone()),
            track_id: Set(track_id.clone()),
            dialogue_node_id: Set(event
                .dialogue_node_id
                .and_then(|n| node_id_map.get(&n).cloned())),
            audio_asset_id: Set(event
                .audio_asset_id
                .and_then(|a| asset_id_map.get(&a).cloned())),
            generated_audio_id: Set(event
                .generated_audio_id
                .and_then(|g| generated_id_map.get(&g).cloned())),
            start_ms: Set(event.start_ms),
            duration_ms: Set(event.duration_ms),
            offset_ms: Set(event.offset_ms),
            volume: Set(event.volume),
            fade_in_ms: Set(event.fade_in_ms),
            fade_out_ms: Set(event.fade_out_ms),
            playback_rate: Set(event.playback_rate),
            looping: Set(event.looping),
            created_at: Set(event.created_at),
            updated_at: Set(event.updated_at),
        }
        .insert(&txn)
        .await?;
    }

    for setting in snapshot.app_settings {
        app_setting::ActiveModel {
            id: Set(new_id()),
            key: Set(setting.key),
            value: Set(setting.value),
            created_at: Set(setting.created_at),
            updated_at: Set(setting.updated_at),
        }
        .insert(&txn)
        .await?;
    }

    txn.commit().await?;
    Ok(project_model)
}
