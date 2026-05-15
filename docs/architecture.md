# Arquitectura — BroadwAI TTS Studio

> Documento canónico. Cualquier desviación del diseño aquí descrito debe pasar
> por un ADR nuevo en [`decisions/`](decisions/).

## 1. Visión general

```
┌────────────────────────────────────────────────────────────────┐
│                     Tauri 2 desktop shell                      │
│                                                                │
│  ┌──────────────────────────┐    ┌──────────────────────────┐  │
│  │     Frontend Nuxt 4      │    │      Backend Rust        │  │
│  │   (Vue 3 + Nuxt UI 3)    │◄──►│   (services + commands)  │  │
│  │                          │    │                          │  │
│  │  pages/  components/     │    │  commands/  services/    │  │
│  │  composables/  types/    │    │  entities/  migrations/  │  │
│  └──────────────────────────┘    └──────────────────────────┘  │
│            ▲                                ▲                  │
│            │   invoke (Tauri IPC)           │                  │
│            └────────────────────────────────┘                  │
└────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
       ┌─────────────────────────────────────────────────────┐
       │              SQLite por proyecto                    │
       │   <project-root>/database/project.sqlite            │
       │   13 tablas, gestionadas vía SeaORM                 │
       └─────────────────────────────────────────────────────┘
                                  │
              ┌───────────────────┼──────────────────┐
              ▼                   ▼                  ▼
       ┌─────────────┐    ┌──────────────┐   ┌─────────────────┐
       │  Keyring SO │    │ DeepSeek API │   │ Gemini TTS API  │
       │ (API keys)  │    │ V4 Flash     │   │ 3.1 Flash       │
       └─────────────┘    └──────────────┘   └─────────────────┘
```

**Local-first:** ningún backend remoto propio. El usuario es dueño de su
proyecto y sus assets. Internet solo es necesario para hablar con DeepSeek y
Gemini.

**Multi-proyecto-runtime:** la app abre **un proyecto a la vez**. Cambiar de
proyecto cierra la conexión SQLite anterior y abre la nueva (`AppState::open`).

## 2. Capas

### 2.1 Frontend (`app/`)

```
app/
├── app.vue          ← root: UApp + NuxtLayout + NuxtPage
├── layouts/
│   └── default.vue  ← header con navegación
├── pages/           ← rutas (file-based routing)
├── components/      ← UI por dominio (auto-import sin path prefix)
├── composables/     ← bindings 1:1 con comandos Tauri
└── types/
    └── domain.ts    ← tipos espejo de entidades Rust
```

Reglas:

- **`ssr: false`**. La app vive en Tauri WebView, no en un servidor.
- **No estado global** en el frontend. El estado vive en la BD; el frontend
  solo cachea y emite invalidaciones por evento.
- **Cada composable es una capa fina** sobre `invoke`. No mete lógica de
  negocio: solo serializa argumentos y devuelve el resultado tipado.
- **Sin `localStorage`** para datos del proyecto ni API keys. Solo
  preferencias triviales de UI (tema, idioma de UI).

### 2.2 Backend (`src-tauri/src/`)

```
src/
├── main.rs           ← entrypoint mínimo: delega a lib::run()
├── lib.rs            ← Tauri Builder + setup + invoke_handler!
├── error.rs          ← AppError + AppResult
├── state.rs          ← AppState (RwLock<OpenProject> + CredentialService)
├── db.rs             ← open_project_database() + migraciones
├── paths.rs          ← ProjectPaths (layout de carpetas)
├── commands/         ← capa de presentación (transport)
├── services/         ← capa de dominio (lógica)
├── entities/         ← capa de persistencia (SeaORM)
└── migrations/       ← schema (fuente de verdad)
```

**Flujo de una llamada:**

```
Vue page → useXxx().method(args)
  → invoke('xxx_command', args)
    → commands::xxx::xxx_command
      → services::xxx_service::do_thing
        → entities::xxx + SeaORM
          → SQLite (file)
```

Reglas:

- **Commands son transport, no dominio.** Solo deserializan argumentos,
  resuelven `AppState`, delegan a un servicio y devuelven `AppResult<T>`.
- **Servicios son stateless.** Reciben `&DatabaseConnection` o `&CredentialService`
  como parámetro. Nunca guardan estado mutable propio (RwLock vive en `AppState`).
- **Entidades son tontas.** Modelos SeaORM puros, sin métodos custom de negocio.
- **Errores convergen en `AppError`.** Cualquier capa puede devolver el tipo y
  se serializa al frontend como `string` (ver `error.rs`).

