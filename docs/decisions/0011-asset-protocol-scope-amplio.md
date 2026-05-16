# 0011 — `assetProtocol` con scope amplio durante el MVP

- **Fecha:** 2026-05-15
- **Estado:** Aceptada
- **Decisores:** Backend Tauri, Frontend

## Contexto

Tras Sprint 3 (TTS por diálogo) el frontend necesita reproducir archivos de
audio que viven en disco bajo `<project-root>/audio/generated/`. En Sprint 5
se suman los assets importados bajo `<project-root>/assets/<kind>/`.

Tauri 2 expone el protocolo `asset://` para que el WebView pueda leer
archivos locales sin pasar por la capa de comandos. El protocolo está
deshabilitado por defecto y requiere:

1. Activar la feature `protocol-asset` en `tauri = { features = [...] }`.
2. Habilitarlo en `tauri.conf.json::app.security.assetProtocol`.
3. Declarar un `scope` con los patrones permitidos.

La alternativa "pura Rust" — devolver los bytes del archivo en cada `play()`
mediante un comando — escala mal con archivos de varios MB y no permite
streaming nativo del `<audio>` HTML.

## Decisión

Habilitamos `assetProtocol` con `scope: ["**"]` mientras dura el MVP. Es
decir, el WebView puede leer **cualquier** archivo del sistema de archivos
del usuario.

```jsonc
// src-tauri/tauri.conf.json
"security": {
  "csp": null,
  "assetProtocol": {
    "enable": true,
    "scope": ["**"]
  }
}
```

```toml
# src-tauri/Cargo.toml
tauri = { version = "2", features = ["protocol-asset"] }
```

## Consecuencias

### Positivas

- `<audio :src="convertFileSrc(path)">` funciona de forma transparente para
  audios generados, assets importados y mezclas exportadas.
- Sin overhead por leer/serializar bytes en cada reproducción.
- Soporta seeking nativo del navegador (`<audio controls>` con barra de
  progreso) y precarga del WebView.

### Negativas / costos asumidos

- **Superficie de ataque ampliada:** un XSS en el WebView podría leer
  cualquier archivo del usuario. La app es local-first y no acepta input
  remoto, pero se asume el riesgo conscientemente.
- Se incumple temporalmente el principio de mínimo privilegio.

### Riesgos abiertos

- **Pendiente pre-release:** restringir `scope` a las rutas reales del
  proyecto abierto (algo como `["<root>/audio/**", "<root>/assets/**", "<root>/exports/**"]`).
  Eso implica reconfigurar `assetProtocol` dinámicamente al abrir un
  proyecto, lo cual no es trivial con la API actual de Tauri 2.
- Definir también CSP estricta en el mismo movimiento (RNF pendiente).

## Alternativas consideradas

### A. Comando `read_audio_bytes(path) → Vec<u8>`

Devolver los bytes vía IPC y crear un `Blob` URL en el frontend. Descartada
por costo de memoria con archivos largos (>10 MB), pérdida de streaming y
saturación del canal IPC.

### B. Servir vía `tauri-plugin-http` local

Levantar un servidor HTTP local que sirva los archivos del proyecto.
Descartada: añade superficie (puerto abierto), complica la sandbox de
Tauri y reintroduce el problema de scoping.

### C. Habilitar `assetProtocol` con scope ya restringido

Es lo correcto, pero requiere reconfigurar el scope al abrir/cerrar
proyecto. Se difiere al sprint de hardening pre-release.

## Referencias

- RNF-01 (seguridad de credenciales) — independiente, pero comparte el
  espíritu de minimizar superficie.
- [`docs/architecture.md`](../architecture.md) §5 (Seguridad).
- Tauri 2 docs: <https://v2.tauri.app/security/asset-protocol/>.
