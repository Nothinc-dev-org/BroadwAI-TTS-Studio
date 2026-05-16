//! GeminiTtsService — síntesis de voz vía Gemini 3.1 Flash TTS Preview
//! (RF-23 a RF-31).
//!
//! La API key se obtiene del [`CredentialService`] y se envía vía header
//! `x-goog-api-key`; nunca debe loggearse el cuerpo de la petición.

use std::time::Duration;

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::{AppError, AppResult};
use crate::services::credential_service::{ApiKeyStatus, CredentialService, Provider};

const DEFAULT_API_BASE: &str = "https://generativelanguage.googleapis.com";
const DEFAULT_MODEL: &str = "gemini-3.1-flash-tts-preview";
const REQUEST_TIMEOUT_SECS: u64 = 120;

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

#[derive(Debug, Deserialize)]
struct GenerateContentResponse {
    #[serde(default)]
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    #[serde(default)]
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    #[serde(rename = "inlineData", default)]
    inline_data: Option<InlineData>,
}

#[derive(Debug, Deserialize)]
struct InlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
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

    pub async fn test_connection(&self) -> AppResult<ApiKeyStatus> {
        let key = match self.credentials.read(Provider::Gemini) {
            Ok(key) => key,
            Err(AppError::InvalidInput(_)) => return Ok(ApiKeyStatus::NotConfigured),
            Err(err) => return Err(err),
        };
        let url = format!("{}/v1beta/models", self.api_base.trim_end_matches('/'));
        let response = match self
            .http
            .get(url)
            .header("x-goog-api-key", key.as_str())
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .send()
            .await
        {
            Ok(response) => response,
            Err(_) => return Ok(ApiKeyStatus::ConnectionError),
        };

        Ok(match response.status() {
            status if status.is_success() => ApiKeyStatus::Valid,
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => ApiKeyStatus::Invalid,
            _ => ApiKeyStatus::ConnectionError,
        })
    }

    pub async fn synthesize(&self, req: TtsRequest) -> AppResult<TtsResult> {
        if req.text.trim().is_empty() {
            return Err(AppError::invalid("texto vacío para TTS"));
        }
        if req.voice_id.trim().is_empty() {
            return Err(AppError::invalid("voice_id vacío"));
        }

        let key = self.credentials.read(Provider::Gemini)?;
        let url = format!(
            "{}/v1beta/models/{}:generateContent",
            self.api_base.trim_end_matches('/'),
            self.model
        );

        let mut body = json!({
            "contents": [{ "parts": [{ "text": req.text }] }],
            "generationConfig": {
                "responseModalities": ["AUDIO"],
                "speechConfig": {
                    "voiceConfig": {
                        "prebuiltVoiceConfig": { "voiceName": req.voice_id }
                    }
                }
            }
        });
        if let Some(prompt) = req
            .style_prompt
            .as_deref()
            .map(str::trim)
            .filter(|p| !p.is_empty())
        {
            body["systemInstruction"] = json!({ "parts": [{ "text": prompt }] });
        }

        let response = self
            .http
            .post(url)
            .header("x-goog-api-key", key.as_str())
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            // No loggear cuerpo: puede contener fragmentos del guion del usuario.
            return Err(AppError::Provider(format!(
                "Gemini TTS respondió con estado HTTP {}",
                response.status()
            )));
        }

        let payload: GenerateContentResponse = response.json().await?;
        let inline = payload
            .candidates
            .into_iter()
            .next()
            .and_then(|c| c.content.parts.into_iter().find_map(|p| p.inline_data))
            .ok_or_else(|| AppError::Provider("Gemini TTS no devolvió audio".into()))?;
        let bytes = BASE64_STANDARD
            .decode(inline.data.as_bytes())
            .map_err(|e| AppError::Provider(format!("base64 inválido en respuesta TTS: {e}")))?;
        Ok(TtsResult {
            bytes,
            mime_type: inline.mime_type,
        })
    }
}
