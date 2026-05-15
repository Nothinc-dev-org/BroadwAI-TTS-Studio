# Documento de Requerimientos

## Aplicación de escritorio para creación de escenas TTS con N personajes usando Tauri.rs, Nuxt 4, Nuxt UI, DeepSeek V4 Flash y Gemini 3.1 Flash TTS Preview

---

## 1. Información general del producto

### 1.1 Nombre provisional

**BroadwAI TTS Studio**

### 1.2 Tipo de aplicación

Aplicación de escritorio local-first para creación, estructuración, edición, generación y mezcla de escenas narrativas con múltiples personajes mediante modelos de lenguaje y modelos Text-to-Speech.

### 1.3 Stack tecnológico definido

* **Desktop shell:** Tauri.rs
* **Backend local:** Rust
* **ORM:** SeaORM
* **Base de datos:** SQLite
* **Frontend:** Nuxt 4
* **UI framework:** Nuxt UI
* **LLM estructurador:** DeepSeek V4 Flash vía API key
* **TTS:** Gemini 3.1 Flash TTS Preview vía API key
* **Persistencia:** Local
* **Arquitectura:** Local-first, sin backend remoto propio para MVP

---

## 2. Objetivo del sistema

La aplicación debe permitir al usuario importar guiones narrativos o literarios, procesarlos mediante DeepSeek V4 Flash para convertirlos en una estructura formal compatible con Gemini 3.1 Flash TTS Preview, editar visualmente las escenas resultantes y generar audio por diálogo o audio final de escena.

El sistema debe abstraer las limitaciones internas de los proveedores de TTS. De cara al usuario, una escena debe poder contener N personajes y N diálogos. Internamente, la aplicación deberá dividir, agrupar, renderizar y mezclar el audio de acuerdo con las restricciones reales del proveedor utilizado.

---

## 3. Alcance general

### 3.1 Incluido en el alcance

La aplicación debe permitir:

* Crear proyectos locales.
* Crear escenas dentro de proyectos.
* Crear y administrar personajes.
* Importar guiones desde copy-paste.
* Importar archivos `.txt` y `.md`.
* Procesar guiones mediante DeepSeek V4 Flash.
* Convertir guiones a bloques estructurados de narrador, personajes, diálogos, pensamientos, sistema y direcciones.
* Asignar etiquetas TTS inline compatibles con Gemini.
* Editar manualmente bloques de diálogo.
* Crear escenas con N personajes.
* Crear escenas con N diálogos.
* Generar audio individual por diálogo.
* Reproducir audio individual por diálogo.
* Generar audio global de escena.
* Reproducir audio global de escena.
* Configurar delays entre diálogos.
* Importar efectos de sonido y pistas de audio.
* Insertar pistas antes, durante o después de diálogos.
* Mezclar voces, efectos, música y ambiente.
* Exportar audio final.
* Guardar todo localmente en SQLite.
* Guardar assets generados/importados en disco.
* Usar API keys configuradas por el usuario.

### 3.2 Fuera del alcance inicial

Para el MVP inicial quedan fuera:

* Sincronización en la nube.
* Edición colaborativa.
* Backend remoto multiusuario.
* Marketplace de voces.
* Publicación directa a plataformas.
* Edición avanzada tipo DAW profesional.
* Importación `.docx`, `.pdf`, `.fdx` o `.fountain`.
* Generación automática de música.
* Doblaje automático con lip sync.
* Entrenamiento de voces personalizadas.
* Sistema de cuentas de usuario.
* Facturación interna.
* Gestión de cuotas multiusuario.

---

## 4. Principios de diseño

### 4.1 Local-first

La aplicación debe funcionar principalmente de forma local.

El usuario debe poder:

* Abrir proyectos sin conexión.
* Editar escenas sin conexión.
* Administrar personajes sin conexión.
* Reproducir audios ya generados sin conexión.
* Exportar archivos ya disponibles sin conexión.

La conexión a internet solo será necesaria para:

* Procesar guiones con DeepSeek.
* Generar audio con Gemini TTS.
* Validar API keys.
* Consultar proveedores externos.

---

### 4.2 El usuario no debe conocer las limitaciones internas del TTS

De cara al usuario, el modelo conceptual debe ser:

```text
Proyecto
  → Escenas
    → N personajes
    → N diálogos
    → Audio final configurable
```

Internamente, la aplicación podrá dividir la escena en jobs de render, generar audio por diálogo, agrupar bloques, crear segmentos o reconstruir timeline según sea necesario.

---

### 4.3 DeepSeek no debe ser la fuente de verdad

DeepSeek debe actuar como motor de estructuración, no como fuente definitiva.

La fuente de verdad será:

```text
Texto original importado
+ estructura validada por la aplicación
+ edición manual del usuario
+ assets asociados
+ timeline de mezcla
```

---

### 4.4 No pérdida de texto

La aplicación debe evitar que el procesamiento con LLM elimine, resuma o pierda fragmentos del guion original.

Debe existir validación para detectar:

* Texto omitido.
* Diálogos no asignados.
* Speakers ambiguos.
* Fragmentos sin clasificar.
* JSON inválido.
* Texto reescrito indebidamente.
* Pérdida de groserías, tono, modismos o contenido narrativo.

---

### 4.5 Edición manual siempre disponible

El usuario debe poder corregir cualquier resultado generado automáticamente.

Debe poder:

* Cambiar speaker.
* Cambiar tipo de bloque.
* Editar texto.
* Editar etiquetas TTS.
* Dividir bloques.
* Fusionar bloques.
* Reordenar diálogos.
* Cambiar voz.
* Cambiar delays.
* Insertar efectos.
* Eliminar bloques.
* Regenerar audio parcial.
* Regenerar audio total.

---

## 5. Arquitectura general

### 5.1 Vista general

```text
Nuxt 4 + Nuxt UI
  ↓
Tauri Commands
  ↓
Rust Services
  ↓
SeaORM
  ↓
SQLite + Filesystem
```

Servicios externos:

```text
DeepSeek V4 Flash API
Gemini 3.1 Flash TTS Preview API
```

---

### 5.2 Componentes principales

