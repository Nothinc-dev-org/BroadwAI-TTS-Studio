<script setup lang="ts">
import type { Character } from '~/types/domain'

const model = defineModel<Partial<Character>>({ required: true })
const emit = defineEmits<{ submit: [] }>()

const voiceProvider = computed({
  get: () => model.value.voice_provider ?? undefined,
  set: value => {
    model.value.voice_provider = value ?? null
  },
})

const voiceId = computed({
  get: () => model.value.voice_id ?? undefined,
  set: value => {
    model.value.voice_id = value ?? null
  },
})

const roleLabels: Record<string, string> = {
  narrator: 'Narrador',
  character: 'Personaje',
  system: 'Sistema',
}

const voiceSampleText = computed(() => [
  model.value.name?.trim(),
  model.value.role ? roleLabels[model.value.role] ?? model.value.role : undefined,
  model.value.description?.trim(),
].filter(Boolean).join('. '))
</script>

<template>
  <UForm :state="model" class="space-y-4" @submit.prevent="emit('submit')">
    <UFormField label="Nombre" required>
      <UInput v-model="model.name" class="w-full" />
    </UFormField>
    <UFormField label="Rol" required>
      <USelect
        v-model="model.role"
        class="w-full"
        :items="[
          { label: 'Narrador', value: 'narrator' },
          { label: 'Personaje', value: 'character' },
          { label: 'Sistema', value: 'system' },
        ]"
      />
    </UFormField>
    <UFormField label="Descripción">
      <UTextarea v-model="model.description" class="w-full" :rows="2" />
    </UFormField>
    <UFormField label="Color">
      <UInput v-model="model.color" class="w-full" type="color" />
    </UFormField>
    <VoiceSelector v-model:voice-id="voiceId" v-model:provider="voiceProvider" :sample-text="voiceSampleText" />
    <slot name="actions" />
  </UForm>
</template>
