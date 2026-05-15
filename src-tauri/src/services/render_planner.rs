//! RenderPlanner — decide cómo dividir la escena en jobs de TTS (RF-29).

use sha2::{Digest, Sha256};

use crate::entities::dialogue_node;

#[derive(Debug, Clone)]
pub struct DialogueRenderInput<'a> {
    pub node: &'a dialogue_node::Model,
    pub voice_id: &'a str,
    pub model: &'a str,
    pub tag_signature: &'a str,
    pub style_prompt: Option<&'a str>,
}

/// Calcula el hash determinístico de un job de TTS para detectar caché vigente
/// (RF-30).
pub fn input_hash(input: &DialogueRenderInput<'_>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.node.text.as_bytes());
    hasher.update(b"|");
    hasher.update(input.voice_id.as_bytes());
    hasher.update(b"|");
    hasher.update(input.model.as_bytes());
    hasher.update(b"|");
    hasher.update(input.tag_signature.as_bytes());
    hasher.update(b"|");
    if let Some(prompt) = input.style_prompt {
        hasher.update(prompt.as_bytes());
    }
    hex::encode(hasher.finalize())
}
