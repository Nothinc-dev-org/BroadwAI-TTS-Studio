# 0013 — Export/import de proyecto: snapshot JSON con remapeo de IDs

- **Fecha:** 2026-05-15
- **Estado:** Aceptada
- **Decisores:** Backend Persistencia, Frontend

## Contexto

RF-39 / RF-40 piden export e import de proyectos completos. Hay dos
preguntas estructurales:

1. **Formato:** ¿zip con SQLite + binarios? ¿JSON plano? ¿zip de JSON +
   carpetas?
2. **IDs:** ¿preservar los UUID originales o regenerarlos al importar?

Restricciones:

- La app es single-project en runtime (ADR-0006). Importar a la misma
  máquina puede coexistir con el proyecto original abierto en otra
  sesión.
- Los binarios (audios generados, assets) viven bajo `<project-root>/`,
  son propiedad del usuario y pueden pesar varios cientos de MB.
- El modelo de datos tiene relaciones cruzadas (`dialogue_node.previous_id`,
  `timeline_event.audio_asset_id`, `generated_audio.dialogue_node_id`,
  etc.). Las FKs internas tienen que sobrevivir al import.

## Decisión

1. **Formato:** snapshot **JSON único** (`ProjectSnapshot`) con
   `schema_version: u32`, `exported_at` y todas las 12 colecciones
   relevantes serializadas via SeaORM.
2. **IDs:** la importación **regenera UUIDs nuevos en cascada** y remapea
   las FKs internas mediante `HashMap<String, String>` por entidad.
3. **Binarios:** se preservan los `file_path` tal cual están en el
   snapshot. El usuario es responsable de mover los archivos si trasladó
   el proyecto a otra máquina.
4. **Idempotencia:** importar dos veces el mismo JSON crea dos proyectos
   distintos. No hay merge.

```rust
pub struct ProjectSnapshot {
    pub schema_version: u32,
    pub exported_at: String,
    pub project: project::Model,
    pub scenes: Vec<scene::Model>,
    // … 10 colecciones más
    pub app_settings: Vec<app_setting::Model>,
}
```

## Consecuencias

### Positivas

- Diff-friendly y git-friendly: el JSON es legible.
- Sin colisiones de UUID al importar a un workspace ya poblado.
- Las FKs (`previous_id`, `next_id`, `dialogue_node_id`, `audio_asset_id`,
  `generated_audio_id`) se preservan correctamente vía los mapas.
- `schema_version` permite migrar formatos futuros sin perder snapshots
  antiguos.

### Negativas / costos asumidos

- Los `file_path` quedan apuntando a rutas absolutas viejas si el
  proyecto se mueve. **El import deja la BD consistente pero los audios
  pueden dejar de existir.** El usuario tiene que reimportar assets o
  regenerar TTS (la regeneración es transparente por `input_hash`).
- El JSON puede crecer mucho con escenas largas (no llega a ser problema
  hasta varias decenas de MB), pero los binarios **nunca** entran al JSON.
- No hay round-trip perfecto: exportar → importar produce UUIDs nuevos.
  Cualquier referencia externa (logs, exports previos por id) se rompe.

### Riesgos abiertos

- **Empaquetado con binarios** (zip + JSON + carpetas) queda como
  follow-up para los usuarios que quieran un único artefacto portable.
  Diseño futuro: ADR separado.
- Importación parcial (solo una escena) no se contempla; vendría con un
  servicio `scene_io_service` distinto.

## Alternativas consideradas

### A. Copiar la SQLite

Tomar `database/project.sqlite` y meterla en el zip. Descartada:
problema de portabilidad entre versiones de SeaORM, y los `file_path`
absolutos siguen rotos. El JSON aporta legibilidad y desacopla del
binary format de SQLite.

### B. Preservar UUIDs originales

Descartada: si el usuario importa a un workspace donde el proyecto
original sigue presente, colisionan por PK. Forzar al usuario a borrar
antes de importar es UX hostil.

### C. Snapshot zip con binarios incluidos

Hacer el snapshot un `.broadwai` (zip) con `project.json` + `audio/` +
`assets/`. Más completo pero implica duplicar GB de datos del usuario en
cada export. Para MVP, los archivos quedan referenciados; portabilidad
total es follow-up.

## Referencias

- RF-39, RF-40 en `Requerimiento.md`.
- ADR-0005 (SQLite por proyecto).
- ADR-0006 (single-project en runtime).
- `services/project_io_service.rs` — implementación.
