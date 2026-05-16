//! TtsService — orquestación de generación de audio por diálogo
//! (RF-24, RF-25, RF-30, RF-31).
//!
//! Reglas:
//! - Antes de pegarle a Gemini, consultamos la caché por `input_hash` y la
//!   existencia del archivo en disco.
//! - Cuando generamos un audio nuevo, marcamos cualquier `generated_audio`
//!   previo del mismo nodo como `outdated`.
//! - Los IDs de archivo coinciden con el id del registro para evitar
//!   colisiones y simplificar el GC futuro.

use std::fs;
use std::path::Path;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set,
};
use sha2::{Digest, Sha256};

use crate::entities::{character, dialogue_node, dialogue_tts_tag, generated_audio};
use crate::error::{AppError, AppResult};
use crate::paths::ProjectPaths;
use crate::services::credential_service::CredentialService;
use crate::services::gemini_tts_service::{GeminiTtsService, TtsRequest, TtsResult};
use crate::services::render_planner::{input_hash, DialogueRenderInput};
use crate::services::{new_id, now};

const PROVIDER: &str = "gemini";
const MODEL: &str = "gemini-3.1-flash-tts-preview";

pub async fn synthesize_dialogue(
    db: &DatabaseConnection,
    paths: &ProjectPaths,
    credentials: &CredentialService,
    dialogue_node_id: &str,
    force: bool,
) -> AppResult<generated_audio::Model> {
    let node = dialogue_node::Entity::find_by_id(dialogue_node_id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("dialogue_node {dialogue_node_id}")))?;
    let character = character::Entity::find_by_id(node.character_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("character {}", node.character_id)))?;
    let voice_id = character.voice_id.clone().ok_or_else(|| {
        AppError::invalid(format!(
            "el personaje '{}' no tiene voz asignada",
            character.name
        ))
    })?;

    let tags = dialogue_tts_tag::Entity::find()
        .filter(dialogue_tts_tag::Column::DialogueNodeId.eq(dialogue_node_id))
        .order_by_asc(dialogue_tts_tag::Column::OrderIndex)
        .all(db)
        .await?;
    let tag_list: Vec<String> = tags.iter().map(|t| t.tag.trim().to_owned()).collect();
    let tag_signature = build_tag_signature(&tag_list);
    let style_prompt = character.default_style_prompt.as_deref();

    let render_input = DialogueRenderInput {
        node: &node,
        voice_id: &voice_id,
        model: MODEL,
        tag_signature: &tag_signature,
        style_prompt,
    };
    let hash = input_hash(&render_input);

    if !force {
        if let Some(cached) = generated_audio::Entity::find()
            .filter(generated_audio::Column::DialogueNodeId.eq(dialogue_node_id))
            .filter(generated_audio::Column::InputHash.eq(&hash))
            .filter(generated_audio::Column::Status.eq("generated"))
            .one(db)
            .await?
        {
            if Path::new(&cached.file_path).exists() {
                return Ok(cached);
            }
        }
    }

    let req = TtsRequest {
        text: build_prompt_text(&tag_list, &node.text),
        voice_id: voice_id.clone(),
        tags: tag_list,
        style_prompt: style_prompt.map(str::to_owned),
    };
    let gemini = GeminiTtsService::new(credentials).with_model(MODEL);
    let result = gemini.synthesize(req).await?;

    fs::create_dir_all(paths.generated_audio_dir())?;
    let audio_id = new_id();
    let file_path = paths.generated_audio_dir().join(format!("{audio_id}.wav"));
    write_audio_as_wav(&file_path, &result)?;
    let duration_ms = wav_duration_ms(&file_path).ok();

    mark_previous_as_outdated(db, dialogue_node_id).await?;

    let inserted = generated_audio::ActiveModel {
        id: Set(audio_id),
        dialogue_node_id: Set(dialogue_node_id.to_owned()),
        provider: Set(PROVIDER.into()),
        model: Set(MODEL.into()),
        voice_id: Set(voice_id),
        input_hash: Set(hash),
        file_path: Set(file_path.to_string_lossy().into_owned()),
        duration_ms: Set(duration_ms),
        status: Set("generated".into()),
        error_message: Set(None),
        created_at: Set(now()),
    }
    .insert(db)
    .await?;

    Ok(inserted)
}

pub async fn regenerate_outdated_in_scene(
    db: &DatabaseConnection,
    paths: &ProjectPaths,
    credentials: &CredentialService,
    scene_id: &str,
) -> AppResult<Vec<generated_audio::Model>> {
    let nodes = dialogue_node::Entity::find()
        .filter(dialogue_node::Column::SceneId.eq(scene_id))
        .order_by_asc(dialogue_node::Column::OrderIndex)
        .all(db)
        .await?;

    let mut results = Vec::new();
    for node in nodes {
        let latest = generated_audio::Entity::find()
            .filter(generated_audio::Column::DialogueNodeId.eq(&node.id))
            .order_by_desc(generated_audio::Column::CreatedAt)
            .one(db)
            .await?;
        let needs = matches!(
            latest.as_ref().map(|a| a.status.as_str()),
            Some("outdated") | Some("failed")
        );
        if needs {
            let regenerated = synthesize_dialogue(db, paths, credentials, &node.id, true).await?;
            results.push(regenerated);
        }
    }
    Ok(results)
}

