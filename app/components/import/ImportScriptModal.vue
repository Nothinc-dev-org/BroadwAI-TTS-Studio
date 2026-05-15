<script setup lang="ts">
import { open as openDialog } from '@tauri-apps/plugin-dialog'

const props = defineProps<{
  open: boolean
  projectId: string
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
  imported: []
}>()

const open = computed({
  get: () => props.open,
  set: (value: boolean) => emit('update:open', value),
})

type Mode = 'paste' | 'file'

const mode = ref<Mode>('paste')
const text = ref('')
const filePath = ref<string | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const status = ref<string | null>(null)

const importer = useImport()

async function pickFile() {
  const selected = await openDialog({
    multiple: false,
    filters: [{ name: 'Guion', extensions: ['txt', 'md'] }],
  })
  if (typeof selected === 'string') {
    filePath.value = selected
  }
}

async function submit() {
  loading.value = true
  error.value = null
  status.value = null
  try {
    if (mode.value === 'paste') {
      if (!text.value.trim()) {
        error.value = 'Pega el texto del guion antes de continuar.'
        return
      }
      const raw = await importer.importText(props.projectId, text.value)
      status.value = `Texto guardado (${raw.id}). El procesamiento con DeepSeek se habilita al configurar la API key.`
    }
    else {
      if (!filePath.value) {
        error.value = 'Selecciona un archivo .txt o .md.'
        return
      }
      const raw = await importer.importFile(props.projectId, filePath.value)
      status.value = `Archivo importado (${raw.id}).`
    }
    emit('imported')
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    loading.value = false
  }
}
</script>

<template>
  <UModal v-model:open="open" title="Importar guion" :ui="{ content: 'max-w-2xl' }">
    <template #body>
      <div class="space-y-4">
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

        <p v-if="status" class="text-sm text-success">{{ status }}</p>
        <p v-if="error" class="text-sm text-error">{{ error }}</p>
      </div>
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton variant="ghost" :disabled="loading" @click="open = false">Cerrar</UButton>
        <UButton :loading="loading" @click="submit">Importar</UButton>
      </div>
    </template>
  </UModal>
</template>
