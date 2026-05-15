//! DeepSeekService — estructuración de guion (RF-11 a RF-15, RF-22).
//!
//! La API key viene del [`CredentialService`] y nunca debe loggearse.

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use crate::services::credential_service::{CredentialService, Provider};

const DEFAULT_API_BASE: &str = "https://api.deepseek.com";
const DEFAULT_MODEL: &str = "deepseek-v4-flash";

pub const SYSTEM_PROMPT: &str = r#"You are a screenplay-to-TTS structuring engine.

Your task is to convert the user's Spanish prose/script into a strict JSON scene format for Gemini 3.1 Flash TTS.

Rules:
- Do not summarize.
- Do not invent dialogue.
- Do not remove profanity, slang, violence, tension, or character tone.
- Preserve the original meaning and order.
- Split narration and spoken dialogue into separate blocks.
- Assign every block to a speaker.
- Use "Narrador" for third-person narration.
- Detect chat-message dialogue as spoken dialogue unless marked otherwise.
- Add Gemini TTS inline tags in English, using bracket syntax, such as [neutral], [warm], [short pause], [tension], [panic], [angry], [whispers].
- Keep tags minimal and meaningful.
- Return only valid JSON.
- No markdown.
- No explanation."#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredCharacter {
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredDialogue {
    pub speaker: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub tts_tags: Vec<String>,
    pub text: String,
    #[serde(default)]
    pub original_excerpt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredScene {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub language: String,
    pub characters: Vec<StructuredCharacter>,
    pub dialogues: Vec<StructuredDialogue>,
    #[serde(default)]
    pub unassigned_fragments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekResult {
    pub scene: StructuredScene,
}

pub struct DeepSeekService<'a> {
    credentials: &'a CredentialService,
    http: Client,
    api_base: String,
    model: String,
}

impl<'a> DeepSeekService<'a> {
    pub fn new(credentials: &'a CredentialService) -> Self {
        Self {
            credentials,
            http: Client::new(),
            api_base: DEFAULT_API_BASE.into(),
            model: DEFAULT_MODEL.into(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Estructura el guion. Pendiente de wiring final con DeepSeek; por ahora
    /// la implementación devuelve `NotImplemented` y sirve como punto de
    /// extensión para el sprint de MVP 1.
    pub async fn structure_script(&self, _text: &str) -> AppResult<DeepSeekResult> {
        // Verificamos que exista la API key sin exponerla.
        let _key = self.credentials.read(Provider::Deepseek)?;
        let _ = &self.http;
        let _ = &self.api_base;
        let _ = &self.model;
        Err(AppError::NotImplemented("DeepSeekService::structure_script"))
    }
}
