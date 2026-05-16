//! DeepSeekService — estructuración de guion (RF-11 a RF-15, RF-22).
//!
//! La API key viene del [`CredentialService`] y nunca debe loggearse.

use std::collections::BTreeMap;
use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;

use crate::error::{AppError, AppResult};
use crate::services::credential_service::{ApiKeyStatus, CredentialService, Provider};

const DEFAULT_API_BASE: &str = "https://api.deepseek.com";
const DEFAULT_MODEL: &str = "deepseek-v4-flash";
const REQUEST_TIMEOUT_SECS: u64 = 90;
const REQUEST_RETRIES: usize = 2;
const STRUCTURE_MAX_TOKENS: u32 = 16384;
const STRUCTURE_CHUNK_MAX_CHARS: usize = 6_000;

pub const TTS_OPTIMIZATION_PROMPT: &str = r#"You are a TTS tag optimization engine for Gemini 3.1 Flash TTS.

You receive a JSON array of dialogue blocks with their current inline TTS tags.

Your job is to suggest improved tags for each block. You MAY:
- Replace, add or remove inline TTS tags such as [neutral], [warm], [short pause], [tension], [panic], [angry], [whispers].
- Reorder tags.

You MUST NOT:
- Change the "text" field.
- Change the "speaker" field.
- Change the "type" field.
- Change the order of blocks.
- Summarize, rephrase or soften the content.

Return ONLY a JSON object of the form:
{
  "updates": [
    { "id": "<block id>", "tags": ["[neutral]", "[short pause]"] }
  ]
}

