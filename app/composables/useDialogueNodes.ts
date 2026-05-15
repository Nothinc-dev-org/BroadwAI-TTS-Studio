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
    update: (params: Partial<DialogueNode> & { id: string }) =>
      invoke<void>('update_dialogue_node', params),
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
