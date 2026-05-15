# AGENTS.md — `app/pages/`

## Propósito

Rutas de la aplicación (file-based routing de Nuxt). Una página = una
pantalla del Requerimiento §11.

## Mapa pantalla → archivo

| Pantalla del RF                          | Archivo                                       |
| ---------------------------------------- | --------------------------------------------- |
| 11.1 Home (proyectos recientes)          | `index.vue`                                   |
| 11.2 Configuración (API keys)            | `settings.vue`                                |
| 11.3 Dashboard del proyecto              | `projects/[id].vue`                           |
| 11.6 Editor de escena                    | `projects/[id]/scenes/[sceneId].vue`          |
| 11.4 Importador de guion                 | (montado como modal en 11.3 vía `ImportScriptModal`) |
| 11.5 Revisión de importación             | (stub `ImportReview`, montará en `[id].vue` cuando DeepSeek esté)|
| 11.7 Editor de personajes                | (stub `CharacterEditor`, modal en `[id].vue`) |
| 11.8 Biblioteca de assets                | (stub `AudioAssetLibrary`, pestaña pendiente) |
| 11.9 Timeline                            | (stub `TimelineEditor`, pestaña pendiente)    |
| 11.10 Cola de render                     | `RenderQueue` (panel en `[sceneId].vue`)      |

## Convenciones

- Las páginas son **delgadas**: orquestan composables y delegan render a
  componentes. Toda lógica reutilizable vive en composables.
- Usar `useTauri().isTauri` para detectar si estamos en el WebView de Tauri
  o en `bun run dev` puro (navegador). En el segundo caso, ocultar acciones
  destructivas o mostrar un `UAlert` con el aviso.
- `onMounted(load)` para carga inicial. Errores se atrapan y muestran con
  `UAlert color="error"`; nunca se silencian.
- Los componentes auto-importan con `pathPrefix: false`
  (ver ADR 0010). Referenciarlos por nombre de fichero.

## Tipado

Importar tipos desde `~/types/domain` (alias resuelto por Nuxt). Nunca
duplicar interfaces ad-hoc en una página; si una vista necesita un tipo que
no existe, añadirlo a `domain.ts`.

## Estado actual

4 páginas implementadas como scaffold de MVP 1:

- `index.vue`: lista proyectos, crear nuevo (con picker de carpeta vía
  `@tauri-apps/plugin-dialog`), abrir existente.
- `settings.vue`: configuración de API keys (DeepSeek, Gemini) con
  guardar/borrar/probar.
- `projects/[id].vue`: dashboard con escenas, personajes, botón importar.
- `projects/[id]/scenes/[sceneId].vue`: editor con `SceneToolbar`,
  `SceneEditor`, `CharacterPanel` compacto y `RenderQueue`.
