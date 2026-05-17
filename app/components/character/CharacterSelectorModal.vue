<script setup lang="ts">
import type { Character } from '~/types/domain'

const props = defineProps<{
  characters: Character[]
  modelValue: boolean
  selectedCharacterId?: string | null
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  confirm: [characterId: string]
}>()

const selectedId = ref<string | null>(props.selectedCharacterId ?? null)

watch(() => props.selectedCharacterId, (value) => {
  selectedId.value = value ?? null
})

watch(() => props.modelValue, (open) => {
  if (open) {
    selectedId.value = props.selectedCharacterId ?? null
  }
})

function select(characterId: string) {
  selectedId.value = characterId
}

function close() {
  emit('update:modelValue', false)
}

function confirm() {
  if (selectedId.value) {
    emit('confirm', selectedId.value)
  }
  close()
}
</script>

<template>
  <UModal
    :open="modelValue"
    title="Cambiar personaje"
    description="Selecciona el personaje que habla este diálogo."
    @update:open="$emit('update:modelValue', $event)"
  >
    <template #body>
      <div
        v-if="characters.length"
        class="grid gap-2"
        style="grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));"
      >
        <button
          v-for="character in characters"
          :key="character.id"
          type="button"
          class="flex flex-col items-start gap-1.5 rounded-md border p-3 text-left transition"
          :class="selectedId === character.id
            ? 'border-primary bg-primary/5 ring-2 ring-primary/30'
            : 'border-default hover:border-primary/50 hover:bg-elevated/50'"
          @click="select(character.id)"
        >
          <div class="truncate text-sm font-medium">
            {{ character.name }}
          </div>
          <div class="truncate text-xs text-muted">
            {{ character.role }}
          </div>
          <UBadge
            v-if="character.voice_id"
            color="primary"
            variant="subtle"
            size="sm"
            class="mt-0.5 truncate"
          >
            {{ character.voice_id }}
          </UBadge>
        </button>
      </div>
      <div v-else class="rounded-md border border-dashed border-default p-4 text-center text-xs text-muted">
        No hay personajes en este proyecto.
      </div>
    </template>

    <template #footer>
      <div class="flex w-full items-center justify-end gap-2">
        <UButton variant="ghost" @click="close">
          Cerrar
        </UButton>
        <UButton
          color="primary"
          :disabled="!selectedId"
          @click="confirm"
        >
          Continuar
        </UButton>
      </div>
    </template>
  </UModal>
</template>
