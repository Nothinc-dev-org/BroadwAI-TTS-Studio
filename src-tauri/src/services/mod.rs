pub mod asset_service;
pub mod audio_mixer;
pub mod character_service;
pub mod credential_service;
pub mod deepseek_service;
pub mod dialogue_service;
pub mod gemini_tts_service;
pub mod import_service;
pub mod project_io_service;
pub mod project_service;
pub mod render_planner;
pub mod scene_mix_service;
pub mod scene_service;
pub mod timeline_service;
pub mod tts_optimization_service;
pub mod tts_service;

/// Identificador del servicio en el keyring del sistema.
pub const KEYRING_SERVICE: &str = "ai.broadwai.tts-studio";

/// Genera un nuevo identificador único para entidades persistidas.
pub fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Devuelve el timestamp actual en formato ISO-8601.
pub fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
