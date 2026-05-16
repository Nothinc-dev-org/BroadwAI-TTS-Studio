//! AssetService — biblioteca de assets de audio (RF-32, RF-33).
//!
//! Reglas:
//! - Importar copia el archivo a `<root>/assets/<kind>/<uuid>.<ext>` para que
//!   el proyecto sea movible (paths absolutos seguirían rotos, pero el archivo
//!   queda dentro del root y la próxima apertura se reasocia con `root_path`).
//! - La duración se mide con symphonia al importar; si falla la sonda, el
//!   asset queda con `duration_ms = None`.

use std::fs;
use std::path::{Path, PathBuf};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::entities::audio_asset;
use crate::error::{AppError, AppResult};
use crate::paths::ProjectPaths;
use crate::services::audio_mixer;
use crate::services::{new_id, now};

const SUPPORTED_EXTENSIONS: &[&str] = &["wav", "mp3", "ogg", "flac"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    SoundEffect,
    Music,
    Ambience,
    Voice,
    Generated,
}

impl AssetKind {
    pub fn parse(value: &str) -> AppResult<Self> {
        match value {
            "sound_effect" => Ok(AssetKind::SoundEffect),
            "music" => Ok(AssetKind::Music),
            "ambience" => Ok(AssetKind::Ambience),
            "voice" => Ok(AssetKind::Voice),
            "generated" => Ok(AssetKind::Generated),
            other => Err(AppError::invalid(format!(
                "kind de asset inválido: {other}"
            ))),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            AssetKind::SoundEffect => "sound_effect",
            AssetKind::Music => "music",
            AssetKind::Ambience => "ambience",
            AssetKind::Voice => "voice",
            AssetKind::Generated => "generated",
        }
    }

    pub fn target_dir(self, paths: &ProjectPaths) -> PathBuf {
        match self {
            AssetKind::SoundEffect => paths.sfx_dir(),
            AssetKind::Music => paths.music_dir(),
            AssetKind::Ambience => paths.ambience_dir(),
            AssetKind::Voice | AssetKind::Generated => paths.assets_dir().join(self.as_str()),
        }
    }
}

pub async fn list(db: &DatabaseConnection, project_id: &str) -> AppResult<Vec<audio_asset::Model>> {
    Ok(audio_asset::Entity::find()
        .filter(audio_asset::Column::ProjectId.eq(project_id))
        .all(db)
        .await?)
}

pub async fn get(db: &DatabaseConnection, id: &str) -> AppResult<audio_asset::Model> {
    audio_asset::Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("audio_asset {id}")))
}

pub async fn import_from_file(
    db: &DatabaseConnection,
    paths: &ProjectPaths,
    project_id: &str,
    source_path: &Path,
    kind: AssetKind,
    name: Option<String>,
) -> AppResult<audio_asset::Model> {
    if !source_path.exists() {
        return Err(AppError::invalid(format!(
            "el archivo no existe: {}",
            source_path.display()
        )));
    }
    let ext = source_path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .ok_or_else(|| AppError::invalid("el archivo no tiene extensión"))?;
    if !SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(AppError::invalid(format!(
            "extensión no soportada: {ext}; usa wav, mp3, ogg o flac"
        )));
    }

    let target_dir = kind.target_dir(paths);
    fs::create_dir_all(&target_dir)?;
    let asset_id = new_id();
    let target_path = target_dir.join(format!("{asset_id}.{ext}"));
    fs::copy(source_path, &target_path)?;

    let duration_ms = audio_mixer::probe_duration_ms(&target_path).unwrap_or(None);
    let display_name = name.unwrap_or_else(|| {
        source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("asset")
            .to_owned()
    });
    let mime_type = guess_mime(&ext);
    let original_file_name = source_path
        .file_name()
        .and_then(|s| s.to_str())
        .map(str::to_owned);

    let inserted = audio_asset::ActiveModel {
        id: Set(asset_id),
        project_id: Set(project_id.to_owned()),
        name: Set(display_name),
        kind: Set(kind.as_str().to_owned()),
        file_path: Set(target_path.to_string_lossy().into_owned()),
        duration_ms: Set(duration_ms),
        original_file_name: Set(original_file_name),
        mime_type: Set(mime_type),
        created_at: Set(now()),
    }
    .insert(db)
    .await?;

    Ok(inserted)
}

pub async fn rename(
    db: &DatabaseConnection,
    id: &str,
    name: Option<String>,
    kind: Option<AssetKind>,
) -> AppResult<audio_asset::Model> {
    let model = get(db, id).await?;
    let mut active = model.into_active_model();
    if let Some(name) = name {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(AppError::invalid("el nombre no puede estar vacío"));
        }
        active.name = Set(trimmed.to_owned());
    }
    if let Some(kind) = kind {
        active.kind = Set(kind.as_str().to_owned());
    }
    Ok(active.update(db).await?)
}

pub async fn delete(db: &DatabaseConnection, id: &str) -> AppResult<()> {
    let model = get(db, id).await?;
    let _ = fs::remove_file(&model.file_path); // archivo puede haber sido movido fuera
    audio_asset::Entity::delete_by_id(model.id).exec(db).await?;
    Ok(())
}

fn guess_mime(ext: &str) -> Option<String> {
    Some(
        match ext {
            "wav" => "audio/wav",
            "mp3" => "audio/mpeg",
            "ogg" => "audio/ogg",
            "flac" => "audio/flac",
            _ => return None,
        }
        .to_owned(),
    )
}
