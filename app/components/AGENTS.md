# AGENTS.md — `app/components/`

## Propósito

Componentes Vue de UI, organizados por dominio. Auto-import con
**`pathPrefix: false`** (ver ADR 0010): el nombre del componente es el del
fichero, no del path.

## Organización por dominio

| Carpeta       | Dominio                                       |
| ------------- | --------------------------------------------- |
| `project/`    | Tarjeta y formulario de proyecto              |
| `import/`     | Wizard de importación + revisión              |
| `scene/`      | Editor, toolbar, lista y bloque de diálogo    |
| `character/`  | Panel, editor, selector de voz                |
| `audio/`      | Player, cola de render, biblioteca de assets  |
| `timeline/`   | Editor, pista, evento                         |

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
3. **Eventos `imported`, `created`, `audioGenerated`, etc.** son emit-only
   para que el padre recargue su estado. Los componentes no tocan estado
   global.
4. **Reproducir audio generado local:** si el WebView rechaza `asset://`, leer
   los bytes vía composable Tauri y crear un `blob:` URL, revocándolo al cambiar
   de audio o desmontar el componente.

## Lista de componentes y su estado

| Componente              | Pantalla RF | Estado |
| ----------------------- | ----------- | ------ |
| `ProjectCard`           | 11.1        | ✅     |
| `ProjectForm`           | 11.1        | ✅     |
| `ImportScriptModal`     | 11.3 / 11.4 | ✅ wizard 2 etapas (importar + procesar → revisión) |
| `ImportPreview`         | 11.4        | ✅ stub útil (texto crudo) |
| `ImportReview`          | 11.5        | ✅ personajes + diálogos + warnings + crear escena |
| `SceneToolbar`          | 11.6        | ✅ generar/play global/exportar WAV/optimizar TTS |
| `SceneEditor`           | 11.6        | ✅ orquesta `DialogueList` + propaga audio |
| `DialogueList`          | 11.6        | ✅ pasa `audiosByNode` por nodo |
| `DialogueBlock`         | 11.6        | ✅ play (auto-genera), regenerar, badge de estado (RF-31), `<audio>` con `blob:` |
| `CharacterPanel`        | 11.3 / 11.6 | ✅ list + compact |
| `CharacterEditor`       | 11.7        | 🟦 formulario, submit no cableado al backend |
| `VoiceSelector`         | 11.7        | ✅     |
| `AudioPlayer`           | 11.6 / 11.8 | ✅ wrapper reutilizable |
| `RenderQueue`           | 11.10       | 🟦 placeholder (no hay `render_jobs` real todavía) |
| `AudioAssetLibrary`     | 11.8        | ✅ import + preview + delete + insertar a escena |
| `TimelineEditor`        | 11.9        | ✅ render por track con eventos |
| `TimelineTrack`         | 11.9        | ✅     |
| `TimelineEvent`         | 11.9        | ✅ mostrar start, asset, loop, quitar |

Leyenda: ✅ implementado · 🟦 placeholder / parcial
