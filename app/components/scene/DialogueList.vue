<script setup lang="ts">
import type { Character, DialogueNode, GeneratedAudio } from '~/types/domain'

const props = defineProps<{
  dialogues: DialogueNode[]
  characters: Character[]
  audiosByNode?: Record<string, GeneratedAudio>
}>()

defineEmits<{
  audioGenerated: [audio: GeneratedAudio]
}>()

function audioFor(nodeId: string): GeneratedAudio | null {
  return props.audiosByNode?.[nodeId] ?? null
}
</script>

<template>
  <div v-if="!dialogues.length" class="rounded-lg border border-dashed border-default p-8 text-center text-sm text-muted">
    Aún no hay bloques. Importa un guion o crea uno manualmente.
  </div>
  <ol v-else class="space-y-2">
    <li v-for="dialogue in dialogues" :key="dialogue.id">
      <DialogueBlock
        :dialogue="dialogue"
        :characters="characters"
        :audio="audioFor(dialogue.id)"
        @generated="(audio: GeneratedAudio) => $emit('audioGenerated', audio)"
      />
    </li>
  </ol>
</template>
