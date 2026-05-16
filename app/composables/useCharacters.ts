import type { Character, CharacterAlias } from '~/types/domain'

export function useCharacters() {
  const { invoke } = useTauri()

  return {
    list: (projectId: string) =>
      invoke<Character[]>('list_characters', { projectId }),
    create: (params: {
      projectId: string
      name: string
      role: string
      description?: string
      color?: string
      voiceProvider?: string
      voiceId?: string
      defaultStylePrompt?: string
    }) =>
      invoke<Character>('create_character', {
        projectId: params.projectId,
        name: params.name,
        role: params.role,
        description: params.description ?? null,
        color: params.color ?? null,
        voiceProvider: params.voiceProvider ?? null,
        voiceId: params.voiceId ?? null,
        defaultStylePrompt: params.defaultStylePrompt ?? null,
      }),
    update: (params: {
      id: string
      name?: string
      role?: string
      description?: string
      color?: string
    }) => invoke<void>('update_character', {
      id: params.id,
      name: params.name ?? null,
      role: params.role ?? null,
      description: params.description ?? null,
      color: params.color ?? null,
    }),
    remove: (id: string) => invoke<void>('delete_character', { id }),
    addAlias: (characterId: string, alias: string) =>
      invoke<CharacterAlias>('add_character_alias', { characterId, alias }),
    removeAlias: (aliasId: string) =>
      invoke<void>('remove_character_alias', { aliasId }),
    assignVoice: (params: {
      characterId: string
      voiceProvider: string
      voiceId: string
      defaultStylePrompt?: string
    }) =>
      invoke<void>('assign_character_voice', {
        characterId: params.characterId,
        voiceProvider: params.voiceProvider,
        voiceId: params.voiceId,
        defaultStylePrompt: params.defaultStylePrompt ?? null,
      }),
  }
}