No markdown. No explanation."#;

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
    #[serde(default, deserialize_with = "deserialize_tts_tags")]
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
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationInputBlock {
    pub id: String,
    pub speaker: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsUpdate {
    pub id: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct OptimizationResponse {
    #[serde(default)]
    updates: Vec<TagsUpdate>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionMessage {
    content: String,
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
            http: deepseek_http_client(),
            api_base: DEFAULT_API_BASE.into(),
            model: DEFAULT_MODEL.into(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub async fn test_connection(&self) -> AppResult<ApiKeyStatus> {
        let key = match self.credentials.read(Provider::Deepseek) {
            Ok(key) => key,
            Err(AppError::InvalidInput(_)) => return Ok(ApiKeyStatus::NotConfigured),
            Err(err) => return Err(err),
        };
        let url = format!("{}/models", self.api_base.trim_end_matches('/'));
        let response = match self
            .http
            .get(url)
            .bearer_auth(key)
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

    pub async fn optimize_tts_tags(
        &self,
        blocks: &[OptimizationInputBlock],
    ) -> AppResult<Vec<TagsUpdate>> {
        if blocks.is_empty() {
            return Ok(Vec::new());
        }
        let key = self.credentials.read(Provider::Deepseek)?;
        let url = format!("{}/chat/completions", self.api_base.trim_end_matches('/'));
        let user_payload = serde_json::to_string(blocks)?;
        let response = self
            .http
            .post(url)
            .bearer_auth(key)
            .header(reqwest::header::ACCEPT_ENCODING, "identity")
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .json(&json!({
                "model": self.model,
                "messages": [
                    { "role": "system", "content": TTS_OPTIMIZATION_PROMPT },
                    { "role": "user", "content": user_payload }
                ],
                "response_format": { "type": "json_object" },
                "temperature": 0.2,
                "max_tokens": 4096
            }))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(AppError::Provider(format!(
                "DeepSeek respondió con estado HTTP {}",
                response.status()
            )));
        }
        let payload = parse_chat_completion_response(response, "optimize_tts_tags").await?;
        let content = payload
            .choices
            .first()
            .ok_or_else(|| AppError::Provider("DeepSeek no devolvió opciones".into()))?
            .message
            .content
            .trim();
        let parsed: OptimizationResponse = serde_json::from_str(content)?;
        let allowed: std::collections::HashSet<&str> =
            blocks.iter().map(|b| b.id.as_str()).collect();
        let updates = parsed
            .updates
            .into_iter()
            .filter(|u| allowed.contains(u.id.as_str()))
            .map(|mut u| {
                u.tags.retain(|t| is_tts_tag(t));
                u
            })
            .collect();
        Ok(updates)
    }

    pub async fn structure_script(&self, text: &str) -> AppResult<DeepSeekResult> {
        self.structure_script_with_progress(text, |_, _| {}).await
    }

    pub async fn structure_script_with_progress<F>(
        &self,
        text: &str,
        mut progress: F,
    ) -> AppResult<DeepSeekResult>
    where
        F: FnMut(usize, usize),
    {
        if text.trim().is_empty() {
            return Err(AppError::invalid("el texto importado está vacío"));
        }

        let key = self.credentials.read(Provider::Deepseek)?;
        let chunks = split_text_chunks(text, STRUCTURE_CHUNK_MAX_CHARS);
        if chunks.len() > 1 {
            tracing::debug!(
                target: "deepseek",
                chunks = chunks.len(),
                text_chars = text.chars().count(),
                "Structuring script in chunks"
            );
            let mut results = Vec::with_capacity(chunks.len());
            let total = chunks.len();
            progress(0, total);
            for (index, chunk) in chunks.into_iter().enumerate() {
                results.push(
                    self.structure_script_once(&key, &chunk, "structure_script_chunk")
                        .await?,
                );
                progress(index + 1, total);
            }
            let mut result = merge_deepseek_results(results)?;
            result.warnings = validate_result(text, &result.scene)?;
            return Ok(result);
        }

        progress(0, 1);
        let mut result = self
            .structure_script_once(&key, text, "structure_script")
            .await?;
        progress(1, 1);
        result.warnings = validate_result(text, &result.scene)?;
        Ok(result)
    }

    async fn structure_script_once(
        &self,
        key: &str,
        text: &str,
        operation: &'static str,
    ) -> AppResult<DeepSeekResult> {
        let url = format!("{}/chat/completions", self.api_base.trim_end_matches('/'));
        let body = json!({
            "model": self.model,
            "messages": [
                { "role": "system", "content": SYSTEM_PROMPT },
                { "role": "user", "content": user_prompt(text) }
            ],
            "response_format": { "type": "json_object" },
            "temperature": 0.1,
            "max_tokens": STRUCTURE_MAX_TOKENS
        });
        let payload = self
            .send_chat_completion_with_retries(&url, key, &body, operation)
            .await?;
        let content = payload
            .choices
            .first()
            .ok_or_else(|| AppError::Provider("DeepSeek no devolvió opciones".into()))?
            .message
            .content
            .trim();
        parse_deepseek_result(content)
    }

    async fn send_chat_completion_with_retries(
        &self,
        url: &str,
        key: &str,
        body: &serde_json::Value,
        operation: &'static str,
    ) -> AppResult<ChatCompletionResponse> {
        let mut last_error = None;
        for attempt in 1..=REQUEST_RETRIES + 1 {
            let response = self
                .http
                .post(url)
                .bearer_auth(key)
                .header(reqwest::header::ACCEPT_ENCODING, "identity")
                .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
                .json(body)
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(AppError::Provider(format!(
                    "DeepSeek respondió con estado HTTP {}",
                    response.status()
                )));
            }

            match parse_chat_completion_response(response, operation).await {
                Ok(payload) => return Ok(payload),
                Err(err) if is_unreadable_body_error(&err) && attempt <= REQUEST_RETRIES => {
                    tracing::warn!(
                        target: "deepseek",
                        operation,
                        attempt,
                        retries = REQUEST_RETRIES,
                        "Retrying DeepSeek request after unreadable response body"
                    );
                    last_error = Some(err);
                }
                Err(err) => return Err(err),
            }
        }

        Err(last_error
            .unwrap_or_else(|| AppError::internal("DeepSeek retry loop ended without result")))
    }
}

fn parse_deepseek_result(content: &str) -> AppResult<DeepSeekResult> {
    serde_json::from_str(content).map_err(|err| {
        tracing::warn!(
            target: "deepseek",
            content_chars = content.chars().count(),
            "DeepSeek returned invalid structured JSON: {err}"
        );
        AppError::Provider(format!(
            "DeepSeek devolvió JSON inválido o incompleto: {err}"
        ))
    })
}

fn deserialize_tts_tags<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        None | Some(serde_json::Value::Null) => Ok(Vec::new()),
        Some(serde_json::Value::String(tag)) => Ok(vec![tag]),
        Some(serde_json::Value::Array(tags)) => tags
            .into_iter()
            .map(|tag| match tag {
                serde_json::Value::String(tag) => Ok(tag),
                other => Err(D::Error::custom(format!(
                    "tts_tags contiene un valor no string: {other}"
                ))),
            })
            .collect(),
        Some(other) => Err(D::Error::custom(format!(
            "tts_tags debe ser string o array de strings, recibido: {other}"
        ))),
    }
}

async fn parse_chat_completion_response(
    response: reqwest::Response,
    operation: &'static str,
) -> AppResult<ChatCompletionResponse> {
    let status = response.status();
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("<missing>")
        .to_owned();
    let content_encoding = response
        .headers()
        .get(reqwest::header::CONTENT_ENCODING)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("<missing>")
        .to_owned();
    let content_length = response.content_length();
    let body = read_response_body(response, operation, status, &content_type, &content_encoding)
        .await
        .map_err(|err| {
            AppError::Provider(format!(
                "DeepSeek devolvió un body HTTP ilegible: {err} (status: {status}, content-type: {content_type}, content-encoding: {content_encoding}, content-length: {})",
                content_length
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "<missing>".into())
            ))
        })?;
    tracing::debug!(
        target: "deepseek",
        operation,
        %status,
        content_type = %content_type,
        content_encoding = %content_encoding,
        content_length = content_length
            .map(|value| value.to_string())
            .unwrap_or_else(|| "<missing>".into()),
        body_bytes = body.len(),
        "DeepSeek response received"
    );

    let value: serde_json::Value = serde_json::from_slice(&body).map_err(|err| {
        tracing::warn!(
            target: "deepseek",
            operation,
            %status,
            content_type = %content_type,
            content_encoding = %content_encoding,
            body_bytes = body.len(),
            "DeepSeek response body is not valid JSON: {err}"
        );
        AppError::Provider(format!("DeepSeek devolvió una respuesta no JSON: {err}"))
    })?;

    if let Some(object) = value.as_object() {
        let keys = object.keys().cloned().collect::<Vec<_>>().join(",");
        tracing::debug!(
            target: "deepseek",
            operation,
            top_level_keys = %keys,
            "DeepSeek response shape"
        );
    }

    let payload: ChatCompletionResponse = serde_json::from_value(value).map_err(|err| {
        tracing::warn!(
            target: "deepseek",
            operation,
            %status,
            content_type = %content_type,
            body_bytes = body.len(),
            "DeepSeek response has unexpected schema: {err}"
        );
        AppError::Provider(format!(
            "DeepSeek devolvió una respuesta con formato inesperado: {err}"
        ))
    })?;

    tracing::debug!(
        target: "deepseek",
        operation,
        choices = payload.choices.len(),
        first_finish_reason = payload
            .choices
            .first()
            .and_then(|choice| choice.finish_reason.as_deref())
            .unwrap_or("<missing>"),
        first_content_chars = payload
            .choices
            .first()
            .map(|choice| choice.message.content.chars().count())
            .unwrap_or(0),
        "DeepSeek chat completion parsed"
    );

    Ok(payload)
}

