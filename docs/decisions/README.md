# Architecture Decision Records (ADRs)

Cada decisión estructural no trivial vive aquí como un archivo
`NNNN-kebab-case-titulo.md`. Las decisiones triviales (renombres, partir un
componente, ajustar un comentario) **no** van a ADR: se documentan en el PR.

## Cuándo escribir un ADR

Escribe un ADR si tu cambio:

- Cambia el stack (librería core, framework, lenguaje).
- Cambia el contrato entre capas (forma de un servicio, firma común de
  comandos, formato del modelo de datos).
- Cambia el modelo de seguridad (almacenamiento de credenciales, CSP, permisos
  Tauri, ABAC en BD).
- Cambia un invariante del dominio (p. ej. relaja la regla "editar texto
  invalida el audio").
- Introduce una nueva capa o un nuevo módulo top-level (`src-tauri/src/foo/`,
  `app/foo/`).
- Sustituye un componente core (cambiar SeaORM por sqlx directo, Nuxt UI por
  otra librería UI, etc.).

## Plantilla

Copia [`_template.md`](_template.md) renombrándolo a `NNNN-titulo.md` con el
siguiente número correlativo.

## Estados

- **Aceptada** — está vigente.
- **Reemplazada por NNNN** — superada por otro ADR. Incluir el link inverso.
- **Rechazada** — se evaluó y se decidió no hacerla. Útil como histórico para
  evitar reproponerla.
- **Deprecada** — ya no aplica pero no fue reemplazada (p. ej. eliminamos la
  funcionalidad asociada).

## Índice

| # | Título | Estado |
| - | ------ | ------ |
| [0001](0001-stack-tauri-nuxt-seaorm.md) | Stack Tauri 2 + Nuxt 4 + SeaORM 1 | Aceptada |
| [0002](0002-keyring-para-api-keys.md)   | Keyring del SO para API keys | Aceptada |
| [0003](0003-bun-como-gestor-js.md)      | bun como gestor y runner de JS | Aceptada |
| [0004](0004-mezcla-audio-en-rust.md)    | Mezcla de audio en Rust (no Web Audio) | Aceptada |
| [0005](0005-sqlite-por-proyecto.md)     | SQLite separada por proyecto | Aceptada |
| [0006](0006-single-project-runtime.md)  | Single-project en runtime | Aceptada |
| [0007](0007-commands-thin-services-stateless.md) | Commands delgados, servicios stateless | Aceptada |
| [0008](0008-input-hash-determinista.md) | Caché TTS por input_hash determinístico | Aceptada |
| [0009](0009-error-unificado-serializable.md) | AppError unificado y serializable | Aceptada |
| [0010](0010-componentes-sin-path-prefix.md) | Auto-import de componentes sin path prefix | Aceptada |
