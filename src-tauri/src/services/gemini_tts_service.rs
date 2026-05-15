//! GeminiTtsService — síntesis de voz vía Gemini 3.1 Flash TTS Preview
//! (RF-23 a RF-31).

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use crate::services::credential_service::{CredentialService, Provider};

const DEFAULT_API_BASE: &str = "https://generativelanguage.googleapis.com";
const DEFAULT_MODEL: &str = "gemini-3.1-flash-tts-preview";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsRequest {
    pub text: String,
    pub voice_id: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub style_prompt: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TtsResult {
    pub bytes: Vec<u8>,
    pub mime_type: String,
}

pub struct GeminiTtsService<'a> {
    credentials: &'a CredentialService,
    http: Client,
    api_base: String,
    model: String,
}

impl<'a> GeminiTtsService<'a> {
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

    pub async fn synthesize(&self, _req: TtsRequest) -> AppResult<TtsResult> {
        let _key = self.credentials.read(Provider::Gemini)?;
        let _ = &self.http;
        let _ = &self.api_base;
        let _ = &self.model;
        Err(AppError::NotImplemented("GeminiTtsService::synthesize"))
    }
}
