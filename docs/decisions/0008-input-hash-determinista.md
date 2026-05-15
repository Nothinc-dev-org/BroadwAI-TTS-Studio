# 0008 — Caché TTS por input_hash determinístico

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

RF-30 exige no regenerar audio si el bloque no cambió. El "no cambió" depende
de varios campos: texto, speaker, voice_id, modelo, tags TTS, prompt de estilo.
Una solución basada en `updated_at` es frágil porque cualquier `UPDATE` que no
afecte al audio invalidaría el caché.

RT-06 (riesgo de costo de API) hace este caché crítico, no opcional.

## Decisión

Calcular un **hash determinístico SHA-256** (`input_hash`) sobre la
concatenación con separadores explícitos de:

```
text | voice_id | model | tag_signature | style_prompt?
```

donde `tag_signature` es la serialización ordenada y canónica de los tags
TTS del bloque.

La función vive en `services::render_planner::input_hash` y es la única
forma legítima de generar el hash. La tabla `generated_audio` indexa
`input_hash` para búsquedas O(1).

Reglas:

- Antes de generar audio, calcular hash y buscar `generated_audio` con ese
  hash y status != `outdated`. Si existe, reutilizar `file_path`.
- Editar texto, speaker, voz, modelo, tags o prompt de estilo cambia el
  hash → nuevo job de generación.
- Editar delays, volumen, fade, orden de timeline, mute/solo **no** afecta
  al hash (RF-38).

## Consecuencias

### Positivas

- Caché correcto por construcción: si el hash coincide, el audio es el
  mismo. Si no, hay que regenerar.
- Permite "deduplicación" entre bloques idénticos (mismo texto + voz + tags)
  reusando el mismo `file_path`.
- Independiente del orden de inserción y de timestamps; deterministicidad
  perfecta.

### Negativas / costos asumidos

- Cambios cosméticos en tags (`[warm]` vs `[ warm ]`) afectarían al hash si
  no canonicalizamos. Mitigación: `tag_signature` debe trimear y ordenar.
- Si Gemini cambia su modelo bajo el mismo nombre, mantenemos audios
  "vigentes" técnicamente pero generados por una versión anterior.
  Mitigación: incluir un `model` versionado (`gemini-3.1-flash-tts-preview`,
  no solo `gemini-flash`) y bumpear cuando cambien firma de modelo.

### Riesgos abiertos

- El concepto de "tag_signature canónico" no está implementado todavía. ADR
  pendiente o issue cuando se implemente generación real.

## Alternativas consideradas

### A. Cache por `updated_at`

Frágil: cualquier update invalida.

### B. Cache por diff explícito

Más preciso, mucho más caro en complejidad de código.

## Referencias

- `Requerimiento.md` RF-30, RF-38, RT-06.
- `src-tauri/src/services/render_planner.rs`.
