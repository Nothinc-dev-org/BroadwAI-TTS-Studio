# AGENTS.md — `src-tauri/src/entities/`

## Propósito

Modelos SeaORM derivados de las tablas definidas en
`src-tauri/src/migrations/`. Una entidad por tabla. Son **tontas**: no
contienen lógica de negocio, solo serialización, derivaciones de SeaORM y
relaciones.

## Convención

```rust
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "xxx")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    …
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation { … }

impl ActiveModelBehavior for ActiveModel {}
```

- **`id`** es `String` (UUID v4 generado en `services::new_id`). No usamos
  enteros autoincrement para mantener IDs estables al copiar/mover proyectos.
- **Fechas** son `String` (ISO-8601 vía `services::now`). Trade-off de
  legibilidad y simplicidad vs. `chrono::DateTime`. Si en algún punto pesa,
  migrar con cambio explícito en el schema y en `app/types/domain.ts`.
- **Columnas reservadas** (`type`, `loop`) se mapean con `column_name`:
  - `dialogue_node::kind` ← `type`
  - `audio_asset::kind` ← `type`
  - `timeline_track::kind` ← `type`
  - `timeline_event::looping` ← `loop`
  - `render_job::kind` ← `type`
- **`is_enabled`, `muted`, `solo`, `looping`** son `i32` (SQLite no tiene
  booleano nativo). 0 = false, 1 = true. Convertir en la capa de servicio si
  se necesita un `bool` para lógica.

## Relaciones

Las relaciones se declaran tipadas con `DeriveRelation` y `Related<T>`. Esto
permite usar `find_with_related()` y joins seguros. Verifica que **siempre**
coincidan con las foreign keys definidas en la migración correspondiente: si
la migración tiene `ON DELETE CASCADE`, la relación debe declarar
`on_delete = "Cascade"`. La discordancia no es un error de compilación; es
un bug silencioso.

## Sincronización con frontend

El frontend tiene un espejo en `app/types/domain.ts`. **Cualquier cambio de
campo aquí debe replicarse allí**. Si no, las llamadas a `invoke` devuelven
objetos con campos no tipados y el compilador TS no lo detecta hasta
runtime.

Lista de tipos espejo:

| Entidad Rust         | Tipo TS              |
| -------------------- | -------------------- |
| `project::Model`     | `Project`            |
| `scene::Model`       | `Scene`              |
| `character::Model`   | `Character`          |
| `character_alias::Model` | `CharacterAlias` |
| `dialogue_node::Model` | `DialogueNode`     |
| `dialogue_tts_tag::Model` | `DialogueTtsTag` |
| `audio_asset::Model` | `AudioAsset`         |
| `generated_audio::Model` | `GeneratedAudio` |
| `timeline_track::Model` | `TimelineTrack`   |
| `timeline_event::Model` | `TimelineEvent`   |
| `render_job::Model`  | `RenderJob`          |
| `app_setting::Model` | *(interno, no espejo)* |
| `raw_import::Model`  | *(definido inline en `useImport.ts`)* |

## Reglas de oro

- **No añadir métodos de negocio**. Si tienes la tentación, ese método
  pertenece a un servicio.
- **No añadir campos calculados**. Calcula en la capa de servicio.
- **No omitir campos** de la migración. La entidad debe reflejar la tabla
  completa.
