export function useTimeline() {
  const { invoke } = useTauri()

  return {
    createTrack: (sceneId: string, name: string, kind: string) =>
      invoke<void>('create_timeline_track', { sceneId, name, kind }),
    updateTrack: (params: {
      id: string
      name?: string
      volume?: number
      muted?: boolean
      solo?: boolean
    }) => invoke<void>('update_timeline_track', params),
    deleteTrack: (id: string) => invoke<void>('delete_timeline_track', { id }),
    createEvent: (sceneId: string, trackId: string, startMs: number) =>
      invoke<void>('create_timeline_event', { sceneId, trackId, startMs }),
    updateEvent: (id: string) => invoke<void>('update_timeline_event', { id }),
    deleteEvent: (id: string) => invoke<void>('delete_timeline_event', { id }),
    render: (sceneId: string, outputPath: string, format: 'wav' | 'mp3') =>
      invoke<void>('render_timeline', { sceneId, outputPath, format }),
  }
}
