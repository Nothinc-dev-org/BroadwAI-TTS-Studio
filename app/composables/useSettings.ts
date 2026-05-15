import type { ApiKeyStatus, ApiProvider } from '~/types/domain'

export function useSettings() {
  const { invoke } = useTauri()

  return {
    /**
     * Persiste la API key en el keyring del SO. La key se envía una sola vez al
     * backend y no se almacena en el frontend bajo ningún concepto.
     */
    setApiKey: (provider: ApiProvider, key: string) =>
      invoke<ApiKeyStatus>('set_api_key', { provider, key }),
    deleteApiKey: (provider: ApiProvider) =>
      invoke<void>('delete_api_key', { provider }),
    testApiKey: (provider: ApiProvider) =>
      invoke<ApiKeyStatus>('test_api_key', { provider }),
    getApiKeyStatus: (provider: ApiProvider) =>
      invoke<ApiKeyStatus>('get_api_key_status', { provider }),
    getAppSettings: () => invoke<Record<string, string>>('get_app_settings'),
    updateAppSettings: (settings: Record<string, string>) =>
      invoke<void>('update_app_settings', { settings }),
  }
}
