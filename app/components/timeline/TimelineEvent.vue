<script setup lang="ts">
import type { TimelineEvent } from '~/types/domain'

defineProps<{ event: TimelineEvent }>()
</script>

<template>
  <div class="rounded-sm border border-default bg-primary/10 px-2 py-1 text-xs">
    <span class="font-medium">{{ event.start_ms }}ms</span>
    <span v-if="event.duration_ms" class="text-muted"> · {{ event.duration_ms }}ms</span>
  </div>
</template>
