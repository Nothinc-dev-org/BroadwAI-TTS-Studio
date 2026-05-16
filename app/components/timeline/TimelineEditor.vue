<script setup lang="ts">
import type { AudioAsset, TimelineEvent, TimelineTrack } from '~/types/domain'

defineProps<{
  tracks: TimelineTrack[]
  events: TimelineEvent[]
  assets: AudioAsset[]
}>()

defineEmits<{
  removeEvent: [id: string]
}>()
</script>

<template>
  <section class="rounded-lg border border-default p-3">
    <h3 class="mb-2 text-sm font-semibold">Timeline</h3>
    <div v-if="!tracks.length" class="rounded-md border border-dashed border-default p-6 text-center text-xs text-muted">
      Aún no hay pistas. Importa assets y añádelos a la escena para crearlas automáticamente.
    </div>
    <div v-else class="space-y-2">
      <TimelineTrack
        v-for="track in tracks"
        :key="track.id"
        :track="track"
        :events="events"
        :assets="assets"
        @remove-event="(id: string) => $emit('removeEvent', id)"
      />
    </div>
  </section>
</template>
