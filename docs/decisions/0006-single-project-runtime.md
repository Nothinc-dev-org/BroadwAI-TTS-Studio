# 0006 — Single-project en runtime

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

Aunque el usuario puede tener N proyectos en disco, la aplicación necesita
decidir cuántos puede tener "abiertos" simultáneamente. Esto afecta diseño de
`AppState`, pooling de conexiones SeaORM y UX.

## Decisión

La aplicación abre **un único proyecto a la vez** en runtime. `AppState`
mantiene `RwLock<Option<Arc<OpenProject>>>`; abrir otro proyecto cierra el
anterior implícitamente.

Implicaciones:

- `AppState::current()` falla con `AppError::invalid("no hay un proyecto
  abierto")` si no se ha llamado `open_project` o `create_project` antes.
- Todos los comandos que necesitan DB resuelven la conexión vía
  `state.current().await?` al inicio.
- Cambiar de proyecto en UI requiere navegar a la home y abrirlo de nuevo;
  no hay "tabs de proyecto" abiertos en paralelo.

## Consecuencias

### Positivas

- Modelo mental simple: en cualquier momento hay un proyecto activo o
  ninguno.
- Pool de conexiones SQLite mínimo (1–5 conexiones por proyecto activo).
- Evita cuestiones de concurrencia entre proyectos.

### Negativas / costos asumidos

- No se puede hacer drag-and-drop de un personaje o asset de un proyecto a
  otro. Para MVP no es requerido.
- Export/import entre proyectos requiere serializar a archivo (RF-39, RF-40)
  y deserializar en el otro, en dos pasos.

### Riesgos abiertos

- Si en el futuro queremos "abrir comparativamente" dos versiones del mismo
  proyecto, este modelo se queda corto. Reevaluar entonces.

## Alternativas consideradas

### A. Multi-proyecto con `HashMap<ProjectId, OpenProject>`

Útil para drag-and-drop entre proyectos, pero introduce complejidad en UX y
en el pool de conexiones. Sin requisito que lo demande.

## Referencias

- `Requerimiento.md` no especifica; queda a discreción técnica.
- `src-tauri/src/state.rs`.