pub async fn list_for_scene(
    db: &DatabaseConnection,
    scene_id: &str,
) -> AppResult<Vec<generated_audio::Model>> {
    let node_ids: Vec<String> = dialogue_node::Entity::find()
        .filter(dialogue_node::Column::SceneId.eq(scene_id))
        .all(db)
        .await?
        .into_iter()
        .map(|node| node.id)
        .collect();
    if node_ids.is_empty() {
        return Ok(Vec::new());
    }
    Ok(generated_audio::Entity::find()
        .filter(generated_audio::Column::DialogueNodeId.is_in(node_ids))
        .order_by_desc(generated_audio::Column::CreatedAt)
        .all(db)
        .await?)
}

pub async fn synthesize_voice_sample(
    paths: &ProjectPaths,
    credentials: &CredentialService,
    voice_provider: &str,
    voice_id: &str,
    sample_text: &str,
) -> AppResult<String> {
    if voice_provider != PROVIDER {
        return Err(AppError::invalid(format!(
            "proveedor de voz no soportado: {voice_provider}"
        )));
    }
    if voice_id.trim().is_empty() {
        return Err(AppError::invalid("voice_id vacío"));
    }
    if sample_text.trim().is_empty() {
        return Err(AppError::invalid("texto vacío para muestra de voz"));
    }

    let sample_dir = paths.cache_dir().join("voice-samples");
    fs::create_dir_all(&sample_dir)?;
    let file_path = sample_dir.join(format!(
        "{}.wav",
        voice_sample_hash(voice_provider, voice_id.trim(), sample_text.trim())
    ));
    if file_path.exists() && wav_duration_ms(&file_path).is_ok() {
        return Ok(file_path.to_string_lossy().into_owned());
    }

    let gemini = GeminiTtsService::new(credentials).with_model(MODEL);
    let result = gemini
        .synthesize(TtsRequest {
            text: sample_text.trim().to_owned(),
            voice_id: voice_id.trim().to_owned(),
            tags: Vec::new(),
            style_prompt: None,
        })
        .await?;
    write_audio_as_wav(&file_path, &result)?;
    Ok(file_path.to_string_lossy().into_owned())
}

async fn mark_previous_as_outdated(
    db: &DatabaseConnection,
    dialogue_node_id: &str,
) -> AppResult<()> {
    let existing = generated_audio::Entity::find()
        .filter(generated_audio::Column::DialogueNodeId.eq(dialogue_node_id))
        .filter(generated_audio::Column::Status.eq("generated"))
        .all(db)
        .await?;
    for audio in existing {
        let mut active = audio.into_active_model();
        active.status = Set("outdated".into());
        active.update(db).await?;
    }
    Ok(())
}

fn build_prompt_text(tags: &[String], text: &str) -> String {
    if tags.is_empty() {
        text.to_owned()
    } else {
        format!("{} {}", tags.join(" "), text)
    }
}

fn build_tag_signature(tags: &[String]) -> String {
    let mut sorted: Vec<String> = tags.iter().map(|t| t.trim().to_owned()).collect();
    sorted.sort();
    sorted.dedup();
    sorted.join(",")
}

fn voice_sample_hash(voice_provider: &str, voice_id: &str, sample_text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(voice_provider.as_bytes());
    hasher.update(b"|");
    hasher.update(voice_id.as_bytes());
    hasher.update(b"|");
    hasher.update(MODEL.as_bytes());
    hasher.update(b"|");
    hasher.update(sample_text.as_bytes());
    hex::encode(hasher.finalize())
}

fn write_audio_as_wav(path: &Path, audio: &TtsResult) -> AppResult<()> {
    if looks_like_wav(&audio.bytes) {
        fs::write(path, &audio.bytes)?;
        return Ok(());
    }

    let (sample_rate, channels) = pcm_mime_params(&audio.mime_type).ok_or_else(|| {
        AppError::Provider(format!(
            "formato de audio no soportado por la mezcla local: {}",
            audio.mime_type
        ))
    })?;
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)
        .map_err(|e| AppError::internal(format!("hound create: {e}")))?;
    for chunk in audio.bytes.chunks_exact(2) {
        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
        writer
            .write_sample(sample)
            .map_err(|e| AppError::internal(format!("hound write: {e}")))?;
    }
    writer
        .finalize()
        .map_err(|e| AppError::internal(format!("hound finalize: {e}")))?;
    Ok(())
}

fn looks_like_wav(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE"
}

fn pcm_mime_params(mime: &str) -> Option<(u32, u16)> {
    let lower = mime.to_ascii_lowercase();
    if !(lower.starts_with("audio/l16")
        || lower.starts_with("audio/pcm")
        || lower.starts_with("audio/wav")
        || lower.starts_with("audio/x-wav"))
    {
        return None;
    }
    let mut sample_rate: u32 = 24_000;
    let mut channels: u16 = 1;
    for part in mime.split(';').skip(1) {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("rate=") {
            sample_rate = value.parse().ok()?;
        } else if let Some(value) = part.strip_prefix("channels=") {
            channels = value.parse().ok()?;
        }
    }
    Some((sample_rate, channels))
}

fn wav_duration_ms(path: &Path) -> AppResult<i32> {
    let reader =
        hound::WavReader::open(path).map_err(|e| AppError::internal(format!("hound open: {e}")))?;
    let spec = reader.spec();
    let frames = reader.duration() as u64;
    if spec.sample_rate == 0 {
        return Ok(0);
    }
    Ok((frames * 1000 / spec.sample_rate as u64) as i32)
}
