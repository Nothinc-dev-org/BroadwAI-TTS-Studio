# 0002 — Keyring del SO para API keys

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

RNF-01 exige que las API keys (DeepSeek, Gemini) **no** se guarden en texto
plano en SQLite, no aparezcan en `localStorage`, no se impriman en logs ni se
expongan en errores hacia el frontend (RNF-01, RF-03). El proyecto será
publicado como open source, por lo que cualquier descuido de manejo de
credenciales sería visible públicamente en el código.

Necesitamos un mecanismo confiable cross-platform que delegue la confidencialidad
al sistema operativo.

## Decisión

Usar el crate **`keyring` 3.x** con backend persistente por sistema operativo:

- macOS/iOS: `apple-native` (Keychain).
- Windows: `windows-native` (Credential Manager / DPAPI).
- Linux/BSD: `sync-secret-service` + `crypto-rust` (Secret Service/libsecret).

No usar `linux-native` para esta aplicación: en `keyring` 3.x ese feature
selecciona `keyutils`, que funciona como cache en memoria del kernel y se limpia
al reiniciar el sistema.

- Servicio: `ai.broadwai.tts-studio`.
- Cuentas: `deepseek`, `gemini` (definidas en `Provider::keyring_account`).
- Acceso: encapsulado en `services::credential_service::CredentialService`.
- Lectura: método `read(provider)` con visibilidad `pub(crate)` para que solo
  los servicios HTTP (DeepSeek/Gemini) puedan obtener la key cruda.
- Error de keyring: convertido a `AppError::Credential` (variante sin payload)
  para evitar que detalles internos viajen al frontend.

Detalles operativos:

- Linux: Secret Service/libsecret (gnome-keyring / kwallet). Requiere
  `libsecret-1-dev` en build y un agente de keyring activo/desbloqueado en runtime.
- macOS: Keychain.
- Windows: Credential Manager (DPAPI).

## Consecuencias

### Positivas

- Confidencialidad delegada al SO: si el usuario tiene gnome-keyring bloqueado,
  la app no puede leer la key.
- No hay almacenamiento de keys en disco controlado por la app.
- El frontend nunca recibe la key tras `set_api_key`; solo recibe un
  `ApiKeyStatus`.

### Negativas / costos asumidos

- En Linux requiere libsecret instalada y un agente de keyring activo. En
  servidores headless es una fricción real; mitigamos asumiendo que la app es
  exclusivamente desktop.
- El usuario que no tenga keyring configurado verá un error genérico
  (`AppError::Credential`) sin detalle, lo que dificulta el soporte. Mitigación:
  loggear el error original interno con `tracing::warn!` para diagnóstico local.

### Riesgos abiertos

- La elección de backend de keyring en Linux puede divergir entre distros
  (gnome-keyring vs kwallet vs KeePassXC). Documentar en `README.md` que se
  requiere un keyring estándar.

## Alternativas consideradas

### A. Archivo cifrado local (AES-GCM)

Requeriría derivar una clave de algo (machine ID, contraseña del usuario), y
en ambos casos la app tendría que decidir cómo proteger esa clave maestra,
generando un problema recursivo. Además abre la puerta a "passphrase fatigue"
si pedimos contraseña.

### B. SQLCipher para cifrar la BD entera

Cifrar la BD entera es ortogonal a proteger las keys; las keys siguen
necesitando una llave maestra. Y SQLCipher introduce una dependencia C nativa
que complica builds cross-platform.

### C. Variables de entorno

Trasladaría el problema al usuario y dejaría las keys en archivos `.env` o
shell rc, que es exactamente lo que RNF-01 quiere evitar.

## Referencias

- `Requerimiento.md` RF-03, RNF-01.
- `src-tauri/src/services/credential_service.rs`.
- `src-tauri/src/error.rs` — variante `AppError::Credential`.
