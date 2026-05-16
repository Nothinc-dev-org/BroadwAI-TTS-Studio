# AGENTS.md — `app/composables/`

## Propósito

Capa fina entre Vue y Tauri. **Un composable por dominio**, mapea 1:1 con
los módulos de `src-tauri/src/commands/`.

## Contrato

Cada composable:

- Es una función `useXxx()` que devuelve un objeto con métodos.
- Cada método llama a `invoke('xxx_command', args)` vía `useTauri()`.
- **Tipa el retorno** con el tipo correspondiente de `~/types/domain`.
- **No hace caching, no maneja estado, no transforma datos**. Es transport
  puro.

```ts
export function useScenes() {
  const { invoke } = useTauri()
  return {
    list: (projectId: string) => invoke<Scene[]>('list_scenes', { projectId }),
    create: (params: { … }) => invoke<Scene>('create_scene', { … }),
  }
}
```

## Mapa composable → comandos

| Composable             | Comandos invocados                                                                                                           |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `useTauri`             | wrapper de `invoke` (no llama comandos)                                                                                      |
| `useProjects`          | `create_project`, `open_project`, `list_recent_projects`, `update_project`, `delete_project`, `export_project`, `import_project` |
| `useScenes`            | `create_scene`, `get_scene`, `list_scenes`, `update_scene`, `delete_scene`, `reorder_scenes`                                  |
| `useCharacters`        | `create_character`, `list_characters`, `update_character`, `delete_character`, `add_character_alias`, `remove_character_alias`, `assign_character_voice` |
| `useDialogueNodes`     | `create_dialogue_node`, `list_dialogue_nodes`, `update_dialogue_node`, `delete_dialogue_node`, `split_dialogue_node`, `merge_dialogue_nodes`, `reorder_dialogue_nodes`, `update_dialogue_tts_tags` |
| `useImport`            | `import_text`, `import_file`, `process_import_with_deepseek`, `validate_import_result`, `create_scene_from_import`           |
| `useTts`               | `generate_dialogue_audio`, `play_dialogue_audio`, `regenerate_outdated_audio`, `list_generated_audio_for_scene`, `generated_audio_bytes`, `preview_voice_sample`, `generate_scene_audio`, `play_scene_audio`, `optimize_scene_tts_tags` |
| `useTimeline`          | `list_timeline_tracks`, `create_timeline_track`, `update_timeline_track`, `delete_timeline_track`, `list_timeline_events`, `create_timeline_event`, `update_timeline_event`, `delete_timeline_event`, `render_timeline` |
| `useAssets`            | `import_audio_asset`, `list_audio_assets`, `update_audio_asset`, `delete_audio_asset`, `preview_audio_asset`                  |
| `useSettings`          | `set_api_key`, `delete_api_key`, `test_api_key`, `get_api_key_status`, `get_app_settings`, `update_app_settings`              |

## Reglas críticas

1. **Argumentos en camelCase desde TS, snake_case en Rust**. Tauri lo traduce
   automáticamente (`projectId` ↔ `project_id`). Mantener la convención
   esperada por cada lenguaje.
2. **Opcional ⇒ `null` explícito**, no `undefined`. Tauri serializa
   `undefined` como ausencia de campo y Rust falla deserializando un
   `Option<T>` ausente. Convertir con `value ?? null`.
3. **No introducir lógica**. Si necesitas combinar dos llamadas, hazlo en la
   página o en un composable de orquestación específico (no en estos).
4. **No persistir resultados en módulos**. Estos composables son "factories",
   no "stores". El estado vive en la página o componente que invoca.
5. **Reproducir audio:** usar `convertFileSrc(path)` de
   `@tauri-apps/api/core` para convertir un `file_path` absoluto a una URL
   que el WebView puede consumir desde `<audio src>`. Requiere
   `assetProtocol` habilitado en `tauri.conf.json`
   (ver [ADR-0011](../../docs/decisions/0011-asset-protocol-scope-amplio.md)).

## Notas sobre `useTauri`

Detecta si estamos dentro del WebView de Tauri (presencia de
`__TAURI_INTERNALS__` en `window`). En modo navegador puro, `invoke` lanza
un error explícito en vez de quedarse colgado. Esto permite mostrar un
banner "Backend no disponible" en lugar de un fallo silencioso.
