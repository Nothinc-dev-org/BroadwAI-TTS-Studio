<script setup lang="ts">
const model = defineModel<{
  title: string
  description: string
  language: string
  rootPath: string
}>({ required: true })

const emit = defineEmits<{ submit: [] }>()
</script>

<template>
  <UForm :state="model" class="space-y-4" @submit.prevent="emit('submit')">
    <UFormField label="Título" required>
      <UInput v-model="model.title" class="w-full" placeholder="Capítulo 1" />
    </UFormField>
    <UFormField label="Descripción">
      <UTextarea v-model="model.description" class="w-full" :rows="2" />
    </UFormField>
    <UFormField label="Idioma">
      <UInput v-model="model.language" class="w-full" placeholder="es-MX" />
    </UFormField>
    <UFormField label="Carpeta">
      <UInput v-model="model.rootPath" class="w-full" readonly />
    </UFormField>
    <slot name="actions" />
  </UForm>
</template>
