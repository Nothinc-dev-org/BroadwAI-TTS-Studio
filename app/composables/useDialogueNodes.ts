import type { DialogueNode, DialogueTtsTag } from '~/types/domain'

export function useDialogueNodes() {
  const { invoke } = useTauri()

  return {
    list: (sceneId: string) =>
      invoke<DialogueNode[]>('list_dialogue_nodes', { sceneId }),
    create: (params: {
      sceneId: string
      characterId: string
      kind: string
      text: string
      orderIndex: number
    }) => invoke<DialogueNode>('create_dialogue_node', params),
    update: (params: {
      id: string
      text?: string | null
      characterId?: string | null
      kind?: string | null
      emotion?: string | null
      intensity?: number | null
      isEnabled?: boolean | null
      beforeDelayMs?: number | null
      afterDelayMs?: number | null
    }) =>
      invoke<void>('update_dialogue_node', {
        id: params.id,
        text: params.text ?? null,
        characterId: params.characterId ?? null,
        kind: params.kind ?? null,
        emotion: params.emotion ?? null,
        intensity: params.intensity ?? null,
        isEnabled: params.isEnabled ?? null,
        beforeDelayMs: params.beforeDelayMs ?? null,
        afterDelayMs: params.afterDelayMs ?? null,
      }),
    remove: (id: string) => invoke<void>('delete_dialogue_node', { id }),
    split: (id: string, splitAt: number) =>
      invoke<void>('split_dialogue_node', { id, splitAt }),
    merge: (firstId: string, secondId: string) =>
      invoke<void>('merge_dialogue_nodes', { firstId, secondId }),
    reorder: (sceneId: string, orderedIds: string[]) =>
      invoke<void>('reorder_dialogue_nodes', { sceneId, orderedIds }),
    updateTags: (dialogueNodeId: string, tags: DialogueTtsTag[]) =>
      invoke<void>('update_dialogue_tts_tags', { dialogueNodeId, tags }),
  }
}
