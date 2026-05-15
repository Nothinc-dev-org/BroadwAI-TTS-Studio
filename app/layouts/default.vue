<script setup lang="ts">
const route = useRoute()

const navLinks = computed(() => [
  { label: 'Inicio', to: '/', icon: 'i-lucide-home' },
  { label: 'Configuración', to: '/settings', icon: 'i-lucide-settings' },
])
</script>

<template>
  <div class="flex h-screen flex-col bg-default text-default">
    <header class="flex items-center justify-between border-b border-default px-4 py-2">
      <div class="flex items-center gap-3">
        <div class="text-lg font-semibold tracking-tight">BroadwAI TTS Studio</div>
        <UBadge color="neutral" variant="subtle" size="sm">v0.1 · MVP 1</UBadge>
      </div>
      <nav class="flex items-center gap-1">
        <UButton
          v-for="link in navLinks"
          :key="link.to"
          :to="link.to"
          :icon="link.icon"
          :variant="route.path === link.to ? 'soft' : 'ghost'"
          size="sm"
        >
          {{ link.label }}
        </UButton>
      </nav>
    </header>
    <main class="flex-1 overflow-y-auto">
      <slot />
    </main>
  </div>
</template>