async fn read_response_body(
    mut response: reqwest::Response,
    operation: &'static str,
    status: StatusCode,
    content_type: &str,
    content_encoding: &str,
) -> Result<Vec<u8>, reqwest::Error> {
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|err| {
        tracing::warn!(
            target: "deepseek",
            operation,
            %status,
            content_type = %content_type,
            content_encoding = %content_encoding,
            partial_body_bytes = body.len(),
            "DeepSeek response body stream failed: {err}"
        );
        err
    })? {
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn is_unreadable_body_error(err: &AppError) -> bool {
    matches!(err, AppError::Provider(message) if message.starts_with("DeepSeek devolvió un body HTTP ilegible"))
}

fn split_text_chunks(text: &str, max_chars: usize) -> Vec<String> {
    if text.chars().count() <= max_chars {
        return vec![text.to_owned()];
    }

    let mut chunks = Vec::new();
    let mut current = String::new();
    for paragraph in text.split_inclusive('\n') {
        let paragraph_len = paragraph.chars().count();
        let current_len = current.chars().count();
        if !current.trim().is_empty() && current_len + paragraph_len > max_chars {
            chunks.push(current.trim().to_owned());
            current.clear();
        }
        current.push_str(paragraph);
    }

    if !current.trim().is_empty() {
        chunks.push(current.trim().to_owned());
    }
    chunks
}

fn merge_deepseek_results(results: Vec<DeepSeekResult>) -> AppResult<DeepSeekResult> {
    let mut results = results.into_iter();
    let first = results
        .next()
        .ok_or_else(|| AppError::Provider("DeepSeek no devolvió resultados".into()))?;

    let mut characters_by_name = BTreeMap::new();
    let mut scene = first.scene;
    for character in std::mem::take(&mut scene.characters) {
        characters_by_name.insert(character.name.trim().to_owned(), character);
    }
    scene.characters = Vec::new();
    let mut warnings = first.warnings;

    for result in results {
        for character in result.scene.characters {
            characters_by_name
                .entry(character.name.trim().to_owned())
                .or_insert(character);
        }
        scene.dialogues.extend(result.scene.dialogues);
        scene
            .unassigned_fragments
            .extend(result.scene.unassigned_fragments);
        warnings.extend(result.warnings);
    }

    scene.characters = characters_by_name.into_values().collect();
    warnings.sort();
    warnings.dedup();

    Ok(DeepSeekResult { scene, warnings })
}

fn deepseek_http_client() -> Client {
    Client::builder()
        .http1_only()
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd()
        .build()
        .expect("failed to build DeepSeek HTTP client")
}

pub fn validate_result(original_text: &str, scene: &StructuredScene) -> AppResult<Vec<String>> {
    if scene.title.trim().is_empty() {
        return Err(AppError::invalid("la escena estructurada no tiene título"));
    }
    if scene.language.trim().is_empty() {
        return Err(AppError::invalid("la escena estructurada no tiene idioma"));
    }
    if scene.characters.is_empty() {
        return Err(AppError::invalid("DeepSeek no devolvió personajes"));
    }
    if scene.dialogues.is_empty() {
        return Err(AppError::invalid("DeepSeek no devolvió diálogos"));
    }

    let character_names: std::collections::HashSet<&str> = scene
        .characters
        .iter()
        .map(|character| character.name.trim())
        .filter(|name| !name.is_empty())
        .collect();
    if character_names.is_empty() {
        return Err(AppError::invalid("DeepSeek devolvió personajes sin nombre"));
    }

    let normalized_original = normalize_text(original_text);
    let mut warnings = Vec::new();
    for (index, dialogue) in scene.dialogues.iter().enumerate() {
        let number = index + 1;
        if dialogue.speaker.trim().is_empty() {
            return Err(AppError::invalid(format!(
                "el diálogo {number} no tiene speaker"
            )));
        }
        if dialogue.text.trim().is_empty() {
            return Err(AppError::invalid(format!(
                "el diálogo {number} no tiene texto"
            )));
        }
        if !is_supported_dialogue_kind(&dialogue.kind) {
            return Err(AppError::invalid(format!(
                "el diálogo {number} tiene tipo inválido: {}",
                dialogue.kind
            )));
        }
        if !character_names.contains(dialogue.speaker.trim()) {
            warnings.push(format!(
                "el speaker '{}' no está en personajes y se creará automáticamente",
                dialogue.speaker.trim()
            ));
        }
        for tag in &dialogue.tts_tags {
            if !is_tts_tag(tag) {
                return Err(AppError::invalid(format!(
                    "el diálogo {number} tiene una etiqueta TTS inválida: {tag}"
                )));
            }
        }
        if let Some(excerpt) = &dialogue.original_excerpt {
            let normalized_excerpt = normalize_text(excerpt);
            if normalized_excerpt.is_empty() {
                warnings.push(format!("el diálogo {number} tiene original_excerpt vacío"));
            } else if !normalized_original.contains(&normalized_excerpt) {
                warnings.push(format!(
                    "el original_excerpt del diálogo {number} no aparece literalmente en el texto original"
                ));
            }
        } else {
            warnings.push(format!("el diálogo {number} no incluye original_excerpt"));
        }
    }

    if !scene.unassigned_fragments.is_empty() {
        warnings.push("DeepSeek devolvió fragmentos no asignados".into());
    }

    warnings.sort();
    warnings.dedup();
    Ok(warnings)
}

fn user_prompt(text: &str) -> String {
    format!(
        r#"Return a JSON object with exactly this top-level shape:
{{
  "scene": {{
    "title": "string",
    "description": "string or null",
    "language": "es-MX",
    "characters": [{{ "name": "Narrador", "role": "narrator", "aliases": [], "description": "string or null" }}],
    "dialogues": [{{ "speaker": "Narrador", "type": "narration", "tts_tags": ["[warm]"], "text": "Texto del bloque.", "original_excerpt": "Cita exacta breve copiada del texto original, máximo 160 caracteres." }}],
    "unassigned_fragments": []
  }}
}}

Keep the JSON compact. Do not repeat long paragraphs in original_excerpt; use a short exact excerpt only.
original_excerpt MUST be copied verbatim from the source text. Preserve dialogue dashes, quotes, punctuation, accents, and casing when present.

Script to structure:
{text}"#
    )
}

fn is_supported_dialogue_kind(kind: &str) -> bool {
    matches!(
        kind,
        "narration" | "dialogue" | "thought" | "system" | "direction"
    )
}

fn is_tts_tag(tag: &str) -> bool {
    let trimmed = tag.trim();
    trimmed.len() >= 3 && trimmed.starts_with('[') && trimmed.ends_with(']')
}

fn normalize_text(value: &str) -> String {
    normalize_validation_punctuation(&strip_inline_markdown(value))
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_validation_punctuation(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || ch.is_whitespace() {
                ch
            } else if is_ignorable_validation_punctuation(ch) {
                ' '
            } else {
                ch
            }
        })
        .collect()
}

