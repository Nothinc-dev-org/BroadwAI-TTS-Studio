<script setup lang="ts">
import type { Character } from '~/types/domain'

withDefaults(
  defineProps<{
    characters: Character[]
    compact?: boolean
  }>(),
  { compact: false },
)
</script>

<template>
  <section>
    <header class="mb-2 flex items-center justify-between" :class="{ 'mb-3': !compact }">
      <h3 :class="compact ? 'text-sm font-semibold' : 'text-base font-semibold'">Personajes</h3>
      <UButton size="xs" variant="soft" icon="i-lucide-plus">Nuevo</UButton>
    </header>

    <ul v-if="characters.length" class="space-y-1">
      <li
        v-for="character in characters"
        :key="character.id"
        class="flex items-center justify-between rounded-md border border-default px-3 py-2"
      >
        <div class="flex items-center gap-2">
          <span
            class="inline-block size-2 rounded-full"
            :style="{ background: character.color ?? '#94a3b8' }"
          />
          <div>
            <div class="text-sm font-medium">{{ character.name }}</div>
            <div class="text-xs text-muted">{{ character.role }}</div>
          </div>
        </div>
        <UBadge v-if="character.voice_id" color="primary" variant="subtle" size="sm">
          {{ character.voice_id }}
        </UBadge>
      </li>
    </ul>
    <div v-else class="rounded-md border border-dashed border-default p-4 text-center text-xs text-muted">
      Aún no hay personajes.
    </div>
  </section>
</template>
