<script setup lang="ts">
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { listen } from '@tauri-apps/api/event'
import type { DeepSeekResult } from '~/types/domain'

const props = defineProps<{
  open: boolean
  projectId: string
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
  'scene-created': [sceneId: string]
}>()

const open = computed({
  get: () => props.open,
  set: (value: boolean) => emit('update:open', value),
})

type Mode = 'paste' | 'file'
type Stage = 'input' | 'review'

const mode = ref<Mode>('paste')
const stage = ref<Stage>('input')
const text = ref('')
const filePath = ref<string | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const rawImportId = ref<string | null>(null)
const result = ref<DeepSeekResult | null>(null)
const progress = ref<{ completed: number; total: number } | null>(null)

interface DeepSeekImportProgress {
  raw_import_id: string
  completed: number
  total: number
}

const progressPercent = computed(() => {
  if (!progress.value || progress.value.total <= 0) return 0
  return Math.round((progress.value.completed / progress.value.total) * 100)
})

const importer = useImport()

watch(open, value => {
  if (!value) reset()
})

function reset() {
  mode.value = 'paste'
  stage.value = 'input'
  text.value = ''
  filePath.value = null
  loading.value = false
  error.value = null
  rawImportId.value = null
  result.value = null
  progress.value = null
}

async function pickFile() {
  const selected = await openDialog({
    multiple: false,
    filters: [{ name: 'Guion', extensions: ['txt', 'md'] }],
  })
  if (typeof selected === 'string') {
    filePath.value = selected
  }
}

async function process() {
  loading.value = true
  error.value = null
  progress.value = null
  let unlistenProgress: (() => void) | null = null
  try {
    let importId = rawImportId.value
    if (!importId) {
      if (mode.value === 'paste') {
        if (!text.value.trim()) {
          error.value = 'Pega el texto del guion antes de continuar.'
          return
        }
        const raw = await importer.importText(props.projectId, text.value)
        importId = raw.id
      }
      else {
        if (!filePath.value) {
          error.value = 'Selecciona un archivo .txt o .md.'
          return
        }
        const raw = await importer.importFile(props.projectId, filePath.value)
        importId = raw.id
      }
      rawImportId.value = importId
    }
    unlistenProgress = await listen<DeepSeekImportProgress>('import://deepseek-progress', event => {
      if (event.payload.raw_import_id !== importId) return
      progress.value = {
        completed: event.payload.completed,
        total: event.payload.total,
      }
    })
    result.value = await importer.processWithDeepSeek(importId)
    stage.value = 'review'
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    unlistenProgress?.()
    loading.value = false
  }
}

async function createScene() {
  if (!rawImportId.value) return
  loading.value = true
  error.value = null
  try {
    const scene = await importer.createScene(rawImportId.value)
    emit('scene-created', scene.id)
    open.value = false
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    loading.value = false
  }
}

function backToInput() {
  stage.value = 'input'
  result.value = null
}
</script>

<template>
  <UModal v-model:open="open" title="Importar guion" :ui="{ content: 'max-w-3xl' }">
    <template #body>
      <div class="space-y-4">
        <template v-if="stage === 'input'">
          <UTabs
            v-model="mode"
            :items="[
              { value: 'paste', label: 'Copy-paste', icon: 'i-lucide-clipboard' },
              { value: 'file', label: 'Archivo .txt/.md', icon: 'i-lucide-file-text' },
            ]"
          />

          <template v-if="mode === 'paste'">
            <UTextarea
              v-model="text"
              class="w-full"
              :rows="10"
              placeholder="Pega aquí el guion en texto plano o markdown…"
            />
          </template>
          <template v-else>
            <div class="flex items-center gap-2">
              <UButton variant="soft" icon="i-lucide-folder" @click="pickFile">Elegir archivo</UButton>
              <span class="truncate text-sm text-muted">{{ filePath ?? 'Ningún archivo seleccionado' }}</span>
            </div>
          </template>

          <p class="text-xs text-muted">
            El texto se guarda localmente y se envía a DeepSeek V4 Flash para estructurarlo.
            Requiere API key configurada en Ajustes.
          </p>

          <div v-if="loading && progress" class="space-y-1 rounded-lg border border-default p-3">
            <div class="flex items-center justify-between gap-3 text-xs text-muted">
              <span>Procesando bloques enviados</span>
              <span>{{ progress.completed }} / {{ progress.total }} · {{ progressPercent }}%</span>
            </div>
            <UProgress :model-value="progressPercent" />
          </div>
        </template>

        <template v-else-if="stage === 'review' && result">
          <ImportReview :result="result" />
        </template>

        <UAlert
          v-if="error"
          icon="i-lucide-circle-alert"
          color="error"
          variant="soft"
          :description="error"
        />
      </div>
    </template>

    <template #footer>
      <div class="flex w-full items-center justify-between gap-2">
        <UButton
          v-if="stage === 'review'"
          variant="ghost"
          icon="i-lucide-arrow-left"
          :disabled="loading"
          @click="backToInput"
        >
          Volver
        </UButton>
        <span v-else />

        <div class="flex gap-2">
          <UButton variant="ghost" :disabled="loading" @click="open = false">Cerrar</UButton>
          <UButton
            v-if="stage === 'input'"
            icon="i-lucide-sparkles"
            :loading="loading"
            @click="process"
          >
            Procesar con DeepSeek
          </UButton>
          <UButton
            v-else
            icon="i-lucide-check"
            :loading="loading"
            @click="createScene"
          >
            Crear escena
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>
