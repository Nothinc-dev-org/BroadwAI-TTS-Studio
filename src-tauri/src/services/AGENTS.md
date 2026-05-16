# AGENTS.md — `src-tauri/src/services/`

## Propósito

Capa de **dominio**. Contiene la lógica de negocio del MVP 1. Mapeo
amplio con la sección 13 de `Requerimiento.md`.

## Convención

Servicios como **funciones libres stateless** que reciben los recursos como
parámetros:

```rust
pub async fn create(db: &DatabaseConnection, input: CreateXInput) -> AppResult<X> { … }
```

No structs con `&self` salvo justificación específica. Las excepciones
actuales son:

- `CredentialService`: encapsula constantes del crate `keyring`.
- `DeepSeekService` / `GeminiTtsService`: encapsulan cliente HTTP +
  configuración (api_base, model). El struct se construye y descarta por
  llamada; sigue siendo efectivamente stateless desde el punto de vista
  de la app.

Ver `docs/decisions/0007-commands-thin-services-stateless.md`.

## Mapa de servicios

| Archivo                        | Responsabilidad                                              | Estado |
| ------------------------------ | ------------------------------------------------------------ | ------ |
| `project_service.rs`           | Crear/listar/abrir proyectos (RF-01, 02)                     | ✅     |
| `project_io_service.rs`        | Snapshot JSON export/import (RF-39, 40) — ADR-0013           | ✅     |
| `scene_service.rs`             | CRUD de escenas (RF-04, 05)                                  | 🟡 parcial |
| `character_service.rs`         | Personajes y alias (RF-06, 07, 08, 23)                       | 🟡 parcial |
| `dialogue_service.rs`          | Bloques narrativos y tags (RF-17–21)                         | 🟡 list/queries |
| `import_service.rs`            | Texto/archivo + creación de escena (RF-09–16)                | ✅     |
| `deepseek_service.rs`          | `structure_script` + `optimize_tts_tags` (RF-11–15, 22)      | ✅     |
| `gemini_tts_service.rs`        | HTTP a Gemini TTS (RF-23)                                    | ✅     |
| `tts_service.rs`               | Orquesta cache + Gemini + escritura WAV (RF-24, 25, 30, 31)  | ✅     |
| `tts_optimization_service.rs`  | Reemplaza tags + invalida audios cacheados (RF-22, 30)       | ✅     |
| `render_planner.rs`            | `input_hash` para caché TTS (RF-30) — ADR-0008               | ✅     |
| `audio_mixer.rs`               | Decoder symphonia + mezcla voice/asset tracks (RF-26, 36, 37) — ADR-0012 | ✅ |
| `scene_mix_service.rs`         | Orquesta TTS + timeline → WAV final (RF-26, 27, 28)          | ✅     |
| `timeline_service.rs`          | CRUD de tracks/eventos (RF-34, 36, 37, 38)                   | ✅     |
| `asset_service.rs`             | Biblioteca de audio + import desde disco (RF-32, 33)         | ✅     |
| `credential_service.rs`        | API keys vía keyring del SO (RF-03, RNF-01) — ADR-0002       | ✅     |

## Reglas críticas

### CredentialService — RNF-01

- `read(provider)` es `pub(crate)`: solo `deepseek_service` y
  `gemini_tts_service` deben llamarlo, y **únicamente** para construir un
  header HTTP. Nunca devuelvas la key cruda fuera del crate.
- Errores de `keyring::Error` se traducen vía `AppError::from_keyring` a
  `AppError::Credential` (sin payload). El error original se loggea con
  `tracing::warn!`.
- No introducir un getter público de la key. Ningún caso de uso lo necesita.

Ver `docs/decisions/0002-keyring-para-api-keys.md`.

### RenderPlanner — RF-30

- `input_hash` es la **única** vía legítima para generar el hash de caché de
  TTS. No reimplementar inline.
- Los campos que entran al hash son exactamente:
  `text | voice_id | model | tag_signature | style_prompt?`. Cualquier
  cambio de receta requiere ADR.
- Ver `docs/decisions/0008-input-hash-determinista.md`.

### DeepSeek y Gemini

- `SYSTEM_PROMPT` (DeepSeek estructuración) es contractual con RF-13.
  Cambios al prompt deben preservar el comportamiento descrito: no
  resumir, no inventar, no suavizar lenguaje, no perder groserías ni
  tono.
- `TTS_OPTIMIZATION_PROMPT` (DeepSeek optimización RF-22) es **más**
  restrictivo: no puede tocar text/speaker/order/type. El servicio
  filtra los IDs devueltos contra los que mandó (defensa en
  profundidad).
- Cada llamada HTTP tiene timeout explícito (`REQUEST_TIMEOUT_SECS`).
- Gemini usa header `x-goog-api-key` en lugar de query param para evitar
  exposición en logs de proxies.
- Nunca loggear el cuerpo de la petición si contiene fragmentos del guion
  del usuario; loggear solo metadatos (tamaño, modelo, status HTTP).

### AudioMixer — ADR-0012

- **Sample rate objetivo: 24 kHz mono.** Constante pública
  `audio_mixer::TARGET_SAMPLE_RATE`. Si cambia, hay que recalcular todos
  los `input_hash` cacheados.
- `decode_to_mono(path)` es la única vía para decodificar audio. Cubre
  WAV/MP3/OGG/FLAC vía symphonia.
- `render_mix` espera buffers ya en sample rate objetivo (los normaliza
  `decode_to_mono`). Volumen/mute/solo se aplican por pista; cada clip
  aporta volumen/fade individuales (RF-37).
- Export MP3 está intencionalmente como `NotImplemented`; ver
  `architecture.md §7` (deuda post-MVP).

### TtsService

- `synthesize_dialogue(force=false)` es **el** punto de entrada para
  obtener audio de un nodo. Respeta cache por `input_hash`.
- Cuando genera un audio nuevo, marca como `outdated` cualquier
  `generated_audio` previo del mismo nodo. Esta función debe permanecer
  consistente con `tts_optimization_service` y con la regla RF-38
  (editar tags invalida TTS; editar delays no).
- El archivo en disco tiene el mismo `id` que el row en BD para evitar
  colisiones y simplificar un GC futuro.

### ProjectIoService — ADR-0013

- `ProjectSnapshot::schema_version` empieza en `1`. Cualquier cambio que
  rompa el formato (nuevo campo no nullable, renombrado de columna) tiene
  que incrementar la versión **y** añadir una rama de migración en
  `import_from_file`.
- El snapshot **nunca** incluye binarios; solo metadatos y `file_path`.
- Al importar, los UUIDs se regeneran. Las FKs internas se preservan
  vía los `HashMap<old_id, new_id>` por entidad.

## Tests

Pendientes. Cuando se añadan, vivirán en módulos `#[cfg(test)]` dentro de
cada servicio. Usar `sea-orm`'s `DatabaseConnection` apuntando a
`sqlite::memory:` para tests de servicios que tocan BD.
