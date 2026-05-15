<script setup lang="ts">
defineProps<{ text: string }>()
</script>

<template>
  <pre class="max-h-96 overflow-auto whitespace-pre-wrap rounded-md border border-default bg-muted/30 p-3 text-sm leading-relaxed">{{ text }}</pre>
</template>
