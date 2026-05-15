use sea_orm_migration::prelude::*;

use super::m20260515_000001_create_projects::Projects;
use super::m20260515_000004_create_dialogue_nodes::DialogueNodes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AudioAssets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AudioAssets::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AudioAssets::ProjectId).text().not_null())
                    .col(ColumnDef::new(AudioAssets::Name).text().not_null())
                    .col(ColumnDef::new(AudioAssets::Type).text().not_null())
                    .col(ColumnDef::new(AudioAssets::FilePath).text().not_null())
                    .col(ColumnDef::new(AudioAssets::DurationMs).integer())
                    .col(ColumnDef::new(AudioAssets::OriginalFileName).text())
                    .col(ColumnDef::new(AudioAssets::MimeType).text())
                    .col(ColumnDef::new(AudioAssets::CreatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audio_assets_project")
                            .from(AudioAssets::Table, AudioAssets::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GeneratedAudio::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GeneratedAudio::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GeneratedAudio::DialogueNodeId)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GeneratedAudio::Provider).text().not_null())
                    .col(ColumnDef::new(GeneratedAudio::Model).text().not_null())
                    .col(ColumnDef::new(GeneratedAudio::VoiceId).text().not_null())
                    .col(ColumnDef::new(GeneratedAudio::InputHash).text().not_null())
                    .col(ColumnDef::new(GeneratedAudio::FilePath).text().not_null())
                    .col(ColumnDef::new(GeneratedAudio::DurationMs).integer())
                    .col(ColumnDef::new(GeneratedAudio::Status).text().not_null())
                    .col(ColumnDef::new(GeneratedAudio::ErrorMessage).text())
                    .col(ColumnDef::new(GeneratedAudio::CreatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_generated_audio_node")
                            .from(GeneratedAudio::Table, GeneratedAudio::DialogueNodeId)
                            .to(DialogueNodes::Table, DialogueNodes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_generated_audio_node")
                    .table(GeneratedAudio::Table)
                    .col(GeneratedAudio::DialogueNodeId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_generated_audio_hash")
                    .table(GeneratedAudio::Table)
                    .col(GeneratedAudio::InputHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GeneratedAudio::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AudioAssets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AudioAssets {
    Table,
    Id,
    ProjectId,
    Name,
    Type,
    FilePath,
    DurationMs,
    OriginalFileName,
    MimeType,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum GeneratedAudio {
    Table,
    Id,
    DialogueNodeId,
    Provider,
    Model,
    VoiceId,
    InputHash,
    FilePath,
    DurationMs,
    Status,
    ErrorMessage,
    CreatedAt,
}