### 2.3 Persistencia

```
<project-root>/
├── database/project.sqlite     ← una BD por proyecto
├── audio/
│   ├── generated/              ← TTS cacheado
│   └── exports/                ← mezclas finales (.wav/.mp3)
├── assets/
│   ├── sfx/  music/  ambience/
├── imports/                    ← copias de los archivos importados
└── cache/                      ← cache de respuestas raw de DeepSeek
```

El usuario **es dueño** del directorio: puede moverlo, hacerle backup, abrir
otra instancia desde otra carpeta. Por eso `root_path` se persiste en cada
proyecto y al abrirlo (`open_project`) se valida la estructura.

## 3. Modelo de datos

13 tablas. Fuente de verdad: las migraciones en `src-tauri/src/migrations/`.

| Tabla                  | Rol                                            |
| ---------------------- | ---------------------------------------------- |
| `projects`             | Metadatos del proyecto y `root_path`           |
| `scenes`               | Escenas dentro de un proyecto                  |
| `characters`           | Narradores, personajes, sistemas hablantes     |
| `character_aliases`    | N alias por personaje (RF-08)                  |
| `raw_imports`          | Texto pegado/importado + JSON crudo de DeepSeek|
| `dialogue_nodes`       | Bloques narrativos en lista enlazada           |
| `dialogue_tts_tags`    | Tags inline (`[neutral]`, `[panic]`, …)        |
| `audio_assets`         | SFX/música/ambiente importados                 |
| `generated_audio`      | TTS cacheado, indexado por `input_hash`        |
| `timeline_tracks`      | Voces / SFX / Música / Ambiente                |
| `timeline_events`      | Evento de audio dentro del timeline            |
| `render_jobs`          | Cola de generación/mezcla                      |
| `app_settings`         | Pares clave/valor por proyecto                 |

### Invariantes

- **Lista enlazada de diálogos** (`dialogue_nodes.previous_id`/`next_id`) y
  `order_index` se mantienen sincronizados; cualquier reordenamiento toca los
  tres campos en la misma transacción.
- **`generated_audio.input_hash`** es determinístico sobre `(text, voice_id,
  model, tag_signature, style_prompt)`. Mismo hash = mismo audio reutilizable
  (RF-30). Calcular vía `services::render_planner::input_hash`.
- **Editar texto, speaker, voz, modelo, tags o prompt de estilo invalida** el
  `generated_audio` asociado (status → `outdated`). Editar delays, volumen,
  fades, orden de timeline **no** invalida (RF-38).
- **`AppError::Credential`** nunca exporta detalle del error de `keyring`; ese
  detalle solo se loggea vía `tracing::warn!`.

## 4. Flujos críticos

### 4.1 Crear proyecto (RF-01)

```
UI → useProjects().create({title, rootPath, …})
  → invoke('create_project')
    → commands::project::create_project
      → ProjectPaths::create_all (crea carpetas en disco)
      → AppState::open(root) (abre/crea SQLite + migra)
      → project_service::create (inserta fila en `projects`)
    ← Project
  ← Project
```

### 4.2 Importar guion y procesarlo (RF-09 a RF-16) — *parcial en MVP 1*

```
UI → useImport().importText(projectId, text)
  → invoke('import_text')
    → import_service::create_raw_import (status: pending)
    ← raw_import row
  ← raw_import row

UI → useImport().processWithDeepSeek(rawImportId)  ← TODO MVP 1
  → invoke('process_import_with_deepseek')
    → DeepSeekService::structure_script(text)
    → validar JSON, comparar con texto original (RF-14, RF-44)
    → guardar processed_json en raw_imports
  ← StructuredScene

UI → useImport().createScene(rawImportId)         ← TODO MVP 1
  → invoke('create_scene_from_import')
    → crear scene + characters + dialogue_nodes + tts_tags en transacción
    → crear timeline base
  ← Scene
```

### 4.3 Generar audio por diálogo (RF-24)

```
UI → useTts().playDialogue(nodeId)
  → invoke('play_dialogue_audio')
    → calcular input_hash
    → buscar GeneratedAudio vigente con ese hash
       └─ encontrado → devolver file_path
       └─ no → crear render_job (queued)
              → GeminiTtsService::synthesize
              → escribir audio/generated/<id>.wav
              → insertar generated_audio (status: generated)
              → marcar render_job como completed
    ← file_path
  ← reproducir vía AudioPlayer
```

### 4.4 Generar y exportar escena (RF-26, RF-28)

