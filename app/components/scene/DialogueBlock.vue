<script setup lang="ts">
import type { Character, DialogueNode, GeneratedAudio } from '~/types/domain'

const props = defineProps<{
  dialogue: DialogueNode
  characters: Character[]
  audio?: GeneratedAudio | null
}>()

const emit = defineEmits<{
  generated: [audio: GeneratedAudio]
}>()

const tts = useTts()

const character = computed(() =>
  props.characters.find(c => c.id === props.dialogue.character_id),
)

const speaker = computed(() => character.value?.name ?? 'Sin asignar')

const characterHref = computed(() =>
  character.value ? `#char-${character.value.id}` : undefined,
)

const kindLabel: Record<string, string> = {
  narration: 'Narración',
  dialogue: 'Diálogo',
  thought: 'Pensamiento',
  system: 'Sistema',
  direction: 'Acotación',
}

const localAudio = ref<GeneratedAudio | null>(props.audio ?? null)
watch(() => props.audio, value => {
  localAudio.value = value ?? null
  if (value?.id !== objectAudioId) {
    revokeAudioBlob()
  }
})

const audioSrc = ref<string | null>(null)
let objectUrl: string | null = null
let objectAudioId: string | null = null

const audioRef = ref<HTMLAudioElement | null>(null)
const isPlaying = ref(false)
const isBusy = ref(false)
const lastError = ref<string | null>(null)

type StatusKey = 'missing' | 'pending' | 'generated' | 'outdated' | 'failed'

const status = computed<StatusKey>(() => {
  if (isBusy.value) return 'pending'
  if (!localAudio.value) return 'missing'
  switch (localAudio.value.status) {
    case 'generated': return 'generated'
    case 'outdated': return 'outdated'
    case 'failed': return 'failed'
    default: return 'pending'
  }
})

const statusMeta: Record<StatusKey, { label: string, color: 'neutral' | 'info' | 'success' | 'warning' | 'error' }> = {
  missing: { label: 'Sin generar', color: 'neutral' },
  pending: { label: 'Generando…', color: 'info' },
  generated: { label: 'Generado', color: 'success' },
  outdated: { label: 'Desactualizado', color: 'warning' },
  failed: { label: 'Error', color: 'error' },
}

async function ensureAudio(force: boolean): Promise<GeneratedAudio | null> {
  isBusy.value = true
  lastError.value = null
  try {
    const result = force
      ? await tts.generateDialogue(props.dialogue.id)
      : await tts.playDialogue(props.dialogue.id)
    localAudio.value = result
    emit('generated', result)
    return result
  }
  catch (err) {
    lastError.value = String(err)
    return null
  }
  finally {
    isBusy.value = false
  }
}

async function onPlay() {
  if (isPlaying.value) {
    audioRef.value?.pause()
    return
  }

  const needsFresh = !localAudio.value || localAudio.value.status !== 'generated'
  if (needsFresh) {
    const result = await ensureAudio(false)
    if (!result) return
  }

  if (!localAudio.value) return

  try {
    await loadAudioBlob(localAudio.value)
  }
  catch (err) {
    lastError.value = String(err)
    return
  }

  await nextTick()
  const el = audioRef.value
  if (!el) return
  el.currentTime = 0
  try {
    el.load()
    await el.play()
  }
  catch (err) {
    lastError.value = String(err)
  }
}

async function onRegenerate() {
  await ensureAudio(true)
}

async function loadAudioBlob(audio: GeneratedAudio) {
  if (objectAudioId === audio.id && audioSrc.value) return
  const bytes = await tts.generatedAudioBytes(audio.id)
  revokeAudioBlob()
  objectUrl = URL.createObjectURL(new Blob([new Uint8Array(bytes)], { type: 'audio/wav' }))
  objectAudioId = audio.id
  audioSrc.value = objectUrl
}

function revokeAudioBlob() {
  if (objectUrl) {
    URL.revokeObjectURL(objectUrl)
  }
  objectUrl = null
  objectAudioId = null
  audioSrc.value = null
}

function onAudioError() {
  const audio = audioRef.value
  const code = audio?.error?.code ?? 'sin código'
  const message = audio?.error?.message || 'sin detalle del WebView'
  lastError.value = `No se pudo cargar el audio generado. Código: ${code}. ${message}. URL: ${audio?.currentSrc || audioSrc.value || 'desconocida'}`
}

onBeforeUnmount(revokeAudioBlob)
</script>

<template>
  <article class="rounded-lg border border-default bg-elevated p-3">
    <header class="mb-2 flex items-center justify-between gap-2">
      <div class="flex items-center gap-2 text-sm">
        <UBadge color="neutral" variant="subtle">{{ kindLabel[dialogue.type] ?? dialogue.type }}</UBadge>
        <a
          v-if="characterHref"
          :href="characterHref"
          class="font-medium hover:underline"
        >{{ speaker }}</a>
        <span v-else class="font-medium">{{ speaker }}</span>
        <UBadge :color="statusMeta[status].color" variant="subtle" size="xs">
          {{ statusMeta[status].label }}
        </UBadge>
      </div>
      <div class="flex items-center gap-1">
        <UButton
          size="xs"
          variant="ghost"
          :icon="isPlaying ? 'i-lucide-pause' : 'i-lucide-play'"
          :loading="isBusy"
          @click="onPlay"
        />
        <UButton
          size="xs"
          variant="ghost"
          icon="i-lucide-refresh-cw"
          :disabled="isBusy"
          :title="status === 'outdated' ? 'Audio desactualizado: regenerar' : 'Regenerar audio'"
          @click="onRegenerate"
        />
        <UButton size="xs" variant="ghost" icon="i-lucide-scissors" disabled />
        <UButton size="xs" variant="ghost" icon="i-lucide-trash-2" disabled />
      </div>
    </header>

    <p class="whitespace-pre-wrap text-sm leading-relaxed">{{ dialogue.text }}</p>

    <p v-if="lastError" class="mt-2 text-xs text-error">{{ lastError }}</p>

      <audio
        v-if="audioSrc"
        ref="audioRef"
        :src="audioSrc"
        preload="none"
        @play="isPlaying = true"
        @pause="isPlaying = false"
        @ended="isPlaying = false"
        @error="onAudioError"
      />
  </article>
</template>
