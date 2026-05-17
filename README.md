# BroadwAI TTS Studio

Editor de escenas TTS multi-personaje, local-first, basado en Tauri 2 + Nuxt 4 + Nuxt UI + SeaORM + SQLite, integrado con DeepSeek V4 Flash (estructuración) y Gemini 3.1 Flash TTS Preview (síntesis de voz).

Estado actual: **scaffold MVP 1**. La base compila, las migraciones cubren el modelo de datos completo del documento de requerimiento y los comandos/servicios están definidos como stubs listos para implementar.

## Stack

| Capa            | Tecnología                          |
| --------------- | ----------------------------------- |
| Desktop shell   | Tauri 2                             |
| Backend         | Rust 1.93+                          |
| ORM             | SeaORM 1                            |
| Base de datos   | SQLite por proyecto                 |
| Frontend        | Nuxt 4 + Nuxt UI 3                  |
| Empaquetado JS  | bun                                 |
| Credenciales    | keyring (libsecret / Keychain / DPAPI) |
| Audio           | symphonia + rubato + hound          |
| LLM             | DeepSeek V4 Flash (HTTP)            |
| TTS             | Gemini 3.1 Flash TTS Preview (HTTP) |

## Prerrequisitos

- Rust 1.93+ con `cargo`
- bun 1.3+
- En Linux: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `libsoup-3.0-dev`, `libsecret-1-dev` y un agente Secret Service activo (gnome-keyring/KWallet)

## Desarrollo

```bash
bun install
bun run tauri dev
```

## Estructura

```
app/                    # Nuxt 4 (pages, components, composables)
src-tauri/              # Tauri 2 + Rust
  src/
    commands/           # #[tauri::command] handlers
    services/           # Lógica de dominio
    entities/           # SeaORM entities
    migrations/         # SeaORM migrations
    db.rs               # Conexión y bootstrap
    state.rs            # AppState compartido
    error.rs            # Tipo de error unificado
    paths.rs            # Rutas estándar del proyecto
```

## Seguridad de credenciales

Las API keys (DeepSeek, Gemini) **nunca** se persisten en SQLite ni en `localStorage`. Se almacenan en el keyring del sistema operativo a través del `CredentialService`. El backend Rust es el único que las lee y las inyecta en las llamadas HTTP a los proveedores.

En Linux se usa Secret Service/libsecret para que las claves persistan tras reiniciar. El backend `keyutils` de `keyring` no se usa porque solo actúa como cache en memoria del kernel.


## Roadmap

Ver `Requerimiento.md` para el detalle completo de RFs, RNFs, MVPs y criterios de aceptación.

