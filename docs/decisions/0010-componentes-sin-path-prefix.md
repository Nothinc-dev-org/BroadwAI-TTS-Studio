# 0010 — Auto-import de componentes sin path prefix

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

Nuxt 4 por defecto auto-importa componentes con prefijo basado en la ruta
relativa a `components/`. Así, `components/audio/RenderQueue.vue` se registra
como `<AudioRenderQueue>`. Para componentes que ya empiezan con el nombre del
directorio (`components/scene/SceneEditor.vue` → `<SceneEditor>`), el prefijo
se colapsa.

Las páginas referencian componentes con nombres planos (`<RenderQueue>`,
`<SceneEditor>`, `<CharacterPanel>`), lo que genera errores `Component not
found` en los que viven en directorios con nombre distinto del prefijo del
fichero.

## Decisión

Configurar `nuxt.config.ts` con:

```ts
components: [
  { path: '~/components', pathPrefix: false },
]
```

Todos los componentes se auto-importan por su nombre de fichero, sin importar
el directorio. Los directorios siguen organizándose por dominio
(`audio/`, `scene/`, `character/`, `timeline/`, `import/`, `project/`) para
mantener orden del repositorio.

## Consecuencias

### Positivas

- Páginas y componentes se referencian con el nombre del fichero, lo que es
  lo que un humano espera.
- La organización por dominio sigue siendo visible en el árbol de archivos
  sin obligar a nombres compuestos.

### Negativas / costos asumidos

- Si dos componentes en directorios distintos comparten nombre, hay
  colisión. Mitigación: convención de nombres unique-por-fichero. En el
  scaffold actual no hay colisiones.

### Riesgos abiertos

- A futuro, si añadimos componentes de terceros que también auto-importan,
  podría haber colisiones. Vigilar.

## Alternativas consideradas

### A. Usar el prefijo por defecto

Obliga a referenciar `<AudioRenderQueue>` y `<TimelineTimelineEvent>`,
nombres duplicados o feos.

### B. Renombrar componentes para que coincidan con el prefijo

Trabajoso y obligaría a `components/audio/AudioRenderQueue.vue`, que es
ruidoso.

## Referencias

- `nuxt.config.ts` — opción `components`.
- Docs Nuxt 4: https://nuxt.com/docs/api/nuxt-config#components
