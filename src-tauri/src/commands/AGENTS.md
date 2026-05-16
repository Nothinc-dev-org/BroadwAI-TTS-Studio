# AGENTS.md — `src-tauri/src/commands/`

## Propósito

Capa de **transport**. Cada archivo expone una familia de `#[tauri::command]`
que el frontend invoca vía `useTauri().invoke('nombre')`.

## Contrato

Cada comando:

1. Lleva el atributo `#[tauri::command]`.
2. Recibe `State<'_, AppState>` cuando necesite acceso a DB o credenciales.
3. Devuelve `AppResult<T>` (alias de `Result<T, AppError>`).
4. Está **registrado** en `tauri::generate_handler!` dentro de `lib.rs`. Si
   olvidas este paso, el frontend recibe `command not found`.
5. **No** contiene lógica de negocio: delega a un servicio.

Ver `docs/decisions/0007-commands-thin-services-stateless.md`.

## Mapa de módulos

| Archivo        | Dominio                              | Servicios usados                                             |
| -------------- | ------------------------------------ | ------------------------------------------------------------ |
| `project.rs`   | Proyectos (RF-01, 02, 39, 40)        | `project_service`, `project_io_service`                      |
| `scene.rs`     | Escenas (RF-04, 05)                  | `scene_service`                                              |
| `character.rs` | Personajes y alias (RF-06–08, 23)    | `character_service`                                          |
| `dialogue.rs`  | Bloques narrativos (RF-17–21)        | `dialogue_service`                                           |
| `import.rs`    | Copy-paste / archivo (RF-09–16)      | `import_service` (que a su vez orquesta `deepseek_service`)  |
| `tts.rs`       | Generación, play, bytes de audio, muestras y optimización TTS | `tts_service`, `scene_mix_service`, `tts_optimization_service` |
| `timeline.rs`  | Pistas, eventos y render (RF-34–37)  | `timeline_service`, `scene_mix_service`                      |
| `assets.rs`    | Biblioteca de audio (RF-32, 33)      | `asset_service`                                              |
| `settings.rs`  | API keys y preferencias (RF-03)      | `credential_service`                                         |

## Reglas de oro

- Una firma de comando es **un contrato público con el frontend**: cambiarla
  obliga a actualizar el composable de `app/composables/` correspondiente y
  el tipo en `app/types/domain.ts` si aplica.
- Nombres en `snake_case` para los comandos (Rust convención). El frontend los
  invoca como strings, por lo que el snake_case se preserva en `invoke('…')`.
- Argumentos opcionales: usar `Option<T>` y serializar como `null` desde el
  frontend (los composables ya lo hacen).
- Errores: nunca `.unwrap()` o `.expect()`. Si un error ocurre, propagar con
  `?`. Si la condición es "esto no debería pasar nunca", usar
  `AppError::internal("descripción")`.

## Añadir un comando nuevo

1. Escribir la función en el módulo correspondiente.
2. Añadir el nombre a `tauri::generate_handler!` en `lib.rs`.
3. Añadir el binding en el composable correspondiente.
4. Si introduce un tipo nuevo de retorno, replicar en `app/types/domain.ts`.

## Estado actual

MVP 1 cerrado. Todos los comandos relevantes para el roadmap están
cableados a sus servicios. Los stubs que quedan (`update_scene`,
`update_character`, `split_dialogue_node`, `merge_dialogue_nodes`,
`delete_project`, `generate_scene_audio`, `play_scene_audio` y otros del
módulo `dialogue.rs`/`scene.rs`/`character.rs`) corresponden a CRUD
ampliado del editor visual y a la deuda post-MVP listada en
`docs/architecture.md` §7.
