<script setup lang="ts">
import type { TimelineEvent, TimelineTrack } from '~/types/domain'

defineProps<{
  tracks: TimelineTrack[]
  events: TimelineEvent[]
}>()
</script>

<template>
  <section class="rounded-lg border border-default p-3">
    <h3 class="mb-2 text-sm font-semibold">Timeline</h3>
    <div v-if="!tracks.length" class="rounded-md border border-dashed border-default p-6 text-center text-xs text-muted">
      Crea pistas (voces, SFX, música, ambiente) para construir la mezcla (RF-36).
    </div>
    <div v-else class="space-y-2">
      <TimelineTrack v-for="track in tracks" :key="track.id" :track="track" :events="events" />
    </div>
  </section>
</template>
