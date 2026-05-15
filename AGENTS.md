# AGENTS.md — BroadwAI TTS Studio (raíz)

> Contrato global para cualquier agente de IA que opere sobre este repositorio.
> Léelo antes de cualquier operación no trivial. Cuando entres a un módulo,
> léete también su `AGENTS.md` local: el global aporta el qué/por qué, el local
> aporta el cómo en ese rincón.

## Qué es esto

BroadwAI TTS Studio es una **aplicación de escritorio local-first** para crear,
editar, generar y mezclar escenas TTS multi-personaje. El usuario importa un
guion (texto pegado o archivo `.txt`/`.md`), DeepSeek V4 Flash lo convierte en
bloques estructurados y Gemini 3.1 Flash TTS Preview los sintetiza. Sobre esos
audios la app construye una mezcla con efectos, música y ambiente.

Documento canónico de requerimientos: `Requerimiento.md` (no editar sin acordarlo).

Documento canónico de arquitectura: [`docs/architecture.md`](docs/architecture.md).

Decisiones arquitectónicas: [`docs/decisions/`](docs/decisions/).

## Estructura del repositorio

```
.
├── AGENTS.md                ← este archivo
├── Requerimiento.md         ← fuente original de requerimientos
├── README.md                ← README de usuario
├── docs/
│   ├── architecture.md      ← arquitectura canónica
│   └── decisions/           ← ADRs (Architecture Decision Records)
├── .ai/                     ← contexto adicional para agentes
├── package.json             ← Nuxt 4 + bun
├── nuxt.config.ts
├── app.config.ts
├── tsconfig.json
├── app/                     ← Frontend Nuxt 4
│   ├── app.vue
│   ├── layouts/
│   ├── pages/               ← rutas (con AGENTS.md propio)
│   ├── components/          ← UI por dominio (con AGENTS.md propio)
│   ├── composables/         ← bindings a comandos Tauri (con AGENTS.md propio)
│   └── types/domain.ts      ← espejo de las entidades Rust
└── src-tauri/               ← Backend Rust + Tauri 2
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── capabilities/
    ├── icons/               ← placeholder, reemplazar antes de bundle
    └── src/
        ├── main.rs · lib.rs ← entrypoint y registro de comandos
        ├── error.rs         ← AppError unificado
        ├── state.rs         ← AppState (proyecto abierto)
        ├── db.rs            ← bootstrap SeaORM
        ├── paths.rs         ← layout de carpetas por proyecto
        ├── commands/        ← #[tauri::command] handlers (AGENTS.md propio)
        ├── services/        ← lógica de dominio (AGENTS.md propio)
        ├── entities/        ← SeaORM entities (AGENTS.md propio)
        └── migrations/      ← SeaORM migrations (AGENTS.md propio)
```

## Stack inamovible

Cambios al stack requieren ADR explícito en `docs/decisions/`.

- **Desktop shell:** Tauri 2.x
- **Backend:** Rust (edition 2021, MSRV 1.77)
- **ORM:** SeaORM 1.x sobre SQLite (`sqlx-sqlite` + `runtime-tokio-rustls`)
- **Frontend:** Nuxt 4 (SSR off, modo SPA) + Nuxt UI 3 + Vue 3.5
- **Gestor de paquetes JS:** **bun**. No usar `npm` ni `pnpm`.
- **Credenciales:** crate `keyring` 3 (libsecret en Linux, Keychain en macOS, Credential Manager en Windows)
- **Audio:** `symphonia` (decode) + `rubato` (resample) + `hound` (encode WAV)
- **LLM:** DeepSeek V4 Flash vía HTTP (`reqwest`)
- **TTS:** Gemini 3.1 Flash TTS Preview vía HTTP (`reqwest`)

## Reglas de contribución para agentes

### Separación

- **Código** → `src-tauri/` (Rust) y `app/` (TS/Vue). Nunca mezclar lógica de
  negocio con presentación.
