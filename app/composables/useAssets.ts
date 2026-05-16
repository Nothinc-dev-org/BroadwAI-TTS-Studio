import type { AudioAsset, AudioAssetKind } from '~/types/domain'

export function useAssets() {
  const { invoke } = useTauri()

  return {
    list: (projectId: string) =>
      invoke<AudioAsset[]>('list_audio_assets', { projectId }),
    import: (params: {
      projectId: string
      filePath: string
      kind: AudioAssetKind
      name?: string
    }) =>
      invoke<AudioAsset>('import_audio_asset', {
        projectId: params.projectId,
        filePath: params.filePath,
        kind: params.kind,
        name: params.name ?? null,
      }),
    update: (id: string, params: { name?: string, kind?: AudioAssetKind }) =>
      invoke<AudioAsset>('update_audio_asset', { id, ...params }),
    remove: (id: string) =>
      invoke<void>('delete_audio_asset', { id }),
    preview: (id: string) =>
      invoke<string>('preview_audio_asset', { id }),
  }
}
