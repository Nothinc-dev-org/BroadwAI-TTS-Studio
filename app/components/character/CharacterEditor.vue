<script setup lang="ts">
import type { Character } from '~/types/domain'

const model = defineModel<Partial<Character>>({ required: true })
const emit = defineEmits<{ submit: [] }>()
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
    <VoiceSelector v-model:voice-id="model.voice_id" v-model:provider="model.voice_provider" />
    <slot name="actions" />
  </UForm>
</template>
