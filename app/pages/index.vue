<script setup lang="ts">
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import type { Project } from '~/types/domain'

const projects = ref<Project[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const showNewProject = ref(false)

const projectsApi = useProjects()
const { isTauri } = useTauri()

async function loadProjects() {
  if (!isTauri) return
  loading.value = true
  error.value = null
  try {
    projects.value = await projectsApi.listRecent()
  }
  catch (err) {
    // Hasta que el usuario abra un proyecto no hay BD; ignoramos.
    projects.value = []
  }
  finally {
    loading.value = false
  }
}

async function pickFolderAndCreate() {
  const selected = await openDialog({ directory: true, multiple: false })
  if (typeof selected === 'string') {
    newProjectForm.value.rootPath = selected
    showNewProject.value = true
  }
}

async function openExisting() {
  const selected = await openDialog({ directory: true, multiple: false })
  if (typeof selected === 'string') {
    try {
      await projectsApi.open(selected)
      await loadProjects()
    }
    catch (err) {
      error.value = String(err)
    }
  }
}

const importing = ref(false)

async function importProject() {
  error.value = null
  const sourcePath = await openDialog({
    multiple: false,
    filters: [{ name: 'JSON', extensions: ['json'] }],
  })
  if (typeof sourcePath !== 'string') return
  const targetRoot = await openDialog({ directory: true, multiple: false })
  if (typeof targetRoot !== 'string') return
  importing.value = true
  try {
    const project = await projectsApi.import(sourcePath, targetRoot)
    await loadProjects()
    await navigateTo(`/projects/${project.id}`)
  }
  catch (err) {
    error.value = String(err)
  }
  finally {
    importing.value = false
  }
}

const newProjectForm = ref({
  title: '',
  description: '',
  language: 'es-MX',
  rootPath: '',
})

async function createProject() {
  if (!newProjectForm.value.title || !newProjectForm.value.rootPath) return
  try {
    await projectsApi.create({
      title: newProjectForm.value.title,
      description: newProjectForm.value.description || undefined,
      language: newProjectForm.value.language,
      rootPath: newProjectForm.value.rootPath,
    })
    showNewProject.value = false
    newProjectForm.value = { title: '', description: '', language: 'es-MX', rootPath: '' }
    await loadProjects()
  }
  catch (err) {
    error.value = String(err)
  }
}

onMounted(loadProjects)
</script>

<template>
  <div class="mx-auto max-w-5xl px-6 py-10">
    <div class="mb-8 flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-semibold">Proyectos</h1>
        <p class="text-sm text-muted">Crea o abre un proyecto local para empezar.</p>
      </div>
      <div class="flex gap-2">
        <UButton
          icon="i-lucide-package-open"
          variant="ghost"
          :loading="importing"
          @click="importProject"
        >
          Importar JSON
        </UButton>
        <UButton icon="i-lucide-folder-open" variant="soft" @click="openExisting">
          Abrir proyecto
        </UButton>
        <UButton icon="i-lucide-plus" @click="pickFolderAndCreate">
          Nuevo proyecto
        </UButton>
      </div>
    </div>

    <UAlert
      v-if="!isTauri"
      icon="i-lucide-info"
      color="warning"
      variant="soft"
      title="Modo navegador"
      description="Estás viendo BroadwAI en el navegador. Para acceder al backend local ejecuta `bun run tauri:dev`."
      class="mb-6"
    />

    <UAlert
      v-if="error"
      icon="i-lucide-circle-alert"
      color="error"
      variant="soft"
      :description="error"
      class="mb-6"
    />

    <div v-if="loading" class="text-sm text-muted">Cargando proyectos…</div>

    <div v-else-if="!projects.length" class="rounded-lg border border-dashed border-default p-12 text-center">
      <UIcon name="i-lucide-folder-open" class="mx-auto mb-3 size-10 text-muted" />
      <p class="font-medium">Aún no hay proyectos abiertos</p>
      <p class="text-sm text-muted">
        Crea uno nuevo o abre la carpeta de uno existente.
      </p>
    </div>

    <ul v-else class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
      <li v-for="project in projects" :key="project.id">
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
      </li>
    </ul>

    <UModal v-model:open="showNewProject" title="Nuevo proyecto">
      <template #body>
        <UForm :state="newProjectForm" class="space-y-4" @submit.prevent="createProject">
          <UFormField label="Título" required>
            <UInput v-model="newProjectForm.title" class="w-full" placeholder="Capítulo 1" />
          </UFormField>
          <UFormField label="Descripción">
            <UTextarea v-model="newProjectForm.description" class="w-full" :rows="2" />
          </UFormField>
          <UFormField label="Idioma">
            <UInput v-model="newProjectForm.language" class="w-full" placeholder="es-MX" />
          </UFormField>
          <UFormField label="Carpeta">
            <UInput v-model="newProjectForm.rootPath" class="w-full" readonly />
          </UFormField>
          <div class="flex justify-end gap-2 pt-2">
            <UButton variant="ghost" @click="showNewProject = false">Cancelar</UButton>
            <UButton type="submit" :disabled="!newProjectForm.title">Crear</UButton>
          </div>
        </UForm>
      </template>
    </UModal>
  </div>
</template>