```text
Frontend Nuxt
  ├── Editor de proyectos
  ├── Editor de escenas
  ├── Importador de guion
  ├── Editor de bloques
  ├── Panel de personajes
  ├── Biblioteca de assets
  ├── Timeline básico
  ├── Cola de render
  └── Configuración

Tauri / Rust
  ├── ProjectService
  ├── SceneService
  ├── ImportService
  ├── DeepSeekService
  ├── GeminiTtsService
  ├── RenderPlanner
  ├── AudioMixer
  ├── AssetService
  ├── TimelineService
  ├── CredentialService
  └── ExportService

SQLite
  ├── projects
  ├── scenes
  ├── characters
  ├── character_aliases
  ├── dialogue_nodes
  ├── dialogue_tts_tags
  ├── raw_imports
  ├── audio_assets
  ├── generated_audio
  ├── timeline_tracks
  ├── timeline_events
  ├── render_jobs
  └── app_settings
```

---

## 6. Modelo de datos conceptual

### 6.1 Project

Representa una obra, capítulo, episodio o colección de escenas.

```ts
Project {
  id: string
  title: string
  description?: string
  language: string
  rootPath: string
  scenes: Scene[]
  characters: Character[]
  settings: ProjectSettings
  createdAt: string
  updatedAt: string
}
```

---

### 6.2 Scene

Representa una escena narrativa editable y renderizable.

```ts
Scene {
  id: string
  projectId: string
  title: string
  description?: string
  orderIndex: number
  dialogues: DialogueNode[]
  characters: Character[]
  ttsSettings: SceneTtsSettings
  timeline: Timeline
  createdAt: string
  updatedAt: string
}
```

---

### 6.3 Character

Representa un narrador, personaje, sistema o entidad hablante.

```ts
Character {
  id: string
  projectId: string
  name: string
  role: "narrator" | "character" | "system"
  aliases: string[]
  description?: string
  color?: string
  voiceProvider: "gemini"
  voiceId?: string
  defaultStylePrompt?: string
  defaultTtsTags?: string[]
  createdAt: string
  updatedAt: string
}
```

---

### 6.4 DialogueNode

Representa un bloque narrativo, diálogo o evento textual.

```ts
DialogueNode {
  id: string
  sceneId: string
  characterId: string
  previousId?: string
  nextId?: string
  orderIndex: number
  type: "narration" | "dialogue" | "thought" | "system" | "direction"
  text: string
  rawText?: string
  emotion?: string
  intensity?: number
  isEnabled: boolean
  ttsTags: TtsTag[]
  createdAt: string
  updatedAt: string
}
```

---

### 6.5 TtsTag

Representa una etiqueta inline de interpretación TTS.

```ts
TtsTag {
  id: string
  dialogueNodeId: string
  tag: string
  position: "prefix" | "inline" | "suffix"
  orderIndex: number
  source: "ai" | "manual"
}
```

Ejemplos:

```text
[neutral]
[warm]
[short pause]
[tension]
[panic]
[angry]
[whispers]
[calm]
[frustration]
```

---

### 6.6 AudioAsset

Representa un archivo de audio importado o generado.

```ts
AudioAsset {
  id: string
  projectId: string
  name: string
  type: "sound_effect" | "music" | "ambience" | "voice" | "generated"
  filePath: string
  durationMs?: number
  originalFileName?: string
  mimeType?: string
  createdAt: string
}
```

---

### 6.7 GeneratedAudio

Representa audio TTS generado para un bloque.

```ts
GeneratedAudio {
  id: string
  dialogueNodeId: string
  provider: "gemini"
  model: string
  voiceId: string
  inputHash: string
  filePath: string
  durationMs?: number
  status: "pending" | "generated" | "failed" | "outdated"
  errorMessage?: string
  createdAt: string
}
```

---

### 6.8 TimelineTrack

Representa una pista dentro del timeline de una escena.

```ts
TimelineTrack {
  id: string
  sceneId: string
  name: string
  type: "voice" | "sfx" | "music" | "ambience"
  orderIndex: number
  volume: number
  muted: boolean
  solo: boolean
}
```

---

### 6.9 TimelineEvent

Representa un evento de audio dentro del timeline.

```ts
TimelineEvent {
  id: string
  sceneId: string
  trackId: string
  dialogueNodeId?: string
  audioAssetId?: string
  generatedAudioId?: string
  startMs: number
  durationMs?: number
  offsetMs?: number
  volume: number
  fadeInMs?: number
  fadeOutMs?: number
  playbackRate: number
  loop: boolean
  createdAt: string
  updatedAt: string
}
```

---

### 6.10 RenderJob

Representa una tarea de generación o mezcla.

```ts
RenderJob {
  id: string
  sceneId: string
  dialogueNodeId?: string
  type: "tts_dialogue" | "tts_batch" | "mix_scene" | "export_project"
  provider: "deepseek" | "gemini" | "local"
  model?: string
  status: "queued" | "running" | "completed" | "failed" | "cancelled"
  inputPayload: string
  outputPath?: string
  errorMessage?: string
  createdAt: string
  updatedAt: string
}
```

---

## 7. Modelo de base de datos propuesto

### 7.1 projects

