import { invoke as tauriInvoke } from '@tauri-apps/api/core'

/**
 * Wrapper sobre `@tauri-apps/api`.
 *
 * Centraliza la invocación de comandos Rust y normaliza errores. Durante el
 * desarrollo (cuando se ejecuta en navegador puro sin Tauri), `invoke` devuelve
 * un error explícito para que las pantallas sepan que el backend no está
 * disponible.
 */
export function useTauri() {
  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

  async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    if (!isTauri) {
      throw new Error(`Backend Tauri no disponible: ${cmd}`)
    }
    return tauriInvoke<T>(cmd, args)
  }

  return { invoke, isTauri }
}
