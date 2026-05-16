use sea_orm_migration::prelude::*;

use super::m20260515_000002_create_scenes::Scenes;
use super::m20260515_000004_create_dialogue_nodes::DialogueNodes;
use super::m20260515_000005_create_audio_tables::{AudioAssets, GeneratedAudio};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TimelineTracks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TimelineTracks::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TimelineTracks::SceneId).text().not_null())
                    .col(ColumnDef::new(TimelineTracks::Name).text().not_null())
                    .col(ColumnDef::new(TimelineTracks::Type).text().not_null())
                    .col(
                        ColumnDef::new(TimelineTracks::OrderIndex)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TimelineTracks::Volume)
                            .double()
                            .not_null()
                            .default(1.0),
                    )
                    .col(
                        ColumnDef::new(TimelineTracks::Muted)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TimelineTracks::Solo)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timeline_tracks_scene")
                            .from(TimelineTracks::Table, TimelineTracks::SceneId)
                            .to(Scenes::Table, Scenes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TimelineEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TimelineEvents::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TimelineEvents::SceneId).text().not_null())
                    .col(ColumnDef::new(TimelineEvents::TrackId).text().not_null())
                    .col(ColumnDef::new(TimelineEvents::DialogueNodeId).text())
                    .col(ColumnDef::new(TimelineEvents::AudioAssetId).text())
                    .col(ColumnDef::new(TimelineEvents::GeneratedAudioId).text())
                    .col(ColumnDef::new(TimelineEvents::StartMs).integer().not_null())
                    .col(ColumnDef::new(TimelineEvents::DurationMs).integer())
                    .col(
                        ColumnDef::new(TimelineEvents::OffsetMs)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TimelineEvents::Volume)
                            .double()
                            .not_null()
                            .default(1.0),
                    )
                    .col(
                        ColumnDef::new(TimelineEvents::FadeInMs)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TimelineEvents::FadeOutMs)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TimelineEvents::PlaybackRate)
                            .double()
                            .not_null()
                            .default(1.0),
                    )
                    .col(
                        ColumnDef::new(TimelineEvents::Loop)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(TimelineEvents::CreatedAt).text().not_null())
                    .col(ColumnDef::new(TimelineEvents::UpdatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timeline_events_scene")
                            .from(TimelineEvents::Table, TimelineEvents::SceneId)
                            .to(Scenes::Table, Scenes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timeline_events_track")
                            .from(TimelineEvents::Table, TimelineEvents::TrackId)
                            .to(TimelineTracks::Table, TimelineTracks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timeline_events_dialogue")
                            .from(TimelineEvents::Table, TimelineEvents::DialogueNodeId)
                            .to(DialogueNodes::Table, DialogueNodes::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timeline_events_asset")
                            .from(TimelineEvents::Table, TimelineEvents::AudioAssetId)
                            .to(AudioAssets::Table, AudioAssets::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timeline_events_generated")
                            .from(TimelineEvents::Table, TimelineEvents::GeneratedAudioId)
                            .to(GeneratedAudio::Table, GeneratedAudio::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_timeline_events_scene")
                    .table(TimelineEvents::Table)
                    .col(TimelineEvents::SceneId)
                    .col(TimelineEvents::StartMs)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TimelineEvents::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TimelineTracks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TimelineTracks {
    Table,
    Id,
    SceneId,
    Name,
    Type,
    OrderIndex,
    Volume,
    Muted,
    Solo,
}

#[derive(DeriveIden)]
pub enum TimelineEvents {
    Table,
    Id,
    SceneId,
    TrackId,
    DialogueNodeId,
    AudioAssetId,
    GeneratedAudioId,
    StartMs,
    DurationMs,
    OffsetMs,
    Volume,
    FadeInMs,
    FadeOutMs,
    PlaybackRate,
    Loop,
    CreatedAt,
    UpdatedAt,
}
