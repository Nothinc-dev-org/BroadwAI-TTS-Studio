<script setup lang="ts">
import type { ApiKeyStatus, ApiProvider } from '~/types/domain'

const settings = useSettings()

interface ProviderState {
  status: ApiKeyStatus
  inputKey: string
  loading: boolean
  message: string | null
  error: string | null
}

const providers: { id: ApiProvider; label: string; description: string }[] = [
  {
    id: 'deepseek',
    label: 'DeepSeek V4 Flash',
    description: 'Se usa para estructurar guiones importados.',
  },
  {
    id: 'gemini',
    label: 'Gemini 3.1 Flash TTS Preview',
    description: 'Se usa para generar audio por diálogo y escena.',
  },
]

const state = reactive<Record<ApiProvider, ProviderState>>({
  deepseek: { status: 'not_configured', inputKey: '', loading: false, message: null, error: null },
  gemini: { status: 'not_configured', inputKey: '', loading: false, message: null, error: null },
})

const statusLabel: Record<ApiKeyStatus, string> = {
  not_configured: 'No configurada',
  configured: 'Configurada',
  valid: 'Válida',
  invalid: 'Inválida',
  connection_error: 'Error de conexión',
}

const statusColor: Record<ApiKeyStatus, 'neutral' | 'primary' | 'success' | 'error' | 'warning'> = {
  not_configured: 'neutral',
  configured: 'primary',
  valid: 'success',
  invalid: 'error',
  connection_error: 'warning',
}

async function refreshStatus(provider: ApiProvider) {
  try {
    state[provider].status = await settings.getApiKeyStatus(provider)
  }
  catch (err) {
    state[provider].error = String(err)
  }
}

async function save(provider: ApiProvider) {
  if (!state[provider].inputKey) return
  state[provider].loading = true
  state[provider].error = null
  state[provider].message = null
  try {
    state[provider].status = await settings.setApiKey(provider, state[provider].inputKey)
    state[provider].inputKey = ''
    state[provider].message = 'API key guardada en el keyring del sistema.'
  }
  catch (err) {
    state[provider].error = String(err)
  }
  finally {
    state[provider].loading = false
  }
}

async function remove(provider: ApiProvider) {
  state[provider].loading = true
  state[provider].error = null
  state[provider].message = null
  try {
    await settings.deleteApiKey(provider)
    await refreshStatus(provider)
    state[provider].message = 'API key eliminada del keyring.'
  }
  catch (err) {
    state[provider].error = String(err)
  }
  finally {
    state[provider].loading = false
  }
}

async function test(provider: ApiProvider) {
  state[provider].loading = true
  state[provider].error = null
  state[provider].message = null
  try {
    state[provider].status = await settings.testApiKey(provider)
    state[provider].message = 'Conexión verificada.'
  }
  catch (err) {
    state[provider].error = String(err)
  }
  finally {
    state[provider].loading = false
  }
}

onMounted(async () => {
  await Promise.all(providers.map(p => refreshStatus(p.id)))
})
</script>

<template>
  <div class="mx-auto max-w-3xl px-6 py-10">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">Configuración</h1>
      <p class="text-sm text-muted">
        Las API keys se almacenan en el keyring del sistema operativo. Nunca se persisten en SQLite ni en localStorage.
      </p>
    </div>

    <div class="space-y-6">
      <UCard v-for="provider in providers" :key="provider.id">
        <template #header>
          <div class="flex items-center justify-between">
            <div>
              <h2 class="font-semibold">{{ provider.label }}</h2>
              <p class="text-sm text-muted">{{ provider.description }}</p>
            </div>
            <UBadge :color="statusColor[state[provider.id].status]" variant="subtle">
              {{ statusLabel[state[provider.id].status] }}
            </UBadge>
          </div>
        </template>

        <div class="space-y-3">
          <UFormField label="Nueva API key">
            <UInput
              v-model="state[provider.id].inputKey"
              class="w-full"
              type="password"
              placeholder="••••••••••••"
              autocomplete="off"
              :disabled="state[provider.id].loading"
            />
          </UFormField>

          <p v-if="state[provider.id].message" class="text-sm text-success">
            {{ state[provider.id].message }}
          </p>
          <p v-if="state[provider.id].error" class="text-sm text-error">
            {{ state[provider.id].error }}
          </p>
        </div>

        <template #footer>
          <div class="flex justify-end gap-2">
            <UButton
              variant="ghost"
              color="error"
              :disabled="state[provider.id].status === 'not_configured' || state[provider.id].loading"
              @click="remove(provider.id)"
            >
              Borrar
            </UButton>
            <UButton
              variant="soft"
              :disabled="state[provider.id].status === 'not_configured' || state[provider.id].loading"
              @click="test(provider.id)"
            >
              Probar conexión
            </UButton>
            <UButton
              :disabled="!state[provider.id].inputKey || state[provider.id].loading"
              @click="save(provider.id)"
            >
              Guardar
            </UButton>
          </div>
        </template>
      </UCard>
    </div>
  </div>
</template>
