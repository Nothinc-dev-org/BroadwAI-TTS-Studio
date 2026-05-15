<script setup lang="ts">
import type { TimelineEvent, TimelineTrack } from '~/types/domain'

const props = defineProps<{
  track: TimelineTrack
  events: TimelineEvent[]
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
    <div class="flex flex-wrap gap-1">
      <TimelineEvent v-for="event in trackEvents" :key="event.id" :event="event" />
    </div>
  </div>
</template>