```
UI → useTts().generateScene(sceneId)
  → RenderPlanner: identificar nodos sin audio vigente
  → encolar render_jobs faltantes
  → ejecutar en paralelo controlado
  → invocar AudioMixer::render con el timeline
    → decodificar fuentes con symphonia
    → resamplear con rubato si fuera necesario
    → aplicar volumen, fades, delays
    → escribir WAV con hound (MP3 pendiente)
  ← path al .wav exportado
```

## 5. Seguridad

### API keys (RNF-01)

Almacenamiento: keyring del SO bajo el servicio `ai.broadwai.tts-studio` con
cuentas `deepseek` y `gemini`. Ver
[`decisions/0002-keyring-para-api-keys.md`](decisions/0002-keyring-para-api-keys.md).

Reglas de manejo:

- **Frontend → Backend:** solo en `set_api_key`. La key sale del input password
  del usuario, viaja por IPC al comando y se entrega a `CredentialService::set`.
- **Backend → Keyring:** vía `keyring::Entry::set_password`.
- **Backend → Provider:** la key se lee con `CredentialService::read` (pub(crate))
  solo dentro de `deepseek_service.rs` o `gemini_tts_service.rs`, se usa para
  construir el header `Authorization`, y se descarta.
- **Nunca:** persistir en SQLite, mandar al frontend, formatear en error,
  imprimir con `println!` o `dbg!`, dejar en variable de entorno.

### CSP y permisos Tauri

`tauri.conf.json` arranca con `csp: null` para acelerar el bootstrap del MVP.
**Antes de cualquier release:** definir CSP estricta (ADR pendiente).

`capabilities/default.json` da permisos por defecto a `dialog`, `fs`, `shell` y
`core`. El `fs` está sin restringir; **antes de release** restringir a las
carpetas del proyecto abierto.

## 6. Estado actual

| Capa            | Estado                                                |
| --------------- | ----------------------------------------------------- |
| Schema (13/13)  | ✅ Migraciones + entidades + tipos TS                 |
| AppState        | ✅ Funcional (single project + credentials)           |
| CredentialSrv   | ✅ Set/delete/has/status. Test API real pendiente.    |
| ProjectSrv      | 🟡 create, list, get. Update/delete/export/import: stub|
| SceneSrv        | 🟡 create, list, get. Update/delete/reorder: stub     |
| CharacterSrv    | 🟡 create, list, addAlias. Resto: stub                |
| DialogueSrv     | 🟡 list, list_tags. CRUD/split/merge/reorder: stub    |
| ImportSrv       | ✅ Texto y archivo `.txt`/`.md` persistidos           |
| DeepSeekSrv     | 🟦 Modelos + prompt definidos. HTTP real: stub        |
| GeminiTtsSrv    | 🟦 Modelos definidos. HTTP real: stub                 |
| RenderPlanner   | ✅ `input_hash` para caché (RF-30)                    |
| AudioMixer      | 🟦 Tipos definidos. Render real: stub                 |
| AssetSrv        | 🟡 list. Resto: stub                                  |
| Frontend pages  | ✅ Home, Settings, Project dashboard, Scene editor    |
| Frontend comps  | ✅ 19 componentes stub (suficientes para MVP 1)       |

Leyenda: ✅ funcional · 🟡 parcial · 🟦 firmas listas, lógica pendiente

## 7. Roadmap por sprint

| Sprint | Objetivo                                                  | RFs        |
| ------ | --------------------------------------------------------- | ---------- |
| 1      | Scaffold compilable (este sprint)                         | 01-02, 04, 06, 09-10 |
| 2      | DeepSeek real + creación de escena desde import           | 11-16      |
| 3      | Gemini TTS + caché + play por diálogo                     | 23-25, 30  |
| 4      | Play global + delays + export WAV                         | 26-28, 35-37 |
| 5      | Biblioteca de assets + SFX en timeline                    | 32-34, 36-38 |
| 6      | Optimización TTS, validaciones avanzadas, export/import   | 22, 39-40  |

## 8. Glosario

- **Bloque / DialogueNode:** unidad mínima editable; `narration`, `dialogue`,
  `thought`, `system` o `direction`.
- **TTS Tag:** etiqueta inline en bracket (`[neutral]`, `[panic]`) que Gemini
  interpreta para modular la voz.
- **Render job:** unidad de trabajo asíncrono (TTS o mezcla) persistida para
  trazabilidad y reintentos.
- **Input hash:** hash determinístico del contenido relevante de un bloque
  para evitar regenerar audio idéntico (RF-30).
