# 0001 — Stack: Tauri 2 + Nuxt 4 + SeaORM 1

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

`Requerimiento.md` define el stack en la sección 1.3 como una restricción dura:
desktop shell Tauri.rs, backend Rust, ORM SeaORM, BD SQLite, frontend Nuxt 4,
UI Nuxt UI. No hay backend remoto propio (local-first, sección 4.1).

Hay que materializar ese stack en versiones concretas y un esqueleto compilable.

## Decisión

Adoptar:

- **Tauri 2.x** (no 1.x): API estable de comandos, sistema de plugins maduro
  (`tauri-plugin-dialog`, `-fs`, `-shell`) y futuro de la plataforma.
- **Nuxt 4.x** con `app/` como `srcDir` por defecto y `ssr: false` (SPA).
- **Nuxt UI 3.x** con Tailwind 4.
- **SeaORM 1.x** con `sqlx-sqlite` y `runtime-tokio-rustls`.
- **sea-orm-migration 1.x** para schema versionado.
- **Rust edition 2021**, MSRV 1.77 (suficiente para Tauri 2 y SeaORM 1).

## Consecuencias

### Positivas

- Cero divergencia con `Requerimiento.md`.
- Tauri 2 + Nuxt 4 son las versiones de soporte largo en su línea, evitamos
  rework de migración a medio plazo.
- SeaORM ofrece migraciones tipadas y entidades derivadas, lo que reduce
  drift entre schema y código.
- `runtime-tokio-rustls` evita acoplarnos a OpenSSL en el sistema (relevante
  para builds reproducibles cross-OS).

### Negativas / costos asumidos

- Tauri 2 todavía recibe breaking changes menores; cualquier upgrade
  requiere revisar release notes.
- Nuxt UI 3 está estabilizándose; algunos componentes pueden cambiar su API.
- SeaORM es más verboso que sqlx puro para queries ad-hoc; aceptado a cambio
  de tipado fuerte de relaciones.

### Riesgos abiertos

- Cambios en la API de `keyring` 3.x o `reqwest` 0.12 podrían forzar actualizaciones puntuales.

## Alternativas consideradas

### A. Tauri 1.x

Más estable pero con API de comandos legada y plugins menos modulares. Empezar
en 1.x significaría migrar a corto plazo.

### B. sqlx directo sin ORM

Más rápido para queries y menos abstracción, pero perderíamos generación
automática de entidades, relaciones tipadas y migraciones expresivas. No
justifica el ahorro de overhead para un MVP local-first.

### C. Diesel

Madurez excelente pero el ecosistema async tokio nativo lo hace menos
ergonómico que SeaORM para este caso.

## Referencias

- `Requerimiento.md` §1.3, §5.1.