fn is_ignorable_validation_punctuation(ch: char) -> bool {
    matches!(
        ch,
        '—' | '–'
            | '-'
            | '.'
            | ','
            | ':'
            | ';'
            | '!'
            | '?'
            | '¡'
            | '¿'
            | '"'
            | '\''
            | '“'
            | '”'
            | '‘'
            | '’'
            | '«'
            | '»'
            | '('
            | ')'
            | '['
            | ']'
    )
}

fn strip_inline_markdown(value: &str) -> String {
    let without_emphasis = strip_paired_delimiters(value, '*', 3);
    let without_underscores = strip_paired_delimiters(&without_emphasis, '_', 3);
    let without_strikethrough = strip_paired_delimiters(&without_underscores, '~', 2);
    strip_paired_delimiters(&without_strikethrough, '`', 3)
}

fn strip_paired_delimiters(value: &str, delimiter: char, max_run_len: usize) -> String {
    let chars: Vec<char> = value.chars().collect();
    let mut remove = vec![false; chars.len()];
    let mut index = 0;

    while index < chars.len() {
        let open_len = delimiter_run_len(&chars, index, delimiter).min(max_run_len);
        if open_len == 0 || !is_opening_delimiter(&chars, index, open_len) {
            index += open_len.max(1);
            continue;
        }

        let mut close_index = index + open_len;
        while close_index < chars.len() {
            let close_len = delimiter_run_len(&chars, close_index, delimiter).min(max_run_len);
            if close_len == open_len && is_closing_delimiter(&chars, close_index, close_len) {
                for offset in 0..open_len {
                    remove[index + offset] = true;
                    remove[close_index + offset] = true;
                }
                index = close_index + close_len;
                break;
            }
            close_index += close_len.max(1);
        }

        if close_index >= chars.len() {
            index += open_len;
        }
    }

    chars
        .into_iter()
        .enumerate()
        .filter_map(|(index, ch)| (!remove[index]).then_some(ch))
        .collect()
}

fn delimiter_run_len(chars: &[char], index: usize, delimiter: char) -> usize {
    chars[index..]
        .iter()
        .take_while(|&&ch| ch == delimiter)
        .count()
}

fn is_opening_delimiter(chars: &[char], index: usize, len: usize) -> bool {
    chars
        .get(index + len)
        .is_some_and(|next| !next.is_whitespace())
}

fn is_closing_delimiter(chars: &[char], index: usize, len: usize) -> bool {
    index > 0
        && !chars[index - 1].is_whitespace()
        && match chars.get(index + len) {
            Some(next) => !next.is_alphanumeric(),
            None => true,
        }
}
