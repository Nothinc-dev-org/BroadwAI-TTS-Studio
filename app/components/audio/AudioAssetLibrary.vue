<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import type { AudioAsset, AudioAssetKind } from '~/types/domain'

const props = defineProps<{
  assets: AudioAsset[]
  projectId: string
  sceneId?: string
}>()

const emit = defineEmits<{
  changed: []
  inserted: []
}>()

const assetsApi = useAssets()
const timelineApi = useTimeline()

const importing = ref(false)
const error = ref<string | null>(null)
const importKind = ref<AudioAssetKind>('sound_effect')
const previewing = ref<string | null>(null)
const previewSrc = ref<string | null>(null)

const kindOptions: { value: AudioAssetKind, label: string }[] = [
  { value: 'sound_effect', label: 'SFX' },
  { value: 'music', label: 'Música' },
  { value: 'ambience', label: 'Ambiente' },
  { value: 'voice', label: 'Voz' },
]

const kindLabel: Record<string, string> = {
  sound_effect: 'SFX',
  music: 'Música',
  ambience: 'Ambiente',
  voice: 'Voz',
  generated: 'Generado',
}

function formatDuration(ms: number | null): string {
  if (!ms || ms <= 0) return '—'
  const total = Math.round(ms / 1000)
  const m = Math.floor(total / 60)
  const s = total % 60
  return `${m}:${String(s).padStart(2, '0')}`
}

async function importAsset() {
  error.value = null
  const selected = await openDialog({
    multiple: false,
    filters: [{ name: 'Audio', extensions: ['wav', 'mp3', 'ogg', 'flac'] }],
  })
  if (typeof selected !== 'string') return
  importing.value = true
  try {
    await assetsApi.import({
      projectId: props.projectId,
      filePath: selected,
      kind: importKind.value,
    })
    emit('changed')
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    importing.value = false
  }
}

async function removeAsset(asset: AudioAsset) {
  error.value = null
  try {
    await assetsApi.remove(asset.id)
    if (previewing.value === asset.id) {
      previewing.value = null
      previewSrc.value = null
    }
    emit('changed')
  }
  catch (err) {
    error.value = String(err)
  }
}

async function addToScene(asset: AudioAsset) {
  if (!props.sceneId) return
  error.value = null
  try {
    await timelineApi.createEvent(props.sceneId, asset.id, 0)
    emit('inserted')
  }
  catch (err) {
    error.value = String(err)
  }
}

function togglePreview(asset: AudioAsset) {
  if (previewing.value === asset.id) {
    previewing.value = null
    previewSrc.value = null
    return
  }
  previewing.value = asset.id
  previewSrc.value = convertFileSrc(asset.file_path)
}
</script>

<template>
  <section>
    <header class="mb-2 flex items-center justify-between gap-2">
      <h3 class="text-sm font-semibold">Biblioteca de assets</h3>
      <div class="flex items-center gap-1">
        <USelect v-model="importKind" :items="kindOptions" size="xs" class="w-32" />
        <UButton
          size="xs"
          variant="soft"
          icon="i-lucide-upload"
          :loading="importing"
          @click="importAsset"
        >
          Importar
        </UButton>
      </div>
    </header>

    <UAlert
      v-if="error"
      icon="i-lucide-circle-alert"
      color="error"
      variant="soft"
      :description="error"
      class="mb-2"
    />

    <ul v-if="assets.length" class="space-y-1">
      <li
        v-for="asset in assets"
        :key="asset.id"
        class="rounded-md border border-default px-3 py-2 text-sm"
      >
        <div class="flex items-center justify-between gap-2">
          <div class="min-w-0">
            <div class="truncate font-medium">{{ asset.name }}</div>
            <div class="flex items-center gap-2 text-xs text-muted">
              <UBadge color="neutral" variant="subtle" size="xs">
                {{ kindLabel[asset.type] ?? asset.type }}
              </UBadge>
              <span>{{ formatDuration(asset.duration_ms) }}</span>
            </div>
          </div>
          <div class="flex items-center gap-1">
            <UButton
              size="xs"
              variant="ghost"
              :icon="previewing === asset.id ? 'i-lucide-x' : 'i-lucide-play'"
              @click="togglePreview(asset)"
            />
            <UButton
              v-if="sceneId"
              size="xs"
              variant="ghost"
              icon="i-lucide-plus"
              title="Añadir a la escena en 0 ms"
              @click="addToScene(asset)"
            />
            <UButton
              size="xs"
              variant="ghost"
              icon="i-lucide-trash-2"
              @click="removeAsset(asset)"
            />
          </div>
        </div>
        <audio
          v-if="previewing === asset.id && previewSrc"
          :src="previewSrc"
          controls
          class="mt-2 h-8 w-full"
        />
      </li>
    </ul>
    <div v-else class="rounded-md border border-dashed border-default p-4 text-center text-xs text-muted">
      Sin assets importados.
    </div>
  </section>
</template>
