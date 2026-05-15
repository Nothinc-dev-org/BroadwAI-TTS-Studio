<script setup lang="ts">
import type { Project } from '~/types/domain'

defineProps<{ project: Project }>()
</script>

<template>
  <NuxtLink
    :to="`/projects/${project.id}`"
    class="block rounded-lg border border-default p-4 transition hover:border-primary hover:bg-primary/5"
  >
    <div class="mb-1 font-semibold">{{ project.title }}</div>
    <p v-if="project.description" class="line-clamp-2 text-sm text-muted">
      {{ project.description }}
    </p>
    <p class="mt-2 truncate text-xs text-muted">{{ project.root_path }}</p>
  </NuxtLink>
</template>
