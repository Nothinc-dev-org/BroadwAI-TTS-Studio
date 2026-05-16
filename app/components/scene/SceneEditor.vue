<script setup lang="ts">
import type { Character, DialogueNode, GeneratedAudio, Scene } from '~/types/domain'

defineProps<{
  scene: Scene
  dialogues: DialogueNode[]
  characters: Character[]
  audiosByNode?: Record<string, GeneratedAudio>
}>()

defineEmits<{
  audioGenerated: [audio: GeneratedAudio]
}>()
</script>

<template>
  <section class="space-y-3">
    <header class="flex items-center justify-between">
      <h2 class="text-lg font-semibold">Diálogos</h2>
      <UButton variant="soft" icon="i-lucide-plus" size="sm">
        Nuevo bloque
      </UButton>
    </header>

    <DialogueList
      :dialogues="dialogues"
      :characters="characters"
      :audios-by-node="audiosByNode"
      @audio-generated="(audio: GeneratedAudio) => $emit('audioGenerated', audio)"
    />
  </section>
</template>
