<script setup lang="ts">
import type { Scene } from '~/types/domain'

const props = defineProps<{ scene: Scene }>()

const tts = useTts()
const loading = ref(false)
const error = ref<string | null>(null)

async function generateAll() {
  loading.value = true
  error.value = null
  try {
    await tts.generateScene(props.scene.id)
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    loading.value = false
  }
}

async function playAll() {
  loading.value = true
  error.value = null
  try {
    await tts.playScene(props.scene.id)
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="flex items-center justify-between rounded-lg border border-default px-4 py-3">
    <div>
      <h1 class="text-xl font-semibold">{{ scene.title }}</h1>
      <p v-if="scene.description" class="text-sm text-muted">{{ scene.description }}</p>
    </div>
    <div class="flex items-center gap-2">
      <UButton variant="soft" icon="i-lucide-wand-sparkles" :loading="loading" @click="generateAll">
        Generar todo
      </UButton>
      <UButton icon="i-lucide-play" :loading="loading" @click="playAll">
        Play global
      </UButton>
      <UButton variant="ghost" icon="i-lucide-download">
        Exportar
      </UButton>
    </div>
    <p v-if="error" class="ml-3 text-sm text-error">{{ error }}</p>
  </div>
</template>
