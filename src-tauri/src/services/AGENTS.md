# AGENTS.md — `src-tauri/src/services/`

## Propósito

Capa de **dominio**. Contiene la lógica de negocio del MVP 1 y los stubs de
los servicios que necesitan implementación en sprints siguientes. Mapeo 1:1
con la sección 13 de `Requerimiento.md`.

## Convención

Servicios como **funciones libres stateless** que reciben los recursos como
parámetros:

```rust
pub async fn create(db: &DatabaseConnection, input: CreateXInput) -> AppResult<X> { … }
```

No structs con `&self` salvo justificación específica (`CredentialService` es
una excepción porque encapsula constantes del crate `keyring`).

Ver `docs/decisions/0007-commands-thin-services-stateless.md`.

## Mapa de servicios

| Archivo                   | Responsabilidad                                        | Estado |
| ------------------------- | ------------------------------------------------------ | ------ |
| `project_service.rs`      | Crear/listar/abrir proyectos (RF-01, 02)               | 🟡 parcial |
| `scene_service.rs`        | CRUD de escenas (RF-04, 05)                            | 🟡 parcial |
| `character_service.rs`    | Personajes y alias (RF-06, 07, 08, 23)                 | 🟡 parcial |
| `dialogue_service.rs`     | Bloques narrativos y tags (RF-17–21)                   | 🟡 list/queries |
| `import_service.rs`       | Texto pegado y archivo `.txt/.md` (RF-09, 10)          | ✅ raw_imports |
| `deepseek_service.rs`     | HTTP a DeepSeek + prompt (RF-11–15, 22)                | 🟦 stub |
| `gemini_tts_service.rs`   | HTTP a Gemini TTS (RF-23–31)                           | 🟦 stub |
| `render_planner.rs`       | Planificación de jobs + `input_hash` (RF-29, 30)       | ✅ hash listo |
| `audio_mixer.rs`          | Mezcla con symphonia/rubato/hound (RF-26, 36, 37)      | 🟦 stub |
| `asset_service.rs`        | Biblioteca de audio (RF-32, 33)                        | 🟡 list |
| `credential_service.rs`   | API keys vía keyring del SO (RF-03, RNF-01)            | ✅ completo |

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

- `SYSTEM_PROMPT` (DeepSeek) es contractual con RF-13. Cambios al prompt
  deben preservar el comportamiento descrito: no resumir, no inventar, no
  suavizar lenguaje, no perder groserías ni tono.
- Cada llamada HTTP debe tener timeout explícito (pendiente de añadir a los
  stubs cuando se implementen).
- Nunca loggear el cuerpo de la petición si contiene fragmentos del guion
  del usuario; loggear solo metadatos (tamaño, modelo, latencia).

## Tests

Pendientes. Cuando se añadan, vivirán en módulos `#[cfg(test)]` dentro de
cada servicio. Usar `sea-orm`'s `DatabaseConnection` apuntando a
`sqlite::memory:` para tests de servicios que tocan BD.
