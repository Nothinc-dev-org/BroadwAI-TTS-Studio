<script setup lang="ts">
import type { AudioAsset, TimelineEvent, TimelineTrack } from '~/types/domain'

const props = defineProps<{
  track: TimelineTrack
  events: TimelineEvent[]
  assets: AudioAsset[]
}>()

defineEmits<{
  removeEvent: [id: string]
}>()

const trackEvents = computed(() =>
  props.events.filter(e => e.track_id === props.track.id),
)
</script>

<template>
  <div class="rounded-md border border-default px-3 py-2">
    <header class="mb-1 flex items-center justify-between text-sm">
      <span class="font-medium">{{ track.name }}</span>
      <UBadge color="neutral" variant="subtle" size="sm">{{ track.type }}</UBadge>
    </header>
    <div v-if="trackEvents.length" class="flex flex-wrap gap-1">
      <TimelineEvent
        v-for="event in trackEvents"
        :key="event.id"
        :event="event"
        :assets="assets"
        @remove="(id: string) => $emit('removeEvent', id)"
      />
    </div>
    <div v-else class="text-xs text-muted">Sin eventos.</div>
  </div>
</template>
