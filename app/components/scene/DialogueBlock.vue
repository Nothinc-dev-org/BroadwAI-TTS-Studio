<script setup lang="ts">
import type { Character, DialogueNode } from '~/types/domain'

const props = defineProps<{
  dialogue: DialogueNode
  characters: Character[]
}>()

const speaker = computed(() =>
  props.characters.find(c => c.id === props.dialogue.character_id)?.name ?? 'Sin asignar',
)

const kindLabel: Record<string, string> = {
  narration: 'Narración',
  dialogue: 'Diálogo',
  thought: 'Pensamiento',
  system: 'Sistema',
  direction: 'Acotación',
}
</script>

<template>
  <article class="rounded-lg border border-default bg-elevated p-3">
    <header class="mb-2 flex items-center justify-between">
      <div class="flex items-center gap-2 text-sm">
        <UBadge color="neutral" variant="subtle">{{ kindLabel[dialogue.type] ?? dialogue.type }}</UBadge>
        <span class="font-medium">{{ speaker }}</span>
      </div>
      <div class="flex items-center gap-1">
        <UButton size="xs" variant="ghost" icon="i-lucide-play" />
        <UButton size="xs" variant="ghost" icon="i-lucide-refresh-cw" />
        <UButton size="xs" variant="ghost" icon="i-lucide-scissors" />
        <UButton size="xs" variant="ghost" icon="i-lucide-trash-2" />
      </div>
    </header>
    <p class="whitespace-pre-wrap text-sm leading-relaxed">{{ dialogue.text }}</p>
  </article>
</template>
