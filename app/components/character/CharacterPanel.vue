<script setup lang="ts">
import type { Character } from '~/types/domain'

withDefaults(
  defineProps<{
    characters: Character[]
    compact?: boolean
  }>(),
  { compact: false },
)

const emit = defineEmits<{
  changed: []
}>()

const charactersApi = useCharacters()
const editingCharacter = ref<Partial<Character> | null>(null)
const saving = ref(false)
const error = ref<string | null>(null)

const editorOpen = computed({
  get: () => Boolean(editingCharacter.value),
  set: value => {
    if (!value) {
      editingCharacter.value = null
      error.value = null
    }
  },
})

function openEditor(character: Character) {
  editingCharacter.value = {
    ...character,
    description: character.description ?? '',
    color: character.color ?? '#94a3b8',
    voice_provider: character.voice_provider ?? 'gemini',
    voice_id: character.voice_id ?? '',
  }
  error.value = null
}

async function saveCharacter() {
  const character = editingCharacter.value
  if (!character?.id || !character.name || !character.role) return

  const voiceProvider = character.voice_provider?.trim()
  const voiceId = character.voice_id?.trim()
  if ((voiceProvider || voiceId) && (!voiceProvider || !voiceId)) {
    error.value = 'Para asignar voz, completa proveedor y Voice ID.'
    return
  }

  saving.value = true
  error.value = null
  try {
    await charactersApi.update({
      id: character.id,
      name: character.name,
      role: character.role,
      description: character.description ?? '',
      color: character.color ?? undefined,
    })
    if (voiceProvider && voiceId) {
      await charactersApi.assignVoice({
        characterId: character.id,
        voiceProvider,
        voiceId,
        defaultStylePrompt: character.default_style_prompt ?? undefined,
      })
    }
    editingCharacter.value = null
    emit('changed')
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    saving.value = false
  }
}
</script>

<template>
  <section>
    <header class="mb-2 flex items-center justify-between" :class="{ 'mb-3': !compact }">
      <h3 :class="compact ? 'text-sm font-semibold' : 'text-base font-semibold'">Personajes</h3>
      <UButton size="xs" variant="soft" icon="i-lucide-plus">Nuevo</UButton>
    </header>

    <div v-if="characters.length" class="grid gap-2" style="grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));">
      <div
        v-for="character in characters"
        :key="character.id"
        :id="`char-${character.id}`"
        class="flex items-start justify-between gap-2 rounded-md border border-default p-3"
      >
        <div class="min-w-0">
          <div class="truncate text-sm font-medium">{{ character.name }}</div>
          <div class="truncate text-xs text-muted">{{ character.role }}</div>
          <UBadge v-if="character.voice_id" color="primary" variant="subtle" size="sm" class="mt-1 truncate">
            {{ character.voice_id }}
          </UBadge>
        </div>
        <UButton
          size="xs"
          variant="ghost"
          icon="i-lucide-pencil"
          title="Editar personaje"
          @click="openEditor(character)"
        />
      </div>
    </div>
    <div v-else class="rounded-md border border-dashed border-default p-4 text-center text-xs text-muted">
      Aún no hay personajes.
    </div>

    <UModal v-model:open="editorOpen" title="Editar personaje">
      <template #body>
        <UAlert
          v-if="error"
          icon="i-lucide-circle-alert"
          color="error"
          variant="soft"
          :description="error"
          class="mb-4"
        />
        <CharacterEditor
          v-if="editingCharacter"
          v-model="editingCharacter"
          @submit="saveCharacter"
        >
          <template #actions>
            <div class="flex justify-end gap-2 pt-2">
              <UButton variant="ghost" :disabled="saving" @click="editorOpen = false">Cancelar</UButton>
              <UButton type="submit" :loading="saving" :disabled="!editingCharacter.name || !editingCharacter.role">
                Guardar
              </UButton>
            </div>
          </template>
        </CharacterEditor>
      </template>
    </UModal>
  </section>
</template>
