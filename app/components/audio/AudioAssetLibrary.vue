<script setup lang="ts">
import type { AudioAsset } from '~/types/domain'

defineProps<{ assets: AudioAsset[] }>()
</script>

<template>
  <section>
    <header class="mb-2 flex items-center justify-between">
      <h3 class="text-sm font-semibold">Biblioteca de assets</h3>
      <UButton size="xs" variant="soft" icon="i-lucide-upload">Importar</UButton>
    </header>
    <ul v-if="assets.length" class="space-y-1">
      <li
        v-for="asset in assets"
        :key="asset.id"
        class="flex items-center justify-between rounded-md border border-default px-3 py-2 text-sm"
      >
        <div>
          <div class="font-medium">{{ asset.name }}</div>
          <div class="text-xs text-muted">{{ asset.type }}</div>
        </div>
        <UButton size="xs" variant="ghost" icon="i-lucide-play" />
      </li>
    </ul>
    <div v-else class="rounded-md border border-dashed border-default p-4 text-center text-xs text-muted">
      Sin assets importados.
    </div>
  </section>
</template>
