<script setup lang="ts">
defineProps<{ sceneId: string }>()
</script>

<template>
  <section class="rounded-lg border border-default p-3">
    <header class="mb-2 flex items-center justify-between">
      <h3 class="text-sm font-semibold">Cola de render</h3>
      <UBadge color="neutral" variant="subtle" size="sm">vacía</UBadge>
    </header>
    <p class="text-xs text-muted">
      Los jobs de generación TTS y mezcla se mostrarán aquí cuando estén implementados (RF-29).
    </p>
  </section>
</template>
