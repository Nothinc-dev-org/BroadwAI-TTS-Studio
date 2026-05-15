# 0009 — `AppError` unificado y serializable

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

`#[tauri::command]` requiere que el tipo `Result::Err` implemente `Serialize`
para poder ser devuelto al frontend como JSON. Existen tres opciones típicas:

1. `Result<T, String>` — fácil, pero pierde información de causa y mezcla
   mensajes técnicos con mensajes para el usuario.
2. `Result<T, anyhow::Error>` — no es `Serialize` por defecto.
3. Tipo de error propio con `Serialize` custom.

Además, el comando puede surgir de errores muy distintos: DB, IO, JSON, HTTP,
keyring, validación. Tratarlos por separado en cada comando es ruidoso.

## Decisión

Definir `AppError` (en `src-tauri/src/error.rs`) como `enum` con `thiserror`:

```rust
pub enum AppError {
    NotImplemented(&'static str),
    NotFound(String),
    InvalidInput(String),
    Database(#[from] sea_orm::DbErr),
    Io(#[from] std::io::Error),
    Json(#[from] serde_json::Error),
    Http(#[from] reqwest::Error),
    Provider(String),
    Credential,                  // ← sin payload (RNF-01)
    Config(String),
    Internal(String),
}
```

`Serialize` se implementa manualmente serializando solo `to_string()` para que
el frontend reciba `string` sin estructura interna sensible.

`AppResult<T> = Result<T, AppError>` se usa universalmente. Todas las capas
(comandos, servicios, entidades) usan este tipo.

El variante `Credential` no lleva payload para garantizar que detalles internos
del keyring no escapen al frontend (RNF-01).

## Consecuencias

### Positivas

- Comandos limpios: `?` propaga errores de DB, IO, HTTP sin glue manual.
- Frontend siempre recibe un `string` predecible.
- `AppError::Credential` blinda RNF-01 a nivel de tipo.

### Negativas / costos asumidos

- Si en el futuro el frontend quiere distinguir tipos de error (p. ej. para
  mostrar UI distinta en `NotFound` vs `InvalidInput`), tendremos que
  serializar como `{kind, message}`. Migración manejable cuando se necesite.
- Errores con payload genéricos (`Internal(String)`) tientan al desarrollador
  a meter mensajes formateados con datos sensibles. Disciplina humana.

### Riesgos abiertos

- Logs internos (`tracing::warn!` en `from_keyring`) deben mantenerse fuera de
  ventas de exporte/telemetría futura. Documentar cuando se añada telemetría.

## Alternativas consideradas

### A. `Result<T, String>` directo

Mezcla mensajes técnicos con mensajes de usuario, hace `?` imposible sin
glue manual.

### B. Tipo `kind` discriminado en el JSON

Más rico pero acopla el frontend a la enumeración de errores. Posible
evolución cuando lo necesitemos.

## Referencias

- `Requerimiento.md` RNF-01, RNF-04.
- `src-tauri/src/error.rs`.
