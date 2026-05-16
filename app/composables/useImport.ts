import type { DeepSeekResult, Scene } from '~/types/domain'

interface RawImport {
  id: string
  project_id: string
  scene_id: string | null
  source_type: string
  source_file_path: string | null
  original_text: string
  processed_json: string | null
  status: string
  error_message: string | null
  created_at: string
}

export function useImport() {
  const { invoke } = useTauri()

  return {
    importText: (projectId: string, text: string) =>
      invoke<RawImport>('import_text', { projectId, text }),
    importFile: (projectId: string, filePath: string) =>
      invoke<RawImport>('import_file', { projectId, filePath }),
    processWithDeepSeek: (rawImportId: string) =>
      invoke<DeepSeekResult>('process_import_with_deepseek', { rawImportId }),
    validate: (rawImportId: string) =>
      invoke<DeepSeekResult>('validate_import_result', { rawImportId }),
    createScene: (rawImportId: string) =>
      invoke<Scene>('create_scene_from_import', { rawImportId }),
  }
}
