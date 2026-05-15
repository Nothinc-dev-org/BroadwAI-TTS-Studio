<script setup lang="ts">
import type { Character, DialogueNode, Scene } from '~/types/domain'

defineProps<{
  scene: Scene
  dialogues: DialogueNode[]
  characters: Character[]
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

    <DialogueList :dialogues="dialogues" :characters="characters" />
  </section>
</template>