```sql
CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  language TEXT NOT NULL DEFAULT 'es-MX',
  root_path TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

---

### 7.2 scenes

```sql
CREATE TABLE scenes (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  order_index INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

---

### 7.3 characters

```sql
CREATE TABLE characters (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  name TEXT NOT NULL,
  role TEXT NOT NULL,
  description TEXT,
  color TEXT,
  voice_provider TEXT,
  voice_id TEXT,
  default_style_prompt TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

---

### 7.4 character_aliases

```sql
CREATE TABLE character_aliases (
  id TEXT PRIMARY KEY,
  character_id TEXT NOT NULL,
  alias TEXT NOT NULL,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);
```

---

### 7.5 raw_imports

```sql
CREATE TABLE raw_imports (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  scene_id TEXT,
  source_type TEXT NOT NULL,
  source_file_path TEXT,
  original_text TEXT NOT NULL,
  processed_json TEXT,
  status TEXT NOT NULL,
  error_message TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE SET NULL
);
```

---

### 7.6 dialogue_nodes

```sql
CREATE TABLE dialogue_nodes (
  id TEXT PRIMARY KEY,
  scene_id TEXT NOT NULL,
  character_id TEXT NOT NULL,
  previous_id TEXT,
  next_id TEXT,
  order_index INTEGER NOT NULL,
  type TEXT NOT NULL,
  text TEXT NOT NULL,
  raw_text TEXT,
  emotion TEXT,
  intensity INTEGER,
  is_enabled INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE RESTRICT,
  FOREIGN KEY (previous_id) REFERENCES dialogue_nodes(id) ON DELETE SET NULL,
  FOREIGN KEY (next_id) REFERENCES dialogue_nodes(id) ON DELETE SET NULL
);
```

---

### 7.7 dialogue_tts_tags

```sql
CREATE TABLE dialogue_tts_tags (
  id TEXT PRIMARY KEY,
  dialogue_node_id TEXT NOT NULL,
  tag TEXT NOT NULL,
  position TEXT NOT NULL,
  order_index INTEGER NOT NULL DEFAULT 0,
  source TEXT NOT NULL,
  FOREIGN KEY (dialogue_node_id) REFERENCES dialogue_nodes(id) ON DELETE CASCADE
);
```

---

### 7.8 audio_assets

```sql
CREATE TABLE audio_assets (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  name TEXT NOT NULL,
  type TEXT NOT NULL,
  file_path TEXT NOT NULL,
  duration_ms INTEGER,
  original_file_name TEXT,
  mime_type TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

---

### 7.9 generated_audio

```sql
CREATE TABLE generated_audio (
  id TEXT PRIMARY KEY,
  dialogue_node_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  model TEXT NOT NULL,
  voice_id TEXT NOT NULL,
  input_hash TEXT NOT NULL,
  file_path TEXT NOT NULL,
  duration_ms INTEGER,
  status TEXT NOT NULL,
  error_message TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (dialogue_node_id) REFERENCES dialogue_nodes(id) ON DELETE CASCADE
);
```

---

### 7.10 timeline_tracks

```sql
CREATE TABLE timeline_tracks (
  id TEXT PRIMARY KEY,
  scene_id TEXT NOT NULL,
  name TEXT NOT NULL,
  type TEXT NOT NULL,
  order_index INTEGER NOT NULL DEFAULT 0,
  volume REAL NOT NULL DEFAULT 1.0,
  muted INTEGER NOT NULL DEFAULT 0,
  solo INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE
);
```

---

### 7.11 timeline_events

```sql
CREATE TABLE timeline_events (
  id TEXT PRIMARY KEY,
  scene_id TEXT NOT NULL,
  track_id TEXT NOT NULL,
  dialogue_node_id TEXT,
  audio_asset_id TEXT,
  generated_audio_id TEXT,
  start_ms INTEGER NOT NULL,
  duration_ms INTEGER,
  offset_ms INTEGER DEFAULT 0,
  volume REAL NOT NULL DEFAULT 1.0,
  fade_in_ms INTEGER DEFAULT 0,
  fade_out_ms INTEGER DEFAULT 0,
  playback_rate REAL NOT NULL DEFAULT 1.0,
  loop INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE,
  FOREIGN KEY (track_id) REFERENCES timeline_tracks(id) ON DELETE CASCADE,
  FOREIGN KEY (dialogue_node_id) REFERENCES dialogue_nodes(id) ON DELETE SET NULL,
  FOREIGN KEY (audio_asset_id) REFERENCES audio_assets(id) ON DELETE SET NULL,
  FOREIGN KEY (generated_audio_id) REFERENCES generated_audio(id) ON DELETE SET NULL
);
```

---

### 7.12 render_jobs

```sql
CREATE TABLE render_jobs (
  id TEXT PRIMARY KEY,
  scene_id TEXT,
  dialogue_node_id TEXT,
  type TEXT NOT NULL,
  provider TEXT NOT NULL,
  model TEXT,
  status TEXT NOT NULL,
  input_payload TEXT NOT NULL,
  output_path TEXT,
  error_message TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE,
  FOREIGN KEY (dialogue_node_id) REFERENCES dialogue_nodes(id) ON DELETE CASCADE
);
```

---

### 7.13 app_settings

```sql
CREATE TABLE app_settings (
  id TEXT PRIMARY KEY,
  key TEXT NOT NULL UNIQUE,
  value TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

---

## 8. Requerimientos funcionales

---

## RF-01 — Crear proyecto

La aplicación debe permitir crear un proyecto local.

### Campos requeridos

* Título.
* Descripción opcional.
* Idioma principal.
* Carpeta local del proyecto.

### Resultado esperado

Al crear un proyecto, la app debe:

* Crear registro en SQLite.
* Crear carpeta raíz del proyecto.
* Crear subcarpetas internas.
* Inicializar configuración básica.
* Crear pistas base si corresponde.

### Estructura de carpeta sugerida

```text
/project-root
  /database
    project.sqlite
  /audio
    /generated
    /exports
  /assets
    /sfx
    /music
    /ambience
  /imports
  /cache
```

---

## RF-02 — Abrir proyecto

La aplicación debe permitir abrir proyectos existentes desde disco.

Debe validar:

* Existencia de base SQLite.
* Compatibilidad de versión.
* Existencia de carpetas requeridas.
* Integridad básica de assets.

---

## RF-03 — Configurar API keys

La aplicación debe permitir configurar:

* API key de DeepSeek.
* API key de Gemini / Google AI Studio / Vertex AI.

### Requerimientos

* Las API keys no deben guardarse en texto plano dentro de SQLite.
* Las API keys deben guardarse usando almacenamiento seguro del sistema operativo cuando sea posible.
* El frontend no debe persistir API keys en `localStorage`.
* El backend Rust debe ser responsable de leer y usar las API keys.
* Debe existir botón para probar conexión.
* Debe existir botón para borrar key.
* Debe mostrarse estado de configuración.

### Estados posibles

```text
No configurada
Configurada
Válida
Inválida
Error de conexión
```

---

## RF-04 — Crear escena manualmente

La aplicación debe permitir crear una escena dentro de un proyecto.

### Campos

* Título.
* Descripción.
* Orden.
* Personajes participantes.
* Configuración TTS inicial.
* Timeline inicial.

---

## RF-05 — Editar escena

La aplicación debe permitir editar:

* Título.
* Descripción.
* Orden.
* Personajes participantes.
* Bloques de diálogo.
* Timeline.
* Configuración de audio.
* Configuración de TTS.

---

## RF-06 — Crear personajes

La aplicación debe permitir crear personajes manualmente.

### Campos

* Nombre.
* Rol.
* Alias.
* Descripción.
* Color.
* Voz asignada.
* Tags TTS por defecto.
* Prompt de estilo.

### Roles válidos

```text
narrator
character
system
```

---

## RF-07 — Detectar personajes automáticamente

Al importar un guion, la aplicación debe crear personajes detectados por DeepSeek.

Debe detectar y normalizar:

* Narrador.
* Personajes con diálogo explícito.
* Personajes en formato chat.
* Sistema.
* Voces internas si se decide representarlas como personaje.
* Alias.

---

## RF-08 — Administrar alias de personajes

La aplicación debe permitir asociar múltiples alias a un personaje.

Ejemplo:

```text
Doctor Fraga
Fraga
José Fraga
El doctor
Doctor
```

Todos pueden apuntar al mismo personaje.

---

## RF-09 — Importar guion por copy-paste

La aplicación debe permitir pegar texto completo en un importador.

Debe aceptar:

* Texto plano.
* Markdown.
* Guion literario.
* Narración en tercera persona.
* Diálogos con guion largo.
* Diálogos entre comillas.
* Diálogos en formato chat.
* Pensamientos internos.
* Acotaciones.
* Groserías.
* Modismos.
* Texto largo.

---

## RF-10 — Importar guion desde archivo

La aplicación debe permitir importar archivos:

```text
.txt
.md
```

La app debe leer el archivo localmente y almacenar una copia del texto original en `raw_imports`.

---

## RF-11 — Procesar guion con DeepSeek V4 Flash

La aplicación debe enviar el texto importado a DeepSeek V4 Flash para estructurarlo.

### El procesamiento debe producir

* Título sugerido de escena.
* Descripción de escena.
* Lista de personajes detectados.
* Lista de diálogos.
* Tipo de cada bloque.
* Speaker de cada bloque.
* Texto original preservado.
* Etiquetas TTS sugeridas.
* Fragmentos no asignados, si existen.

---

## RF-12 — Formato de salida esperado de DeepSeek

La aplicación debe solicitar a DeepSeek una respuesta JSON válida con esta estructura conceptual:

```json
{
  "scene": {
    "title": "string",
    "description": "string",
    "language": "es-MX",
    "characters": [
      {
        "name": "Narrador",
        "role": "narrator",
        "aliases": [],
        "description": "Narrador literario en tercera persona."
      }
    ],
    "dialogues": [
      {
        "speaker": "Narrador",
        "type": "narration",
        "tts_tags": ["[warm]"],
        "text": "Texto del bloque.",
        "original_excerpt": "Texto original correspondiente."
      }
    ],
    "unassigned_fragments": []
  }
}
```

---

## RF-13 — Prompt base para DeepSeek

La app debe usar un prompt de sistema equivalente a:

```text
You are a screenplay-to-TTS structuring engine.

Your task is to convert the user's Spanish prose/script into a strict JSON scene format for Gemini 3.1 Flash TTS.

Rules:
- Do not summarize.
- Do not invent dialogue.
- Do not remove profanity, slang, violence, tension, or character tone.
- Preserve the original meaning and order.
- Split narration and spoken dialogue into separate blocks.
- Assign every block to a speaker.
- Use "Narrador" for third-person narration.
- Detect chat-message dialogue as spoken dialogue unless marked otherwise.
- Add Gemini TTS inline tags in English, using bracket syntax, such as [neutral], [warm], [short pause], [tension], [panic], [angry], [whispers].
- Keep tags minimal and meaningful.
- Return only valid JSON.
- No markdown.
- No explanation.
```

---

## RF-14 — Validar respuesta de DeepSeek

La aplicación debe validar:

* Que la respuesta sea JSON válido.
* Que exista `scene`.
* Que exista lista de personajes.
* Que exista lista de diálogos.
* Que cada diálogo tenga speaker.
* Que cada speaker exista como personaje o pueda crearse.
* Que cada bloque tenga texto.
* Que las etiquetas TTS tengan formato válido.
* Que no haya bloques vacíos.
* Que no se haya perdido texto relevante.

---

## RF-15 — Reparar respuesta de DeepSeek

Si DeepSeek devuelve JSON inválido o incompleto, la aplicación debe permitir:

* Reintentar procesamiento.
* Intentar reparación automática.
* Mostrar respuesta cruda.
* Permitir corrección manual.
* Cancelar importación.
* Crear escena con advertencias.

---

## RF-16 — Crear escena desde importación

Una vez validado el JSON, la app debe crear:

* Escena.
* Personajes detectados.
* Alias.
* DialogueNodes.
* Tags TTS.
* Lista enlazada de diálogos.
* Timeline base.

---

## RF-17 — Crear lista enlazada de diálogos

La aplicación debe conectar los bloques de diálogo usando:

* `previous_id`
* `next_id`
* `order_index`

Debe mantener consistencia cuando el usuario:

* Reordene bloques.
* Elimine bloques.
* Inserte bloques.
* Divida bloques.
* Fusione bloques.

---

## RF-18 — Editor visual de diálogos

La aplicación debe mostrar cada diálogo como un bloque editable.

Cada bloque debe mostrar:

* Speaker.
* Tipo de bloque.
* Texto.
* Tags TTS.
* Estado de audio.
* Botón play.
* Botón regenerar.
* Botón dividir.
* Botón fusionar.
* Botón eliminar.
* Botón duplicar.
* Control de delay.
* Control de volumen.
* Estado de validación.

---

## RF-19 — Tipos de bloque

Los bloques deben poder clasificarse como:

```text
narration
dialogue
thought
system
direction
```

### narration

Texto narrado por narrador.

### dialogue

Texto hablado por personaje.

### thought

Pensamiento interno. Puede ser narrado por el personaje o por el narrador según configuración.

### system

Mensajes de sistema, pantallas, notificaciones o interfaces.

### direction

Acotaciones que no necesariamente deben generar audio.

---

## RF-20 — Editar texto de diálogo

El usuario debe poder editar manualmente el texto de cualquier bloque.

Al editar texto:

* El audio generado debe marcarse como desactualizado.
* El hash del bloque debe invalidarse.
* La mezcla final debe marcarse como desactualizada.
* Los assets externos no deben alterarse.

---

## RF-21 — Editar etiquetas TTS

El usuario debe poder:

* Agregar etiquetas.
* Eliminar etiquetas.
* Reordenar etiquetas.
* Editar etiquetas.
* Cambiar posición de etiquetas.
* Marcar etiquetas como manuales.

Al modificar etiquetas, el audio TTS asociado debe marcarse como desactualizado.

---

## RF-22 — Optimización de performance TTS con DeepSeek

La aplicación debe incluir una función opcional para optimizar etiquetas TTS sin alterar el texto.

### Reglas

DeepSeek podrá:

* Ajustar etiquetas TTS.
* Añadir pausas.
* Añadir emoción.
* Ajustar intensidad.
* Sugerir tags más adecuados.

DeepSeek no podrá:

* Cambiar texto.
* Cambiar speakers.
* Cambiar orden.
* Resumir.
* Eliminar contenido.
* Suavizar lenguaje.
* Corregir estilo literario.

### Resultado esperado

La app debe aplicar cambios solo en etiquetas TTS y metadatos de performance.

---

## RF-23 — Asignar voz por personaje

La aplicación debe permitir asignar una voz TTS a cada personaje.

Cada personaje debe poder tener:

* Proveedor.
* Modelo.
* Voice ID.
* Prompt de estilo opcional.
* Tags por defecto.

---

## RF-24 — Generar audio por diálogo

Cada bloque debe tener un botón para generar audio individual.

### Flujo

```text
Usuario presiona generar/play
  ↓
La app revisa hash
  ↓
Si audio existe y está vigente, reproduce
  ↓
Si no existe o está desactualizado, genera con Gemini
  ↓
Guarda audio
  ↓
Reproduce audio
```

---

## RF-25 — Play por diálogo

Cada bloque debe tener botón de reproducción.

Si el audio no existe, la app debe:

* Generarlo automáticamente según configuración, o
* Preguntar al usuario, según preferencia configurada.

---

## RF-26 — Generar audio global de escena

La escena debe tener un botón para generar audio completo.

La app debe:

* Detectar audios faltantes.
* Generar audios faltantes.
* Reutilizar audios cacheados.
* Construir timeline.
* Aplicar delays.
* Insertar efectos.
* Mezclar pistas.
* Exportar preview interno.
* Permitir reproducción global.

---

## RF-27 — Play global

La escena debe tener botón de reproducción global.

Debe reproducir la mezcla completa de:

* Voces generadas.
* Delays.
* Efectos de sonido.
* Música.
* Ambiente.
* Fades.
* Volúmenes.

---

## RF-28 — Exportar audio final

La aplicación debe permitir exportar audio final de escena.

Formatos MVP:

```text
.wav
.mp3
```

Formatos futuros:

```text
.ogg
.flac
```

---

## RF-29 — Render planner interno

La aplicación debe incluir un componente interno responsable de decidir cómo generar el audio.

### Responsabilidades

* Revisar limitaciones del proveedor.
* Dividir escena en jobs.
* Generar audio por bloque.
* Agrupar bloques cuando sea posible.
* Evitar regeneraciones innecesarias.
* Crear cola de render.
* Reportar errores.
* Reintentar jobs fallidos.

---

## RF-30 — Caché de audio

La app debe evitar regenerar audio si el bloque no cambió.

El hash de entrada debe considerar:

* Texto.
* Speaker.
* Voice ID.
* Modelo.
* Tags TTS.
* Prompt de estilo.
* Configuración relevante de generación.

---

## RF-31 — Estados de audio por bloque

Cada bloque debe mostrar estado:

```text
Sin generar
Generando
Generado
Desactualizado
Error
```

---

## RF-32 — Importar assets de audio

La aplicación debe permitir importar archivos:

```text
.wav
.mp3
.ogg
.flac
```

Tipos de asset:

```text
sound_effect
music
ambience
voice
generated
```

---

## RF-33 — Biblioteca de assets

La aplicación debe tener una biblioteca de assets por proyecto.

Debe permitir:

* Ver assets.
* Renombrar assets.
* Clasificar assets.
* Reproducir assets.
* Eliminar assets.
* Insertar assets en timeline.
* Ver duración.
* Ver ruta local.

---

## RF-34 — Insertar efectos de sonido

El usuario debe poder insertar efectos:

* Antes de un diálogo.
* Después de un diálogo.
* Encima de un diálogo.
* Entre dos diálogos.
* En una pista independiente.

---

## RF-35 — Delays configurables

La aplicación debe permitir configurar delays.

### Delay global de escena

Aplica por defecto entre diálogos.

```text
defaultDialogueGapMs
```

### Delay por diálogo

Cada bloque puede tener:

```text
beforeDelayMs
afterDelayMs
```

### Delay por evento de timeline

Cada evento puede tener posición absoluta o relativa:

```text
startMs
offsetMs
```

---

## RF-36 — Timeline básico

La aplicación debe tener un timeline básico con pistas.

Pistas mínimas:

```text
Voces
SFX
Música
Ambiente
```

El timeline debe permitir:

* Ver eventos.
* Mover eventos.
* Cambiar duración cuando aplique.
* Cambiar volumen.
* Aplicar fade in.
* Aplicar fade out.
* Silenciar pista.
* Reproducir escena.

---

## RF-37 — Mixer básico

La aplicación debe permitir:

* Volumen por pista.
* Volumen por evento.
* Mute por pista.
* Solo por pista.
* Fade in por evento.
* Fade out por evento.
* Loop para ambientes.
* Playback rate básico.
* Exportar mezcla.

---

## RF-38 — Reconstruir mezcla sin regenerar voces

Editar timeline no debe obligar a regenerar audio TTS.

Cambios que no invalidan TTS:

* Delay.
* Volumen.
* Fade.
* Orden de timeline.
* Insertar SFX.
* Insertar música.
* Insertar ambiente.
* Mute/solo.

Cambios que sí invalidan TTS:

* Texto.
* Speaker.
* Voz.
* Modelo.
* Tags TTS.
* Prompt de estilo.

---

## RF-39 — Exportar proyecto

La aplicación debe permitir exportar:

* Proyecto completo en JSON.
* Escena en JSON.
* Guion Gemini TTS en TXT.
* Audio final.
* Audios individuales.
* Metadata de personajes.

---

## RF-40 — Importar proyecto

La aplicación debe permitir importar un proyecto exportado.

Debe restaurar:

* Proyecto.
* Escenas.
* Personajes.
* Diálogos.
* Tags.
* Assets.
* Timeline.
* Audio generado, si está incluido.

---

## 9. Requerimientos no funcionales

---

## RNF-01 — Seguridad de API keys

Las API keys deben guardarse de forma segura.

No deben:

* Guardarse en texto plano en SQLite.
* Imprimirse en logs.
* Exponerse en errores.
* Persistirse en localStorage.
* Enviarse al frontend salvo estrictamente necesario.

---

## RNF-02 — Rendimiento

La aplicación debe manejar escenas largas con cientos o miles de bloques.

Debe evitar:

* Renderizar toda la lista si no es necesario.
* Regenerar audios ya cacheados.
* Bloquear la UI durante procesamiento.
* Cargar todos los audios en memoria simultáneamente.

---

## RNF-03 — Escalabilidad local

La aplicación debe soportar proyectos grandes.

Objetivo inicial:

```text
100 escenas por proyecto
500 personajes por proyecto
10,000 bloques de diálogo por proyecto
Miles de assets de audio
```

---

## RNF-04 — Resiliencia

La aplicación debe tolerar:

* Fallos de red.
* API keys inválidas.
* Respuestas inválidas de DeepSeek.
* Errores de Gemini TTS.
* Assets faltantes.
* Audio corrupto.
* Jobs interrumpidos.
* Cierre inesperado de la app.

---

## RNF-05 — Trazabilidad

La aplicación debe guardar metadatos sobre:

* Texto original importado.
* Fecha de procesamiento.
* Modelo usado.
* Prompt usado.
* Respuesta cruda de DeepSeek.
* Errores de validación.
* Parámetros de generación TTS.
* Hashes de audio.

---

## RNF-06 — Compatibilidad de plataforma

Tauri debe permitir compilar para:

* Linux
* Windows
* macOS

Para MVP se prioriza Linux y Windows.

---

## RNF-07 — Usabilidad

La interfaz debe permitir:

* Flujo claro de importación.
* Edición rápida de bloques.
* Vista limpia de personajes.
* Reproducción inmediata.
* Estados visibles.
* Mensajes de error claros.
* Deshacer/rehacer en operaciones editoriales, si es viable.

---

## RNF-08 — No bloqueo de UI

Las operaciones largas deben ejecutarse de forma asíncrona:

* Importar archivo largo.
* Procesar con DeepSeek.
* Generar audio.
* Mezclar escena.
* Exportar audio.

La UI debe mostrar progreso.

---

## RNF-09 — Integridad de datos

La aplicación debe mantener consistencia entre:

* DialogueNodes.
* Lista enlazada.
* TimelineEvents.
* GeneratedAudio.
* Characters.
* AudioAssets.

---

## RNF-10 — Versionado de base de datos

SeaORM debe manejar migraciones.

Cada versión de la app debe poder:

* Crear nuevas tablas.
* Migrar datos existentes.
* Validar versión de schema.
* Evitar abrir proyectos incompatibles sin migración.

---

## 10. Flujos principales

---

## 10.1 Flujo de creación de proyecto

```text
Usuario abre la app
  ↓
Selecciona "Nuevo proyecto"
  ↓
Ingresa título, descripción y carpeta
  ↓
La app crea estructura local
  ↓
La app crea SQLite
  ↓
La app registra proyecto
  ↓
La app abre dashboard del proyecto
```

---

## 10.2 Flujo de importación por copy-paste

```text
Usuario abre importador
  ↓
Pega guion
  ↓
Selecciona modo de importación
  ↓
Presiona "Procesar"
  ↓
La app guarda raw_import
  ↓
La app envía texto a DeepSeek
  ↓
DeepSeek devuelve JSON estructurado
  ↓
La app valida JSON
  ↓
La app muestra revisión
  ↓
Usuario acepta
  ↓
La app crea escena, personajes y bloques
```

---

## 10.3 Flujo de importación desde archivo

```text
Usuario abre importador
  ↓
Selecciona archivo .txt o .md
  ↓
La app lee archivo
  ↓
La app muestra preview
  ↓
Usuario confirma
  ↓
La app procesa con DeepSeek
  ↓
La app valida respuesta
  ↓
La app crea escena
```

---

## 10.4 Flujo de edición de escena

```text
Usuario abre escena
  ↓
La app carga personajes, diálogos y timeline
  ↓
Usuario edita bloques
  ↓
La app guarda cambios
  ↓
La app invalida audio si corresponde
  ↓
Usuario reproduce o regenera
```

---

## 10.5 Flujo de generación por diálogo

```text
Usuario presiona Play en bloque
  ↓
La app revisa si existe audio vigente
  ↓
Si existe, reproduce
  ↓
Si no existe, crea render_job
  ↓
Gemini genera audio
  ↓
La app guarda archivo
  ↓
La app registra generated_audio
  ↓
La app reproduce audio
```

---

## 10.6 Flujo de generación global

```text
Usuario presiona Play escena
  ↓
La app revisa todos los bloques habilitados
  ↓
Detecta audios faltantes o desactualizados
  ↓
Genera audios necesarios
  ↓
Construye timeline
  ↓
Mezcla voces, delays y assets
  ↓
Reproduce preview
```

---

## 10.7 Flujo de exportación final

```text
Usuario selecciona Exportar escena
  ↓
La app valida audios necesarios
  ↓
La app genera faltantes si corresponde
  ↓
La app renderiza timeline completo
  ↓
La app exporta WAV/MP3
  ↓
La app guarda archivo en /audio/exports
```

---

## 11. Pantallas requeridas

---

## 11.1 Pantalla Home

Debe mostrar:

* Proyectos recientes.
* Botón nuevo proyecto.
* Botón abrir proyecto.
* Acceso a configuración.

---

## 11.2 Pantalla de configuración

Debe permitir configurar:

* API key DeepSeek.
* API key Gemini.
* Modelo DeepSeek.
* Modelo Gemini TTS.
* Carpeta por defecto.
* Formato de audio por defecto.
* Delay global por defecto.
* Política de generación automática.
* Política de caché.

---

## 11.3 Dashboard de proyecto

Debe mostrar:

* Título del proyecto.
* Descripción.
* Lista de escenas.
* Lista de personajes.
* Botón importar guion.
* Botón nueva escena.
* Botón exportar proyecto.

---

## 11.4 Importador de guion

Debe incluir:

* Textarea para copy-paste.
* Botón cargar archivo.
* Preview del texto.
* Modo de importación.
* Botón procesar con DeepSeek.
* Estado de procesamiento.
* Vista de errores.

---

## 11.5 Revisión de importación

Debe mostrar:

* Título sugerido.
* Descripción sugerida.
* Personajes detectados.
* Diálogos detectados.
* Tags detectados.
* Fragmentos no asignados.
* JSON crudo opcional.
* Botón aceptar.
* Botón reprocesar.
* Botón cancelar.

---

## 11.6 Editor de escena

Debe mostrar:

* Título de escena.
* Descripción.
* Lista de bloques.
* Panel de personajes.
* Panel de tags.
* Panel de audio.
* Botón generar todo.
* Botón play global.
* Botón exportar.

---

## 11.7 Editor de personajes

Debe permitir:

* Crear personaje.
* Editar personaje.
* Eliminar personaje.
* Agregar alias.
* Asignar voz.
* Probar voz.
* Ver bloques asociados.

---

## 11.8 Biblioteca de assets

Debe permitir:

* Importar audio.
* Reproducir audio.
* Clasificar audio.
* Renombrar audio.
* Insertar en timeline.
* Eliminar audio.

---

## 11.9 Timeline

Debe mostrar pistas:

* Voces.
* SFX.
* Música.
* Ambiente.

Debe permitir:

* Insertar eventos.
* Mover eventos.
* Ajustar volumen.
* Ajustar fades.
* Configurar loops.
* Reproducir mezcla.

---

## 11.10 Cola de render

Debe mostrar:

* Jobs pendientes.
* Jobs en ejecución.
* Jobs completados.
* Jobs fallidos.
* Botón reintentar.
* Botón cancelar.
* Error detallado.

---

## 12. Comandos Tauri requeridos

### 12.1 Proyectos

```rust
create_project
open_project
list_recent_projects
update_project
delete_project
export_project
import_project
```

---

### 12.2 Escenas

```rust
create_scene
get_scene
list_scenes
update_scene
delete_scene
reorder_scenes
```

---

### 12.3 Personajes

```rust
create_character
list_characters
update_character
delete_character
add_character_alias
remove_character_alias
assign_character_voice
```

---

### 12.4 Diálogos

```rust
create_dialogue_node
list_dialogue_nodes
update_dialogue_node
delete_dialogue_node
split_dialogue_node
merge_dialogue_nodes
reorder_dialogue_nodes
update_dialogue_tts_tags
```

---

### 12.5 Importación

```rust
import_text
import_file
process_import_with_deepseek
validate_import_result
create_scene_from_import
```

---

### 12.6 TTS

```rust
generate_dialogue_audio
generate_scene_audio
regenerate_outdated_audio
play_dialogue_audio
play_scene_audio
```

---

### 12.7 Timeline

```rust
create_timeline_track
update_timeline_track
delete_timeline_track
create_timeline_event
update_timeline_event
delete_timeline_event
render_timeline
```

---

### 12.8 Assets

```rust
import_audio_asset
list_audio_assets
update_audio_asset
delete_audio_asset
preview_audio_asset
```

---

### 12.9 Configuración

```rust
set_api_key
delete_api_key
test_api_key
get_app_settings
update_app_settings
```

---

## 13. Servicios Rust requeridos

---

## 13.1 ProjectService

Responsable de:

* Crear proyectos.
* Abrir proyectos.
* Validar estructura.
* Exportar proyectos.
* Importar proyectos.

---

## 13.2 SceneService

Responsable de:

* CRUD de escenas.
* Orden de escenas.
* Relación escena/personajes.
* Configuración de escena.

---

## 13.3 CharacterService

Responsable de:

* CRUD de personajes.
* Alias.
* Voces.
* Normalización de speakers.

---

## 13.4 DialogueService

Responsable de:

* CRUD de bloques.
* Lista enlazada.
* Orden.
* Split.
* Merge.
* Tags.
* Validación.

---

## 13.5 ImportService

Responsable de:

* Leer texto pegado.
* Leer archivos `.txt` y `.md`.
* Guardar raw imports.
* Preparar payload para DeepSeek.
* Validar respuesta.
* Crear escena desde resultado.

---

## 13.6 DeepSeekService

Responsable de:

* Consumir DeepSeek API.
* Enviar prompt.
* Solicitar JSON.
* Manejar errores.
* Reintentar.
* Optimizar tags TTS.
* Guardar respuesta cruda.

---

## 13.7 GeminiTtsService

Responsable de:

* Consumir Gemini TTS API.
* Generar audio.
* Manejar voces.
* Guardar archivos.
* Reportar errores.
* Normalizar salida.

---

## 13.8 RenderPlanner

Responsable de:

* Analizar escena.
* Detectar audios faltantes.
* Detectar audios desactualizados.
* Crear jobs.
* Dividir según restricciones del proveedor.
* Orquestar render parcial o total.

---

## 13.9 AudioMixer

Responsable de:

* Construir timeline.
* Mezclar voces.
* Insertar delays.
* Insertar SFX.
* Insertar música.
* Insertar ambiente.
* Aplicar volumen.
* Aplicar fades.
* Exportar audio final.

---

## 13.10 CredentialService

Responsable de:

* Guardar API keys.
* Leer API keys.
* Eliminar API keys.
* Validar API keys.
* Evitar exposición al frontend.

---

## 14. Estructura sugerida del proyecto

```text
app/
  pages/
    index.vue
    settings.vue
    projects/
      [id].vue
      [id]/
        scenes/
          [sceneId].vue

  components/
    project/
      ProjectCard.vue
      ProjectForm.vue

    import/
      ImportScriptModal.vue
      ImportPreview.vue
      ImportReview.vue

    scene/
      SceneEditor.vue
      DialogueBlock.vue
      DialogueList.vue
      SceneToolbar.vue

    character/
      CharacterPanel.vue
      CharacterEditor.vue
      VoiceSelector.vue

    audio/
      AudioPlayer.vue
      RenderQueue.vue
      AudioAssetLibrary.vue

    timeline/
      TimelineEditor.vue
      TimelineTrack.vue
      TimelineEvent.vue

  composables/
    useProjects.ts
    useScenes.ts
    useCharacters.ts
    useDialogueNodes.ts
    useImport.ts
    useTts.ts
    useTimeline.ts
    useAssets.ts
    useSettings.ts

src-tauri/
  src/
    main.rs

    commands/
      project_commands.rs
      scene_commands.rs
      character_commands.rs
      dialogue_commands.rs
      import_commands.rs
      deepseek_commands.rs
      gemini_tts_commands.rs
      audio_commands.rs
      timeline_commands.rs
      settings_commands.rs

    services/
      project_service.rs
      scene_service.rs
      character_service.rs
      dialogue_service.rs
      import_service.rs
      deepseek_service.rs
      gemini_tts_service.rs
      render_planner.rs
      audio_mixer.rs
      asset_service.rs
      credential_service.rs

    entities/
      project.rs
      scene.rs
      character.rs
      character_alias.rs
      dialogue_node.rs
      dialogue_tts_tag.rs
      raw_import.rs
      audio_asset.rs
      generated_audio.rs
      timeline_track.rs
      timeline_event.rs
      render_job.rs
      app_setting.rs

    migrations/
      mod.rs
      m20260515_create_projects.rs
      m20260515_create_scenes.rs
      m20260515_create_characters.rs
      m20260515_create_dialogue_nodes.rs
      m20260515_create_audio_tables.rs
      m20260515_create_timeline_tables.rs
      m20260515_create_render_jobs.rs
```

---

## 15. MVP

---

## 15.1 MVP 1

El primer MVP debe incluir:

```text
1. Proyecto local con SQLite.
2. Crear y abrir proyecto.
3. Configurar API keys.
4. Importar texto por copy-paste.
5. Importar .txt y .md.
6. Procesar guion con DeepSeek V4 Flash.
7. Crear escena estructurada.
8. Crear personajes detectados.
9. Crear bloques de diálogo.
10. Editar bloques manualmente.
11. Editar tags TTS.
12. Asignar voz por personaje.
13. Generar audio por diálogo.
14. Play por diálogo.
15. Generar audio global con delays simples.
16. Play global.
17. Exportar audio final WAV/MP3.
```

---

## 15.2 MVP 2

El segundo MVP debe incluir:

```text
1. Importar efectos de sonido.
2. Biblioteca de assets.
3. Insertar SFX antes/después de diálogos.
4. Timeline visual básico.
5. Volumen por evento.
6. Fade in/fade out.
7. Re-render de mezcla sin regenerar voces.
8. Exportar escena con assets.
```

---

## 15.3 MVP 3

El tercer MVP debe incluir:

```text
1. Música y ambiente en pistas.
2. Loop de ambiente.
3. Overlap entre voces, SFX y música.
4. Normalización básica de volumen.
5. Optimización de tags TTS con DeepSeek.
6. Exportación avanzada.
7. Importación/exportación completa de proyecto.
```

---

## 16. Criterios de aceptación

### CA-01 — Crear proyecto

Dado que el usuario ingresa título y carpeta válida, cuando presiona crear proyecto, entonces la aplicación debe crear el proyecto, inicializar SQLite y abrir el dashboard.

---

### CA-02 — Importar guion pegado

Dado que el usuario pega un guion, cuando presiona procesar, entonces la app debe enviar el texto a DeepSeek y mostrar una estructura revisable.

---

### CA-03 — Crear escena desde importación

Dado que DeepSeek devuelve JSON válido, cuando el usuario acepta, entonces la app debe crear escena, personajes, diálogos y timeline base.

---

### CA-04 — Editar diálogo

Dado que el usuario modifica el texto de un diálogo, cuando guarda, entonces el audio asociado debe marcarse como desactualizado.

---

### CA-05 — Generar audio individual

Dado que un bloque no tiene audio, cuando el usuario presiona Play, entonces la app debe generar audio con Gemini, guardarlo y reproducirlo.

---

### CA-06 — Reutilizar audio cacheado

Dado que un bloque ya tiene audio vigente, cuando el usuario presiona Play, entonces la app debe reproducir el audio existente sin llamar a Gemini.

---

### CA-07 — Generar audio global

Dado que una escena tiene varios diálogos, cuando el usuario presiona Play global, entonces la app debe generar faltantes, construir mezcla y reproducir la escena completa.

---

### CA-08 — Insertar SFX

Dado que el usuario importó un efecto de sonido, cuando lo inserta antes de un diálogo, entonces debe aparecer en el timeline y sonar en la mezcla global.

---

### CA-09 — Cambiar delay

Dado que el usuario cambia el delay entre dos diálogos, cuando reproduce la escena, entonces la separación temporal debe reflejar el valor configurado sin regenerar voces.

---

### CA-10 — Exportar audio final

Dado que una escena tiene audio generado y timeline válido, cuando el usuario exporta, entonces la app debe crear un archivo WAV o MP3 reproducible.

---

## 17. Riesgos técnicos

### RT-01 — Restricciones de multi-speaker en Gemini

Gemini puede tener límites por llamada. La aplicación debe resolver esto con render por bloque, render por segmentos o render planner interno.

---

### RT-02 — JSON inválido de DeepSeek

DeepSeek puede devolver JSON inválido o incompleto. La aplicación debe validar, reparar o solicitar reprocesamiento.

---

### RT-03 — Pérdida de texto durante estructuración

El LLM puede omitir fragmentos. La app debe detectar diferencias entre texto original y texto estructurado.

---

### RT-04 — Mezcla de audio

La mezcla local requiere manejo correcto de formatos, sample rates, duración, volumen, fades y sincronización.

---

### RT-05 — Gestión de archivos

Los proyectos locales pueden romperse si el usuario mueve carpetas o elimina assets. La app debe detectar assets faltantes.

---

### RT-06 — Costos de API

Generar audio por muchos diálogos puede consumir cuota. La app debe cachear agresivamente y mostrar estados claros.

---

## 18. Roadmap recomendado

### Fase 1 — Núcleo local

* Tauri + Nuxt 4.
* SQLite + SeaORM.
* Crear/abrir proyecto.
* CRUD escenas.
* CRUD personajes.
* CRUD diálogos.

### Fase 2 — Importación inteligente

* Importar texto.
* Consumir DeepSeek.
* Validar JSON.
* Crear escena desde importación.
* Editor de bloques.

### Fase 3 — TTS por diálogo

* Configurar Gemini.
* Asignar voces.
* Generar audio por bloque.
* Play por bloque.
* Caché.

### Fase 4 — Play global

* Timeline base.
* Delays simples.
* Mezcla de voces.
* Play global.
* Export WAV/MP3.

### Fase 5 — Assets y timeline

* Biblioteca de audio.
* Insertar SFX.
* Música.
* Ambiente.
* Fades.
* Volumen.
* Timeline visual.

### Fase 6 — Refinamiento

* Optimización de tags TTS.
* Export/import proyecto.
* Validación avanzada.
* Repair tools.
* Mejoras UX.

---

## 19. Definición final del producto

La aplicación será un editor local de escenas TTS con capacidad de importar guiones narrativos, convertirlos automáticamente en bloques estructurados mediante DeepSeek V4 Flash, editarlos visualmente con Nuxt 4 y Nuxt UI, generar audio con Gemini 3.1 Flash TTS Preview y construir una mezcla final con voces, delays, efectos de sonido, música y ambiente.

La aplicación debe presentar al usuario un modelo simple:

```text
Proyecto
  → Escenas
    → Personajes
    → Diálogos
    → Audio
```

Internamente, deberá implementar:

```text
Scene Graph
  → Dialogue Nodes
  → Render Jobs
  → Generated Audio Cache
  → Timeline Events
  → Audio Mixer
  → Final Export
```

El valor central del sistema será permitir trabajar con escenas complejas de N personajes sin que el usuario tenga que preocuparse por las restricciones internas de DeepSeek, Gemini o cualquier proveedor TTS.
