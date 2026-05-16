<script setup lang="ts">
import type { DeepSeekResult } from '~/types/domain'

const props = defineProps<{ result: DeepSeekResult }>()

const warningDialogOpen = ref(false)
const selectedWarning = ref<string | null>(null)

const selectedWarningDialogue = computed(() => {
  if (!selectedWarning.value) return null
  const match = selectedWarning.value.match(/diálogo (\d+)/)
  if (!match) return null
  const index = Number(match[1]) - 1
  return props.result.scene.dialogues[index] ?? null
})

const kindLabel: Record<string, string> = {
  narration: 'Narración',
  dialogue: 'Diálogo',
  thought: 'Pensamiento',
  system: 'Sistema',
  direction: 'Acotación',
}

const roleLabel: Record<string, string> = {
  narrator: 'Narrador',
  character: 'Personaje',
  system: 'Sistema',
}

function showWarningDetails(warning: string) {
  selectedWarning.value = warning
  warningDialogOpen.value = true
}
</script>

<template>
  <div class="space-y-4">
    <header>
      <div class="flex items-baseline gap-2">
        <h3 class="text-base font-semibold">{{ result.scene.title }}</h3>
        <UBadge color="neutral" variant="subtle" size="sm">{{ result.scene.language }}</UBadge>
      </div>
      <p v-if="result.scene.description" class="mt-1 text-sm text-muted">{{ result.scene.description }}</p>
    </header>

    <UAlert
      v-if="result.warnings.length"
      icon="i-lucide-triangle-alert"
      color="warning"
      variant="soft"
      title="Avisos del análisis"
    >
      <template #description>
        <ul class="space-y-1">
          <li
            v-for="warning in result.warnings"
            :key="warning"
            class="flex items-start gap-2"
          >
            <span class="mt-2 size-1.5 shrink-0 rounded-full bg-warning" />
            <span class="min-w-0 flex-1">{{ warning }}</span>
            <UButton
              icon="i-lucide-info"
              variant="ghost"
              color="warning"
              size="xs"
              square
              aria-label="Ver detalle del aviso"
              @click="showWarningDetails(warning)"
            />
          </li>
        </ul>
      </template>
    </UAlert>

    <UModal v-model:open="warningDialogOpen" title="Detalle del aviso" :ui="{ content: 'max-w-xl' }">
      <template #body>
        <div class="space-y-4">
          <UAlert
            v-if="selectedWarning"
            icon="i-lucide-triangle-alert"
            color="warning"
            variant="soft"
            :description="selectedWarning"
          />

          <template v-if="selectedWarningDialogue">
            <div class="space-y-2 rounded-lg border border-default p-3">
              <div class="flex flex-wrap items-center gap-2 text-xs">
                <UBadge color="neutral" variant="subtle" size="xs">
                  {{ kindLabel[selectedWarningDialogue.type] ?? selectedWarningDialogue.type }}
                </UBadge>
                <span class="font-medium">{{ selectedWarningDialogue.speaker }}</span>
                <UBadge
                  v-for="tag in selectedWarningDialogue.tts_tags"
                  :key="tag"
                  color="info"
                  variant="subtle"
                  size="xs"
                >
                  {{ tag }}
                </UBadge>
              </div>

              <div>
                <h5 class="mb-1 text-xs font-semibold text-muted">Texto estructurado</h5>
                <p class="whitespace-pre-wrap text-sm leading-relaxed">
                  {{ selectedWarningDialogue.text }}
                </p>
              </div>

              <div v-if="selectedWarningDialogue.original_excerpt">
                <h5 class="mb-1 text-xs font-semibold text-muted">original_excerpt recibido</h5>
                <p class="whitespace-pre-wrap rounded-md bg-muted px-2 py-1.5 text-sm leading-relaxed">
                  {{ selectedWarningDialogue.original_excerpt }}
                </p>
              </div>
            </div>

            <p class="text-xs text-muted">
              Este aviso indica que la cita de origen no coincide con el texto original tras normalizar markdown básico. Revisa si el bloque conserva el contenido correcto antes de crear la escena.
            </p>
          </template>

          <p v-else class="text-sm text-muted">
            Este aviso no está asociado a un diálogo concreto.
          </p>
        </div>
      </template>
    </UModal>

    <section>
      <h4 class="mb-2 text-sm font-semibold">Personajes ({{ result.scene.characters.length }})</h4>
      <ul class="space-y-1">
        <li
          v-for="character in result.scene.characters"
          :key="character.name"
          class="flex items-start justify-between gap-3 rounded-md border border-default px-3 py-2"
        >
          <div class="min-w-0">
            <div class="flex items-center gap-2 text-sm font-medium">
              <span class="truncate">{{ character.name }}</span>
              <UBadge color="neutral" variant="subtle" size="xs">
                {{ roleLabel[character.role] ?? character.role }}
              </UBadge>
            </div>
            <p v-if="character.description" class="mt-0.5 text-xs text-muted">{{ character.description }}</p>
          </div>
          <div v-if="character.aliases.length" class="flex flex-wrap justify-end gap-1">
            <UBadge
              v-for="alias in character.aliases"
              :key="alias"
              color="primary"
              variant="subtle"
              size="xs"
            >
              {{ alias }}
            </UBadge>
          </div>
        </li>
      </ul>
    </section>

    <section>
      <h4 class="mb-2 text-sm font-semibold">Diálogos ({{ result.scene.dialogues.length }})</h4>
      <ol class="max-h-96 space-y-2 overflow-auto pr-1">
        <li
          v-for="(dialogue, index) in result.scene.dialogues"
          :key="index"
          class="rounded-lg border border-default bg-elevated p-3"
        >
          <header class="mb-1.5 flex flex-wrap items-center gap-2 text-xs">
            <span class="text-muted">#{{ index + 1 }}</span>
            <UBadge color="neutral" variant="subtle" size="xs">
              {{ kindLabel[dialogue.type] ?? dialogue.type }}
            </UBadge>
            <span class="font-medium">{{ dialogue.speaker }}</span>
            <UBadge
              v-for="tag in dialogue.tts_tags"
              :key="tag"
              color="info"
              variant="subtle"
              size="xs"
            >
              {{ tag }}
            </UBadge>
          </header>
          <p class="whitespace-pre-wrap text-sm leading-relaxed">{{ dialogue.text }}</p>
        </li>
      </ol>
    </section>

    <section v-if="result.scene.unassigned_fragments.length">
      <h4 class="mb-2 text-sm font-semibold">Fragmentos sin asignar</h4>
      <ul class="space-y-1">
        <li
          v-for="(fragment, index) in result.scene.unassigned_fragments"
          :key="index"
          class="rounded-md border border-dashed border-default px-3 py-2 text-sm text-muted"
        >
          {{ fragment }}
        </li>
      </ul>
    </section>
  </div>
</template>
