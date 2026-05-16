# 0012 — `symphonia` para decodificación universal + resample lineal en el mixer

- **Fecha:** 2026-05-15
- **Estado:** Aceptada
- **Decisores:** Backend Audio

## Contexto

El mixer (`services/audio_mixer.rs`) tiene que componer en una sola pista
maestra tres familias de fuentes:

1. Audios TTS de Gemini (WAV PCM L16, 24 kHz, mono).
2. Audios generados por el TTS local cacheados (WAV).
3. Assets importados por el usuario (RF-32): **`.wav` / `.mp3` / `.ogg` / `.flac`**.

`hound` solo decodifica WAV. `symphonia` cubre los cuatro formatos con una
única API. El sample rate del output del mixer es **24 kHz mono 16-bit**,
fijado por consistencia con Gemini (decisión de ADR-0008 / `input_hash`).

Cuando una fuente tiene un sample rate distinto al objetivo (música a
44.1 kHz, SFX a 48 kHz), hay que resamplear. `rubato` está disponible en
deps (decisión ADR-0004) pero su API en 0.16 requiere chunks de tamaño fijo
y manejo manual de la cola; introducirlo correctamente cuesta tiempo y la
mayoría de assets del MVP vienen ya cerca de 24 kHz.

## Decisión

1. Toda decodificación pasa por **`symphonia::default::get_probe`** vía la
   función pública `audio_mixer::decode_to_mono(path)`, que devuelve
   `Vec<f32>` mono normalizado al sample rate objetivo.
2. El resample se hace con **interpolación lineal** inline en
   `audio_mixer::resample_linear`, sin dependencias adicionales.
3. `rubato` permanece en `Cargo.toml` como dependencia "lista para activar"
   cuando se requiera calidad de resample.

```rust
pub const TARGET_SAMPLE_RATE: u32 = 24_000;

pub fn decode_to_mono(path: &Path) -> AppResult<Vec<f32>> {
    let (samples, sample_rate, channels) = decode_with_symphonia(path)?;
    let mono = downmix_to_mono(samples, channels);
    if sample_rate == TARGET_SAMPLE_RATE {
        Ok(mono)
    } else {
        Ok(resample_linear(mono, sample_rate, TARGET_SAMPLE_RATE))
    }
}
```

## Consecuencias

### Positivas

- Un único punto de entrada para decodificar audio en todo el backend.
- Soporte automático de WAV/MP3/OGG/FLAC sin código específico por formato.
- Cero coste adicional (resample lineal es O(n) y sin allocs intermedios).
- Cuando el sample rate ya coincide (caso 99% TTS de Gemini), no hay
  trabajo de resample en absoluto.

### Negativas / costos asumidos

- Resample lineal introduce **aliasing audible** cuando el ratio es
  agresivo (p. ej. 48 kHz → 24 kHz pierde calidad apreciable en alta
  frecuencia). Aceptable para SFX/voz, **no** para música mezclada a alto
  volumen.
- `symphonia` añade ~200 KB al binario por sus codecs `all-codecs all-formats`,
  pero ya pagamos ese costo desde el inicio del proyecto.

### Riesgos abiertos

- Si se incorpora una funcionalidad de mastering musical, este resample
  será insuficiente. Migrar a `rubato::FftFixedInOut` queda como
  follow-up; la API pública del mixer (`decode_to_mono` + sample rate
  objetivo) no cambia, así que el upgrade es local.

## Alternativas consideradas

### A. Solo `hound` (WAV únicamente)

Descartada: incumple RF-32, que exige `.wav .mp3 .ogg .flac`.

### B. `rodio` con decoder + sink

`rodio` es de más alto nivel y orientado a *playback*, no a renderizar
buffers. Descartada por no encajar con el patrón "decodificar → componer →
escribir WAV" del mixer.

### C. Adoptar `rubato` desde Sprint 5

Lo correcto a largo plazo, pero requiere lidiar con el manejo de tail
(`process_partial`), chunk sizes y latency compensation. Coste de
implementación alto vs. ganancia inaudible mientras todas las fuentes
vivan cerca de 24 kHz.

## Referencias

- RF-26, RF-32, RF-36, RF-37 en `Requerimiento.md`.
- ADR-0004 (mezcla de audio en Rust).
- ADR-0008 (input_hash determinista — fija el sample rate de la cadena TTS).
- symphonia: <https://docs.rs/symphonia/latest/symphonia/>.
- rubato: <https://docs.rs/rubato/latest/rubato/>.
