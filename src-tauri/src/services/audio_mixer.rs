//! AudioMixer — construye la mezcla final a partir del timeline (RF-26, RF-36, RF-37).
//!
//! Implementación pendiente: usa `symphonia` para decodificar fuentes, `rubato`
//! para resamplear cuando los sample rates difieran y `hound` para escribir el
//! WAV de salida. El MP3 quedará para MVP 2 (requiere wrapper opcional).

use std::path::Path;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Wav,
    Mp3,
}

pub struct MixRequest<'a> {
    pub scene_id: &'a str,
    pub output: &'a Path,
    pub format: ExportFormat,
}

pub async fn render(_req: MixRequest<'_>) -> AppResult<()> {
    Err(AppError::NotImplemented("AudioMixer::render"))
}