- **Documentación** → `docs/`. Nunca en `src-tauri/` ni en `app/` (excepto
  `AGENTS.md` por módulo, que **es** documentación pero vive cerca del código
  para dar contexto).
- **Configuración de IA** → `.ai/`. Prompts del sistema, datasets de prueba,
  scripts de validación de prompts. No mezclar con `docs/`.

### Estructura

1. **Antes de cualquier operación**, leer este `AGENTS.md` global y el local del
   directorio donde se va a trabajar.
2. **Al crear un directorio nuevo** dentro de `src-tauri/src/` o `app/`, generar
   un `AGENTS.md` propio describiendo el propósito del módulo, sus dependencias
   internas y los invariantes que respeta.
3. **Una decisión estructural** (cambio de stack, de patrón, de capa o de
   contrato entre capas) **debe** quedar registrada como ADR en
   `docs/decisions/` con la plantilla del proyecto (ver `docs/decisions/README.md`).
4. **Una decisión local trivial** (renombrar una función, partir un componente)
   **no** va a ADR; va al PR.

### Seguridad (RNF-01)

Las API keys del usuario nunca:

- se persisten en SQLite
- se imprimen en logs (`tracing` o `println!`)
- se serializan al frontend (excepto en el momento de `set_api_key`, que va del
  frontend al backend, **nunca en sentido contrario**)
- aparecen en errores que viajen al frontend (usar `AppError::Credential`, no
  formatear el error de `keyring`)

Esto aplica a cualquier agente que toque `credential_service.rs`,
`deepseek_service.rs`, `gemini_tts_service.rs` o los comandos de
`settings.rs`. Ver `docs/decisions/0002-keyring-para-api-keys.md`.

### Modelo de datos

La fuente de verdad del modelo de datos es **las migraciones SeaORM** en
`src-tauri/src/migrations/`. Cualquier cambio de schema debe:

1. Añadir una nueva migración (no editar las existentes una vez mergeadas).
2. Actualizar la entidad correspondiente en `src-tauri/src/entities/`.
3. Actualizar el tipo TypeScript espejo en `app/types/domain.ts`.

Romper esta cadena rompe el contrato frontend ↔ backend silenciosamente.

### Comandos Tauri

Cada comando registrado en `lib.rs` debe:

1. Recibir `State<'_, AppState>` cuando necesite acceso a la DB.
2. Devolver `AppResult<T>` (nunca `Result<T, String>` ni paneles).
3. Tener su contraparte en un composable de `app/composables/`.
4. Estar listado en el `invoke_handler!` de `lib.rs`.

Olvidar el paso 4 = el frontend recibe `Error: command not found`.

## Estado actual

**Sprint 1 entregado:** scaffold MVP 1 (este commit/sesión).

- Estructura completa de archivos compila (`cargo check` ✅, `bun install` ✅).
- 13 tablas creadas vía 8 migraciones.
- ~50 comandos Tauri registrados; ~15 funcionales (proyecto, escena,
  personaje básico, importación raw, credenciales).
- ~35 comandos devuelven `AppError::NotImplemented` con firma final.

**Próximo:** RF-11 a RF-16 (procesamiento real con DeepSeek y creación de
escena desde importación).

## Comandos útiles

```bash
# Frontend
bun install
bun run dev            # Nuxt en :1420 sin Tauri (modo navegador)

# Stack completo (recomendado durante desarrollo)
bun run tauri:dev

# Validación rápida
cd src-tauri && cargo check
bun run typecheck
```

## Notas operativas

- **Puerto del dev server:** 1420 (estándar Tauri). El cambio desde el típico
  3000 evita colisiones con otros servicios y simplifica capabilities.
- **`ssr: true` en `nuxt.config.ts`:** workaround temporal para una regresión
  en `@nuxt/vite-builder@4.4.5` (`resolveServerEntry` falla con `ssr: false`).
  Para producción seguimos generando estáticos vía `bun run generate`
  (`tauri.conf.json::beforeBuildCommand`). Cuando se publique el fix upstream,
  puede revertirse a `ssr: false` para evitar el overhead de prerender en dev.
