import type { Project } from '~/types/domain'

export function useProjects() {
  const { invoke } = useTauri()

  return {
    create: (params: {
      title: string
      description?: string
      language?: string
      rootPath: string
    }) =>
      invoke<Project>('create_project', {
        title: params.title,
        description: params.description ?? null,
        language: params.language ?? null,
        rootPath: params.rootPath,
      }),

    open: (rootPath: string) => invoke<void>('open_project', { rootPath }),

    listRecent: () => invoke<Project[]>('list_recent_projects'),

    update: (params: {
      id: string
      title?: string
      description?: string
      language?: string
    }) =>
      invoke<Project>('update_project', {
        id: params.id,
        title: params.title ?? null,
        description: params.description ?? null,
        language: params.language ?? null,
      }),

    remove: (id: string) => invoke<void>('delete_project', { id }),

    export: (id: string, targetPath: string) =>
      invoke<string>('export_project', { id, targetPath }),

    import: (sourcePath: string, targetRootPath: string) =>
      invoke<Project>('import_project', { sourcePath, targetRootPath }),
  }
}
