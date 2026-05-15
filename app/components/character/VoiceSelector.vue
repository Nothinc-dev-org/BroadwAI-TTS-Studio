<script setup lang="ts">
const provider = defineModel<string | null | undefined>('provider')
const voiceId = defineModel<string | null | undefined>('voiceId')

const providers = [
  { label: 'Gemini 3.1 Flash TTS', value: 'gemini' },
]
</script>

<template>
  <div class="grid grid-cols-2 gap-3">
    <UFormField label="Proveedor de voz">
      <USelect v-model="provider" class="w-full" :items="providers" />
    </UFormField>
    <UFormField label="Voice ID">
      <UInput v-model="voiceId" class="w-full" placeholder="ej. Aria" />
    </UFormField>
  </div>
</template>
