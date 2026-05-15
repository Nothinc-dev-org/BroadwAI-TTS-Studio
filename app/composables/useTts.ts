export function useTts() {
  const { invoke } = useTauri()

  return {
    generateDialogue: (dialogueNodeId: string) =>
      invoke<void>('generate_dialogue_audio', { dialogueNodeId }),
    generateScene: (sceneId: string) =>
      invoke<void>('generate_scene_audio', { sceneId }),
    regenerateOutdated: (sceneId: string) =>
      invoke<void>('regenerate_outdated_audio', { sceneId }),
    playDialogue: (dialogueNodeId: string) =>
      invoke<void>('play_dialogue_audio', { dialogueNodeId }),
    playScene: (sceneId: string) => invoke<void>('play_scene_audio', { sceneId }),
  }
}
