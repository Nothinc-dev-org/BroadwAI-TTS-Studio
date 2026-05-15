<script setup lang="ts">
defineProps<{
  src?: string
  label?: string
}>()

const audioRef = ref<HTMLAudioElement | null>(null)
const playing = ref(false)

function toggle() {
  if (!audioRef.value) return
  if (playing.value) {
    audioRef.value.pause()
  }
  else {
    void audioRef.value.play()
  }
}
</script>

<template>
  <div class="flex items-center gap-2">
    <UButton
      size="xs"
      variant="soft"
      :icon="playing ? 'i-lucide-pause' : 'i-lucide-play'"
      :disabled="!src"
      @click="toggle"
    />
    <span class="text-xs text-muted">{{ label ?? src ?? 'Sin audio' }}</span>
    <audio
      v-if="src"
      ref="audioRef"
      :src="src"
      @play="playing = true"
      @pause="playing = false"
      @ended="playing = false"
    />
  </div>
</template>
