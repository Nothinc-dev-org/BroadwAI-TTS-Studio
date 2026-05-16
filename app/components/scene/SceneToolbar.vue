<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import { save as saveDialog } from '@tauri-apps/plugin-dialog'
import type { DialogueNode, GeneratedAudio, Scene } from '~/types/domain'

const props = defineProps<{
  scene: Scene
  selectedDialogues?: DialogueNode[]
}>()

const emit = defineEmits<{
  audioGenerated: [audio: GeneratedAudio]
}>()

const tts = useTts()
const timeline = useTimeline()

const generating = ref(false)
const playing = ref(false)
const playingSelection = ref(false)
const exporting = ref(false)
const optimizing = ref(false)
const status = ref<string | null>(null)
const error = ref<string | null>(null)

const mixSrc = ref<string | null>(null)
const audioRef = ref<HTMLAudioElement | null>(null)
const selectionAudioRef = ref<HTMLAudioElement | null>(null)
const isAudioPlaying = ref(false)
const selectionSrc = ref<string | null>(null)
let selectionObjectUrl: string | null = null

function formatDuration(ms: number): string {
  const total = Math.round(ms / 1000)
  const m = Math.floor(total / 60)
  const s = total % 60
  return `${m}:${String(s).padStart(2, '0')}`
}

async function generateAll() {
  generating.value = true
  error.value = null
  status.value = null
  try {
    const audios = await tts.generateScene(props.scene.id)
    status.value = `Audios listos: ${audios.length}`
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    generating.value = false
  }
}

async function playAll() {
  playing.value = true
  error.value = null
  status.value = null
  try {
    const result = await tts.playScene(props.scene.id)
    mixSrc.value = convertFileSrc(result.output_path)
    status.value = `Mezcla (${formatDuration(result.duration_ms)}) lista`
    await nextTick()
    if (audioRef.value) {
      audioRef.value.currentTime = 0
      await audioRef.value.play()
    }
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    playing.value = false
  }
}

async function playSelection() {
  if (!props.selectedDialogues?.length) return
  playingSelection.value = true
  error.value = null
  status.value = null
  try {
    let completed = 0
    for (const dialogue of props.selectedDialogues) {
      if (!playingSelection.value) break
      status.value = `Reproduciendo selección ${completed + 1}/${props.selectedDialogues.length}`
      const audio = await tts.playDialogue(dialogue.id)
      emit('audioGenerated', audio)
      const bytes = await tts.generatedAudioBytes(audio.id)
      await playAudioBytes(bytes)
      completed += 1
    }
    if (completed === props.selectedDialogues.length) {
      status.value = `Selección reproducida: ${completed} bloque(s)`
    }
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    playingSelection.value = false
    revokeSelectionBlob()
  }
}

async function playAudioBytes(bytes: number[]) {
  revokeSelectionBlob()
  selectionObjectUrl = URL.createObjectURL(new Blob([new Uint8Array(bytes)], { type: 'audio/wav' }))
  selectionSrc.value = selectionObjectUrl
  await nextTick()
  const el = selectionAudioRef.value
  if (!el) return
  el.currentTime = 0
  el.load()
  await new Promise<void>((resolve, reject) => {
    el.onended = () => resolve()
    el.onerror = () => reject(new Error('No se pudo reproducir un bloque seleccionado'))
    void el.play().catch(reject)
  })
}

function revokeSelectionBlob() {
  if (selectionObjectUrl) {
    URL.revokeObjectURL(selectionObjectUrl)
  }
  selectionObjectUrl = null
  selectionSrc.value = null
}

async function optimizeTags() {
  optimizing.value = true
  error.value = null
  status.value = null
  try {
    const updates = await tts.optimizeSceneTags(props.scene.id)
    status.value = updates.length
      ? `Tags actualizados en ${updates.length} bloque(s). Audios marcados como desactualizados.`
      : 'DeepSeek no sugirió cambios.'
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    optimizing.value = false
  }
}

async function exportMix() {
  exporting.value = true
  error.value = null
  status.value = null
  try {
    const target = await saveDialog({
      title: 'Exportar mezcla',
      defaultPath: `${props.scene.title || 'escena'}.wav`,
      filters: [{ name: 'WAV', extensions: ['wav'] }],
    })
    if (!target) return
    const result = await timeline.render(props.scene.id, target, 'wav')
    status.value = `Exportado a ${result.output_path} (${formatDuration(result.duration_ms)})`
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    exporting.value = false
  }
}

function togglePlayback() {
  if (!audioRef.value) return
  if (isAudioPlaying.value) audioRef.value.pause()
  else void audioRef.value.play()
}

onBeforeUnmount(revokeSelectionBlob)
</script>

<template>
  <div class="space-y-2">
    <div class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-default px-4 py-3">
      <div class="min-w-0">
        <h1 class="truncate text-xl font-semibold">{{ scene.title }}</h1>
        <p v-if="scene.description" class="truncate text-sm text-muted">{{ scene.description }}</p>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        <UButton
          variant="soft"
          icon="i-lucide-wand-sparkles"
          :loading="generating"
          :disabled="playing || exporting || optimizing"
          @click="generateAll"
        >
          Generar todo
        </UButton>
        <UButton
          variant="ghost"
          icon="i-lucide-sparkles"
          :loading="optimizing"
          :disabled="generating || playing || exporting"
          title="DeepSeek: optimizar etiquetas TTS sin tocar el texto"
          @click="optimizeTags"
        >
          Optimizar TTS
        </UButton>
        <UButton
          icon="i-lucide-play"
          :loading="playing"
          :disabled="generating || playingSelection || exporting"
          @click="playAll"
        >
          Play global
        </UButton>
        <UButton
          v-if="selectedDialogues?.length"
          variant="soft"
          icon="i-lucide-list-video"
          :loading="playingSelection"
          :disabled="generating || playing || exporting || optimizing"
          @click="playSelection"
        >
          Reproducir Selección ({{ selectedDialogues.length }})
        </UButton>
        <UButton
          variant="ghost"
          icon="i-lucide-download"
          :loading="exporting"
          :disabled="generating || playing || playingSelection"
          @click="exportMix"
        >
          Exportar
        </UButton>
      </div>
    </div>

    <div v-if="mixSrc" class="flex items-center gap-2 rounded-lg border border-default px-4 py-2">
      <UButton
        size="xs"
        variant="soft"
        :icon="isAudioPlaying ? 'i-lucide-pause' : 'i-lucide-play'"
        @click="togglePlayback"
      />
      <audio
        ref="audioRef"
        :src="mixSrc"
        preload="auto"
        controls
        class="h-8 flex-1"
        @play="isAudioPlaying = true"
        @pause="isAudioPlaying = false"
        @ended="isAudioPlaying = false"
      />
    </div>

    <p v-if="status" class="text-xs text-success">{{ status }}</p>
    <p v-if="error" class="text-xs text-error">{{ error }}</p>
    <audio v-if="selectionSrc" ref="selectionAudioRef" :src="selectionSrc" preload="none" />
  </div>
</template>
