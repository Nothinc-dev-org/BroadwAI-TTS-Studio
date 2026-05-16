import type { GeneratedAudio, SceneMixResult, TagsUpdate } from '~/types/domain'

export function useTts() {
  const { invoke } = useTauri()

  return {
    generateDialogue: (dialogueNodeId: string) =>
      invoke<GeneratedAudio>('generate_dialogue_audio', { dialogueNodeId }),
    playDialogue: (dialogueNodeId: string) =>
      invoke<GeneratedAudio>('play_dialogue_audio', { dialogueNodeId }),
    regenerateOutdated: (sceneId: string) =>
      invoke<GeneratedAudio[]>('regenerate_outdated_audio', { sceneId }),
    listForScene: (sceneId: string) =>
      invoke<GeneratedAudio[]>('list_generated_audio_for_scene', { sceneId }),
    generatedAudioBytes: (generatedAudioId: string) =>
      invoke<number[]>('generated_audio_bytes', { generatedAudioId }),
    previewVoice: (voiceProvider: string, voiceId: string, sampleText: string) =>
      invoke<string>('preview_voice_sample', { voiceProvider, voiceId, sampleText }),
    previewVoiceBytes: (voiceProvider: string, voiceId: string, sampleText: string) =>
      invoke<number[]>('preview_voice_sample_bytes', { voiceProvider, voiceId, sampleText }),
    generateScene: (sceneId: string) =>
      invoke<GeneratedAudio[]>('generate_scene_audio', { sceneId }),
    playScene: (sceneId: string) =>
      invoke<SceneMixResult>('play_scene_audio', { sceneId }),
    optimizeSceneTags: (sceneId: string) =>
      invoke<TagsUpdate[]>('optimize_scene_tts_tags', { sceneId }),
  }
}
