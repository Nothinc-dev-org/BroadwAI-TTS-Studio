import type {
  ExportFormat,
  SceneMixResult,
  TimelineEvent,
  TimelineTrack,
} from '~/types/domain'

export interface EventUpdate {
  start_ms?: number
  duration_ms?: number
  offset_ms?: number
  volume?: number
  fade_in_ms?: number
  fade_out_ms?: number
  playback_rate?: number
  looping?: boolean
}

export function useTimeline() {
  const { invoke } = useTauri()

  return {
    listTracks: (sceneId: string) =>
      invoke<TimelineTrack[]>('list_timeline_tracks', { sceneId }),
    listEvents: (sceneId: string) =>
      invoke<TimelineEvent[]>('list_timeline_events', { sceneId }),
    createTrack: (sceneId: string, name: string, kind: string) =>
      invoke<TimelineTrack>('create_timeline_track', { sceneId, name, kind }),
    updateTrack: (params: {
      id: string
      name?: string
      volume?: number
      muted?: boolean
      solo?: boolean
    }) => invoke<TimelineTrack>('update_timeline_track', params),
    deleteTrack: (id: string) =>
      invoke<void>('delete_timeline_track', { id }),
    createEvent: (sceneId: string, audioAssetId: string, startMs: number) =>
      invoke<TimelineEvent>('create_timeline_event', { sceneId, audioAssetId, startMs }),
    updateEvent: (id: string, update: EventUpdate) =>
      invoke<TimelineEvent>('update_timeline_event', { id, update }),
    deleteEvent: (id: string) =>
      invoke<void>('delete_timeline_event', { id }),
    render: (sceneId: string, outputPath: string, format: ExportFormat) =>
      invoke<SceneMixResult>('render_timeline', { sceneId, outputPath, format }),
  }
}
