// https://nuxt.com/docs/api/configuration/nuxt-config
const host = process.env.TAURI_DEV_HOST

export default defineNuxtConfig({
  compatibilityDate: '2026-05-15',
  devtools: { enabled: true },

  modules: ['@nuxt/ui'],

  css: ['~/assets/css/main.css'],

  // SPA en producción (Tauri carga estáticos) vía `bun run generate`.
  // En dev mantenemos SSR activo: hay un bug en @nuxt/vite-builder 4.4.5 que
  // rompe `nuxt dev` con `ssr: false` (resolveServerEntry no encuentra entry).
  ssr: true,

  nitro: {
    preset: 'static',
  },

  components: [
    { path: '~/components', pathPrefix: false },
  ],

  app: {
    head: {
      title: 'BroadwAI TTS Studio',
      meta: [{ charset: 'utf-8' }, { name: 'viewport', content: 'width=device-width, initial-scale=1' }],
    },
  },

  devServer: {
    host: host || 'localhost',
    port: 1420,
  },

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_ENV_*'],
    server: {
      strictPort: true,
      host: host || false,
      hmr: host
        ? { protocol: 'ws', host, port: 1421 }
        : undefined,
      watch: {
        ignored: ['**/src-tauri/**'],
      },
    },
  },

  typescript: {
    strict: true,
  },
})
