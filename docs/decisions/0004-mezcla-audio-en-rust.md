# 0004 — Mezcla de audio en Rust (no Web Audio)

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

Una vez generados los audios TTS, hay que mezclarlos con SFX, música y
ambiente (RF-26, RF-36, RF-37) y exportar el resultado (RF-28, formatos
WAV/MP3 en MVP, OGG/FLAC en futuro). El procesamiento puede correr en:

1. **Frontend (Web Audio API):** rápido de prototipar, integrado con el WebView.
2. **Backend (Rust):** requiere crates de audio, pero permite trabajos en
   background sin bloquear UI (RNF-08) y exportar fuera de horas de uso.

`Requerimiento.md` exige RNF-02 (manejar escenas largas) y RNF-08 (no bloquear
UI durante operaciones largas). Una escena de 100 bloques con SFX y música es
suficiente para saturar el thread del navegador.

## Decisión

Implementar la mezcla en **Rust**, en el `AudioMixer` (`services::audio_mixer`),
usando:

- **`symphonia`** para decode (WAV, MP3, OGG, FLAC, AAC, etc., con features
  `all-codecs` y `all-formats`).
- **`rubato`** para resampling cuando las fuentes tengan sample rates
  distintos.
- **`hound`** para encode WAV.
- MP3 queda para MVP 2 con un wrapper opcional (no añadimos `lame` ahora para
  evitar dependencia C en el scaffold).

El audio resultante se escribe en `<project>/audio/exports/` y se devuelve la
ruta al frontend, que lo reproduce con `<audio>` apuntando al asset local
(Tauri `asset:` protocol).

## Consecuencias

### Positivas

- Mezclas largas no bloquean la UI; el frontend solo polea estado del job.
- Mismo motor de mezcla en producción y en CI (potenciales tests
  determinísticos).
- Exportación independiente del estado del WebView.

### Negativas / costos asumidos

- Más código Rust que mantener vs. delegar a una API estándar del navegador.
- Rubato es competente pero no de nivel SoX/FFmpeg; latencia y calidad
  aceptables, no de mastering profesional (fuera de alcance del MVP).
- MP3 no llega en MVP 1; documentar limitación en UI.

### Riesgos abiertos

- Multi-canal/surround no entra en MVP. Si se demanda en MVP 3, revisitar
  rubato vs. otra opción.

## Alternativas consideradas

### A. Web Audio API en frontend

Más rápido para prototipar, pero limita el offload async y degrada con
escenas largas. El export bloqueante (offline rendering) pasa por
`OfflineAudioContext`, que sigue corriendo en el thread del WebView.

### B. FFmpeg vía CLI

Potente y maduro, pero introducir un binario externo rompe el local-first
zero-config (el usuario tendría que instalarlo) y aumenta la superficie de
seguridad.

## Referencias

- `Requerimiento.md` RF-26, RF-28, RF-36, RF-37, RNF-02, RNF-08.
- `src-tauri/src/services/audio_mixer.rs`.
