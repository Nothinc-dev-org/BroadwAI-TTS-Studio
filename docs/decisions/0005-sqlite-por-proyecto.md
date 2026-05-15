# 0005 — SQLite separada por proyecto

- **Fecha:** 2026-05-15
- **Estado:** Aceptada

## Contexto

`Requerimiento.md` define proyectos como carpetas locales con sub-estructura
fija (RF-01.4) que incluye `database/project.sqlite`. La app es local-first y
multi-proyecto, pero no multi-usuario ni colaborativa (RF §3.2).

Decisión a tomar: ¿una BD global con todos los proyectos del usuario, o una BD
por proyecto?

## Decisión

Una **SQLite por proyecto**, dentro de `<root>/database/project.sqlite`. La
aplicación abre/cierra esta BD al cambiar de proyecto vía `AppState::open`.

Las preferencias de usuario que crucen proyectos (modelo de DeepSeek por
defecto, política de generación automática) viven en `app_settings` **dentro
de cada proyecto**, no en una BD global. Las que son verdaderamente globales
(API keys) viven en el keyring del SO.

## Consecuencias

### Positivas

- **Portabilidad:** copiar la carpeta del proyecto a otra máquina mueve toda
  la información (proyecto, escenas, audios, assets, configuraciones).
- **Backups simples:** rsync o tar de la carpeta = backup completo.
- **Aislamiento:** corromper un proyecto no afecta a otros.
- **Tamaño manejable:** una BD por proyecto evita un único archivo gigante.

### Negativas / costos asumidos

- No hay vista cross-proyecto (p. ej. "todas las escenas de todos mis
  proyectos"). Para MVP no es necesario.
- "Proyectos recientes" no se persiste en BD; vive en un archivo global de
  configuración aparte (pendiente: por ahora la lista no se persiste).

### Riesgos abiertos

- Lista de "proyectos recientes" requiere otro mecanismo (archivo JSON en el
  data dir del SO). Pendiente de implementar.

## Alternativas consideradas

### A. BD global única

Sencillo de implementar pero rompe portabilidad y backups por proyecto.

### B. Híbrido: BD global con metadatos + BDs por proyecto

Posible, pero añade complejidad sin beneficio inmediato. Si necesitamos vista
cross-proyecto en el futuro, escribimos un servicio de agregación que abra
varias BDs en read-only.

## Referencias

- `Requerimiento.md` RF-01 (estructura de carpetas), §4.1 (local-first).
- `src-tauri/src/paths.rs`, `src-tauri/src/state.rs::AppState::open`.
