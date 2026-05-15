# AGENTS.md — `src-tauri/src/migrations/`

## Propósito

**Fuente de verdad del modelo de datos**. Lo que esté aquí define el schema
SQLite. Las entidades de `entities/` y los tipos de `app/types/domain.ts`
son consecuencias, no fuentes.

## Convención de naming

```
m<YYYYMMDD>_<NNNNNN>_<descripcion>.rs
```

- `<YYYYMMDD>`: fecha de creación (no se cambia al editar).
- `<NNNNNN>`: número secuencial dentro del día (000001, 000002, …).
- `<descripcion>`: snake_case que describe qué tablas/cambios trae.

Ejemplo: `m20260515_000004_create_dialogue_nodes.rs`.

## Estructura de un archivo

```rust
use sea_orm_migration::prelude::*;
use super::m<previous>::TablaQueReferencio;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> { … }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> { … }
}

#[derive(DeriveIden)]
pub enum TablaNueva { Table, Id, Campo1, Campo2, … }
```

- El `enum DeriveIden` se exporta para que migraciones posteriores puedan
  referenciar columnas (`super::m<X>::TablaNueva`). Es la única manera segura
  de evitar typos en nombres de tablas/columnas.
- `up()` y `down()` deben ser **simétricos**: `down` deshace exactamente lo
  que `up` hace, en orden inverso (incluyendo índices).

## Reglas críticas

1. **Una migración mergeada no se edita**. Si necesitas cambiar el schema,
   añade una migración nueva (`alter_table`, `create_index`, etc.).
2. **Registrar en `mod.rs`**. Cada nueva migración va al `vec![…]` de
   `Migrator::migrations` en **orden cronológico**. Saltarse este paso hace
   que la migración no se ejecute.
3. **Foreign keys con acción explícita**. Decidir y declarar `ON DELETE`:
   - `Cascade`: borrar el padre borra el hijo (default para casi todo).
   - `SetNull`: borrar el padre deja huérfano al hijo. Útil para
     `timeline_events.dialogue_node_id`.
   - `Restrict`: borrar el padre falla si hay hijos. Útil para
     `dialogue_nodes.character_id` (no perder un bloque por borrar un
     personaje sin darse cuenta).
4. **Índices explícitos** para las columnas usadas en `WHERE`/`ORDER BY`
   frecuentes (`*_project_id`, `dialogue_nodes(scene_id, order_index)`,
   `generated_audio(input_hash)`, `render_jobs(status)`).
5. **Tipos consistentes con SQLite**: `text`, `integer`, `double` (no usar
   `varchar`, `boolean`, `decimal`, etc., aunque SeaORM los acepte;
   en SQLite todo se traduce a TEXT/INTEGER/REAL y la consistencia ayuda).
6. **Booleanos como `integer` con default `0`/`1`**. No usar `boolean()`.

## Tablas actuales

| #  | Migración                                | Tablas creadas                        |
| -- | ---------------------------------------- | ------------------------------------- |
| 01 | `…000001_create_projects.rs`             | `projects`                            |
| 02 | `…000002_create_scenes.rs`               | `scenes`                              |
| 03 | `…000003_create_characters.rs`           | `characters`, `character_aliases`     |
| 04 | `…000004_create_dialogue_nodes.rs`       | `raw_imports`, `dialogue_nodes`, `dialogue_tts_tags` |
| 05 | `…000005_create_audio_tables.rs`         | `audio_assets`, `generated_audio`     |
| 06 | `…000006_create_timeline_tables.rs`      | `timeline_tracks`, `timeline_events`  |
| 07 | `…000007_create_render_jobs.rs`          | `render_jobs`                         |
| 08 | `…000008_create_app_settings.rs`         | `app_settings`                        |

## Workflow de cambio de schema

1. Diseñar la migración nueva. Si toca campos críticos para `input_hash`
   (text, voice_id, model, tags), considerar el efecto sobre el caché
   existente.
2. Crear `m<fecha>_<NNNNNN>_<descripcion>.rs`.
3. Registrar en `mod.rs::Migrator::migrations`.
4. Actualizar la entidad correspondiente en `entities/`.
5. Actualizar `app/types/domain.ts`.
6. `cargo check` y, si se altera caché TTS, ADR nuevo.
