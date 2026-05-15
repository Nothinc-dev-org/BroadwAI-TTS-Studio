import type { Scene } from '~/types/domain'

export function useScenes() {
  const { invoke } = useTauri()

  return {
    list: (projectId: string) => invoke<Scene[]>('list_scenes', { projectId }),
    get: (id: string) => invoke<Scene | null>('get_scene', { id }),
    create: (params: {
      projectId: string
      title: string
      description?: string
      orderIndex?: number
    }) =>
      invoke<Scene>('create_scene', {
        projectId: params.projectId,
        title: params.title,
        description: params.description ?? null,
        orderIndex: params.orderIndex ?? null,
      }),
    update: (params: {
      id: string
      title?: string
      description?: string
      orderIndex?: number
    }) => invoke<void>('update_scene', params),
    remove: (id: string) => invoke<void>('delete_scene', { id }),
    reorder: (projectId: string, orderedIds: string[]) =>
      invoke<void>('reorder_scenes', { projectId, orderedIds }),
  }
}
