# 0003 — bun como gestor y runner de JS

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

Necesitamos un gestor de paquetes y runner JS para Nuxt 4. La opción
predeterminada en la comunidad Nuxt suele ser `pnpm` o `npm`, pero el equipo
ya tiene `bun` instalado y prefiere su velocidad de install (5–10× sobre
npm en proyectos con dependencias tipo Nuxt).

## Decisión

Adoptar **bun** (1.3+) como único gestor y runner para todo lo de JS:

- `bun install` para resolver dependencias.
- `bun run <script>` para ejecutar scripts de `package.json`.
- `tauri.conf.json` invoca `bun run dev` y `bun run generate` como pre-commands.

No usar `npm` ni `pnpm` en ningún script, documentación o instrucción CI.

## Consecuencias

### Positivas

- Install drásticamente más rápido (5–10×).
- `bun` también es runtime, lo que abre la opción futura de scripts de
  utilidades (validación de prompts, generación de fixtures) sin Node.

### Negativas / costos asumidos

- Algunos paquetes muy específicos pueden tener bugs solo en bun.
  Mitigación: si aparece uno, documentar workaround o fallback puntual a
  `node_modules` ejecutados con `node` (sin cambiar gestor).
- Contribuyentes externos que solo conozcan npm necesitan instalar bun. El
  README cubre el prerrequisito.

### Riesgos abiertos

- `bun` evoluciona rápido. Pinear la versión major en CI si llegamos a tenerlo.

## Alternativas consideradas

### A. pnpm

Excelente, ya maduro, pero el equipo ya está moviéndose a bun y mantener dos
flujos es overhead.

### B. npm

Lento en proyectos con dependencias profundas como Nuxt + Nuxt UI.

## Referencias

- `Requerimiento.md` no especifica gestor JS; queda a discreción técnica.
- `package.json` scripts, `src-tauri/tauri.conf.json` `beforeDevCommand`.
