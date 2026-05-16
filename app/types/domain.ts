// Tipos espejo de las entidades Rust en src-tauri/src/entities.
// Cualquier cambio en columnas SeaORM debe replicarse aquí.

export interface Project {
  id: string
  title: string
  description: string | null
  language: string
  root_path: string
  created_at: string
  updated_at: string
}

export interface Scene {
  id: string
  project_id: string
  title: string
  description: string | null
  order_index: number
  created_at: string
  updated_at: string
}

export interface StructuredCharacter {
  name: string
  role: string
  aliases: string[]
  description: string | null
}

export interface StructuredDialogue {
  speaker: string
  type: string
  tts_tags: string[]
  text: string
  original_excerpt: string | null
}

export interface StructuredScene {
  title: string
  description: string | null
  language: string
  characters: StructuredCharacter[]
  dialogues: StructuredDialogue[]
  unassigned_fragments: string[]
}

export interface DeepSeekResult {
  scene: StructuredScene
  warnings: string[]
}

export type CharacterRole = 'narrator' | 'character' | 'system'

export interface Character {
  id: string
  project_id: string
  name: string
  role: string
  description: string | null
  color: string | null
  voice_provider: string | null
  voice_id: string | null
  default_style_prompt: string | null
  created_at: string
  updated_at: string
}

export interface CharacterAlias {
  id: string
  character_id: string
  alias: string
}

export type DialogueKind = 'narration' | 'dialogue' | 'thought' | 'system' | 'direction'

export interface DialogueNode {
  id: string
  scene_id: string
  character_id: string
  previous_id: string | null
  next_id: string | null
  order_index: number
  type: string
  text: string
  raw_text: string | null
  emotion: string | null
  intensity: number | null
  is_enabled: number
  before_delay_ms: number | null
  after_delay_ms: number | null
  created_at: string
  updated_at: string
}

export type TtsTagPosition = 'prefix' | 'inline' | 'suffix'
export type TtsTagSource = 'ai' | 'manual'

export interface DialogueTtsTag {
  id: string
  dialogue_node_id: string
  tag: string
  position: TtsTagPosition
  order_index: number
  source: TtsTagSource
}

export type AudioAssetKind = 'sound_effect' | 'music' | 'ambience' | 'voice' | 'generated'

export interface AudioAsset {
  id: string
  project_id: string
  name: string
  type: string
  file_path: string
  duration_ms: number | null
  original_file_name: string | null
  mime_type: string | null
  created_at: string
}

export type GeneratedAudioStatus = 'pending' | 'generated' | 'failed' | 'outdated'

export interface GeneratedAudio {
  id: string
  dialogue_node_id: string
  provider: string
  model: string
  voice_id: string
  input_hash: string
  file_path: string
  duration_ms: number | null
  status: string
  error_message: string | null
  created_at: string
}

export type TimelineTrackKind = 'voice' | 'sfx' | 'music' | 'ambience'

export interface TimelineTrack {
  id: string
  scene_id: string
  name: string
  type: string
  order_index: number
  volume: number
  muted: number
  solo: number
}

export interface TimelineEvent {
  id: string
  scene_id: string
  track_id: string
  dialogue_node_id: string | null
  audio_asset_id: string | null
  generated_audio_id: string | null
  start_ms: number
  duration_ms: number | null
  offset_ms: number | null
  volume: number
  fade_in_ms: number | null
  fade_out_ms: number | null
  playback_rate: number
  loop: number
  created_at: string
  updated_at: string
}

export type RenderJobStatus = 'queued' | 'running' | 'completed' | 'failed' | 'cancelled'

export interface RenderJob {
  id: string
  scene_id: string | null
  dialogue_node_id: string | null
  type: string
  provider: string
  model: string | null
  status: string
  input_payload: string
  output_path: string | null
  error_message: string | null
  created_at: string
  updated_at: string
}

export type ApiProvider = 'deepseek' | 'gemini'
export type ApiKeyStatus = 'not_configured' | 'configured' | 'valid' | 'invalid' | 'connection_error'

export interface SceneMixResult {
  output_path: string
  duration_ms: number
}

export type ExportFormat = 'wav' | 'mp3'

export interface TagsUpdate {
  id: string
  tags: string[]
}
