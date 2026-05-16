<script setup lang="ts">
import type { AudioAsset, TimelineEvent } from '~/types/domain'

const props = defineProps<{
  event: TimelineEvent
  assets: AudioAsset[]
}>()

defineEmits<{
  remove: [id: string]
}>()

const assetName = computed(() => {
  if (!props.event.audio_asset_id) return null
  return props.assets.find(a => a.id === props.event.audio_asset_id)?.name ?? null
})

function formatStart(ms: number): string {
  if (ms <= 0) return '0:00'
  const total = Math.round(ms / 1000)
  const m = Math.floor(total / 60)
  const s = total % 60
  return `${m}:${String(s).padStart(2, '0')}`
}
</script>

<template>
  <div class="flex items-center gap-1 rounded-sm border border-default bg-primary/10 px-2 py-1 text-xs">
    <span class="font-medium">{{ formatStart(event.start_ms) }}</span>
    <span v-if="assetName" class="text-muted">· {{ assetName }}</span>
    <span v-else-if="event.duration_ms" class="text-muted">· {{ event.duration_ms }}ms</span>
    <UBadge v-if="event.loop" color="info" variant="subtle" size="xs">loop</UBadge>
    <UButton
      size="xs"
      variant="ghost"
      icon="i-lucide-x"
      class="ml-1"
      @click="$emit('remove', event.id)"
    />
  </div>
</template>
