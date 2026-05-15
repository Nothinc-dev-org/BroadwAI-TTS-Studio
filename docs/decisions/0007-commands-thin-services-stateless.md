# 0007 — Commands delgados, servicios stateless

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

Tauri ofrece `#[tauri::command]` como mecanismo principal para exponer
funciones al frontend. Hay riesgo de meter lógica de negocio dentro de los
comandos, lo que dificulta tests, reutilización y razonamiento sobre la
arquitectura.

`Requerimiento.md` §13 enumera servicios de dominio con responsabilidades
claras (ProjectService, SceneService, etc.). Aprovechamos esa lista para
imponer una separación de capas.

## Decisión

- **Comandos** (`src-tauri/src/commands/*.rs`) son **transport**. Solo:
  1. Reciben argumentos desde el frontend.
  2. Resuelven `AppState` y, si necesitan DB, obtienen `&DatabaseConnection`
     vía `state.current().await?`.
  3. Llaman a una función de un servicio.
  4. Devuelven `AppResult<T>`.

- **Servicios** (`src-tauri/src/services/*.rs`) son **funciones libres
  stateless** que reciben los recursos (DB, CredentialService, paths) como
  parámetros. No guardan estado mutable propio.

- **Entidades** (`src-tauri/src/entities/*.rs`) son modelos SeaORM puros.
  No tienen métodos de negocio.

Ejemplo:

```rust
// commands/scene.rs
#[tauri::command]
pub async fn create_scene(state: State<'_, AppState>, …) -> AppResult<scene::Model> {
    let current = state.current().await?;
    scene_service::create(&current.db, scene_service::CreateSceneInput { … }).await
}

// services/scene_service.rs
pub async fn create(db: &DatabaseConnection, input: CreateSceneInput) -> AppResult<scene::Model> {
    // lógica
}
```

## Consecuencias

### Positivas

- Tests unitarios de servicios sin tocar Tauri: basta con una `DatabaseConnection`
  in-memory.
- Reutilización entre comandos: un servicio puede ser invocado desde varios
  comandos sin duplicar lógica.
- Diagnóstico más simple: la pila de llamadas es `command → service → entity`.

### Negativas / costos asumidos

- Más boilerplate en los comandos (5-10 líneas de "plumbing").
- Tentación de saltarse el servicio para queries triviales; aceptamos esa
  tentación con disciplina humana, no con código.

### Riesgos abiertos

- Si un servicio crece y empieza a necesitar estado (p. ej. una cola de jobs
  en memoria), revisitar: probablemente toque vivir en `AppState` como otra
  estructura (similar a `CredentialService`).

## Alternativas consideradas

### A. Servicios como structs con `&self`

Más OOP pero introduce ciclos de vida y `Arc<Mutex<…>>` para nada en la
mayoría de casos.

### B. Lógica directa en los comandos

Más rápido en el corto plazo, costo creciente y difícil de testear.

## Referencias

- `Requerimiento.md` §5.2, §12, §13.
- `src-tauri/src/commands/*.rs`, `src-tauri/src/services/*.rs`.
