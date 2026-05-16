<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    sampleText?: string
  }>(),
  { sampleText: 'Esta es una muestra de voz para BroadwAI TTS Studio.' },
)

const provider = defineModel<string | undefined>('provider')
const voiceId = defineModel<string | undefined>('voiceId')
const tts = useTts()

const providers = [
  { label: 'Gemini 3.1 Flash TTS', value: 'gemini' },
]

const geminiVoices = [
  { label: 'Aoede', value: 'Aoede' },
  { label: 'Achernar', value: 'Achernar' },
  { label: 'Achird', value: 'Achird' },
  { label: 'Algenib', value: 'Algenib' },
  { label: 'Algieba', value: 'Algieba' },
  { label: 'Alnilam', value: 'Alnilam' },
  { label: 'Autonoe', value: 'Autonoe' },
  { label: 'Callirhoe', value: 'Callirhoe' },
  { label: 'Charon', value: 'Charon' },
  { label: 'Despina', value: 'Despina' },
  { label: 'Enceladus', value: 'Enceladus' },
  { label: 'Erinome', value: 'Erinome' },
  { label: 'Fenrir', value: 'Fenrir' },
  { label: 'Gacrux', value: 'Gacrux' },
  { label: 'Iapetus', value: 'Iapetus' },
  { label: 'Kore', value: 'Kore' },
  { label: 'Laomedeia', value: 'Laomedeia' },
  { label: 'Leda', value: 'Leda' },
  { label: 'Orus', value: 'Orus' },
  { label: 'Pulcherrima', value: 'Pulcherrima' },
  { label: 'Puck', value: 'Puck' },
  { label: 'Rasalgethi', value: 'Rasalgethi' },
  { label: 'Sadachbia', value: 'Sadachbia' },
  { label: 'Sadaltager', value: 'Sadaltager' },
  { label: 'Schedar', value: 'Schedar' },
  { label: 'Sulafat', value: 'Sulafat' },
  { label: 'Umbriel', value: 'Umbriel' },
  { label: 'Vindemiatrix', value: 'Vindemiatrix' },
  { label: 'Zephyr', value: 'Zephyr' },
  { label: 'Zubenelgenubi', value: 'Zubenelgenubi' },
]

const voiceItems = computed(() => {
  if (provider.value === 'gemini') return geminiVoices
  return []
})

const audioRef = ref<HTMLAudioElement | null>(null)
const audioSrc = ref<string | null>(null)
let objectUrl: string | null = null
const previewing = ref(false)
const previewError = ref<string | null>(null)
const isPlaying = ref(false)

const canPreview = computed(() =>
  Boolean(provider.value && voiceId.value && props.sampleText.trim()),
)

async function playPreview() {
  if (!provider.value || !voiceId.value) return

  previewing.value = true
  previewError.value = null
  try {
    const bytes = await tts.previewVoiceBytes(provider.value, voiceId.value, props.sampleText)
    if (objectUrl) {
      URL.revokeObjectURL(objectUrl)
    }
    objectUrl = URL.createObjectURL(new Blob([new Uint8Array(bytes)], { type: 'audio/wav' }))
    audioSrc.value = objectUrl
    await nextTick()
    const audio = audioRef.value
    if (audio) {
      audio.load()
      await audio.play()
    }
  }
  catch (err) {
    previewError.value = String(err)
  }
  finally {
    previewing.value = false
  }
}

function onAudioError() {
  const audio = audioRef.value
  const code = audio?.error?.code ?? 'sin código'
  const message = audio?.error?.message || 'sin detalle del WebView'
  previewError.value = `No se pudo cargar la muestra generada. Código: ${code}. ${message}. URL: ${audio?.currentSrc || audioSrc.value || 'desconocida'}`
}

onBeforeUnmount(() => {
  if (objectUrl) {
    URL.revokeObjectURL(objectUrl)
  }
})
</script>

<template>
  <div class="grid grid-cols-2 gap-3">
    <UFormField label="Proveedor de voz">
      <USelect v-model="provider" class="w-full" :items="providers" />
    </UFormField>
    <UFormField label="Voz">
      <div class="flex gap-2">
        <UInputMenu
          v-model="voiceId"
          class="w-full"
          :items="voiceItems"
          value-key="value"
          placeholder="Selecciona una voz"
        />
        <UButton
          icon="i-lucide-play"
          variant="soft"
          :disabled="!canPreview || previewing || isPlaying"
          :loading="previewing"
          title="Reproducir muestra"
          @click="playPreview"
        />
      </div>
      <p v-if="previewError" class="mt-1 text-xs text-error">{{ previewError }}</p>
      <audio v-if="audioSrc" ref="audioRef" :src="audioSrc" preload="none" @play="isPlaying = true" @pause="isPlaying = false" @ended="isPlaying = false" @error="onAudioError" />
    </UFormField>
  </div>
</template>
