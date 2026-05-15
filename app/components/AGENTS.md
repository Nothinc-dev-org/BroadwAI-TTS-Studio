# AGENTS.md — `app/components/`

## Propósito

Componentes Vue de UI, organizados por dominio. Auto-import con
**`pathPrefix: false`** (ver ADR 0010): el nombre del componente es el del
fichero, no del path.

## Organización por dominio

| Carpeta       | Dominio                                   |
| ------------- | ----------------------------------------- |
| `project/`    | Tarjeta y formulario de proyecto          |
| `import/`     | Modal de importación + previews/reviews   |
| `scene/`      | Editor, toolbar, lista y bloque de diálogo|
| `character/`  | Panel, editor, selector de voz            |
| `audio/`      | Player, cola de render, biblioteca        |
| `timeline/`   | Editor, pista, evento                     |

## Convenciones

- **`<script setup lang="ts">`** en todos los componentes.
- Props: declarar con `defineProps<{}>()` tipado (sin runtime declaration).
- Emits: declarar con `defineEmits<{}>()`.
- Modelos: usar `defineModel()` (Vue 3.4+) en vez de prop + emit manuales.
- Nuxt UI 3 components con prefijo `U` (`UButton`, `UCard`, `UFormField`).
- Iconos de Lucide vía `i-lucide-*`; otros sets disponibles vía Nuxt Icon.

## Reglas críticas

1. **No invocar `invoke` directamente**. Pasar siempre por un composable de
   `~/composables/`. Esto facilita mocks y mantiene la capa de transport
   centralizada.
2. **No persistir credenciales** (ni siquiera transitoriamente con `ref`
   vivo después del submit). Limpiar el input password tras `set_api_key`.
   Ver `settings.vue` como ejemplo.
3. **Eventos `imported`, `created`, etc.** son emit-only para que el padre
   recargue su estado. Los componentes no tocan estado global.

## Lista de componentes y su pantalla

| Componente              | Pantalla RF | Estado |
| ----------------------- | ----------- | ------ |
| `ProjectCard`           | 11.1        | ✅     |
| `ProjectForm`           | 11.1        | ✅     |
| `ImportScriptModal`     | 11.3 / 11.4 | ✅ texto/archivo, DeepSeek pendiente |
| `ImportPreview`         | 11.4        | ✅ stub |
| `ImportReview`          | 11.5        | 🟦 placeholder |
| `SceneToolbar`          | 11.6        | ✅ botones, lógica TTS pendiente |
| `SceneEditor`           | 11.6        | ✅ orquesta `DialogueList` |
| `DialogueList`          | 11.6        | ✅     |
| `DialogueBlock`         | 11.6        | ✅ acciones aún no cableadas |
| `CharacterPanel`        | 11.3 / 11.6 | ✅ list + compact |
| `CharacterEditor`       | 11.7        | ✅ formulario, submit no cableado |
| `VoiceSelector`         | 11.7        | ✅     |
| `AudioPlayer`           | 11.6 / 11.8 | ✅     |
| `RenderQueue`           | 11.10       | 🟦 placeholder |
| `AudioAssetLibrary`     | 11.8        | ✅ stub |
| `TimelineEditor`        | 11.9        | ✅ stub |
| `TimelineTrack`         | 11.9        | ✅     |
| `TimelineEvent`         | 11.9        | ✅     |

Leyenda: ✅ implementado · 🟦 stub
