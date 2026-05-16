use sea_orm_migration::prelude::*;

use super::m20260515_000001_create_projects::Projects;
use super::m20260515_000002_create_scenes::Scenes;
use super::m20260515_000003_create_characters::Characters;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RawImports::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RawImports::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RawImports::ProjectId).text().not_null())
                    .col(ColumnDef::new(RawImports::SceneId).text())
                    .col(ColumnDef::new(RawImports::SourceType).text().not_null())
                    .col(ColumnDef::new(RawImports::SourceFilePath).text())
                    .col(ColumnDef::new(RawImports::OriginalText).text().not_null())
                    .col(ColumnDef::new(RawImports::ProcessedJson).text())
                    .col(ColumnDef::new(RawImports::Status).text().not_null())
                    .col(ColumnDef::new(RawImports::ErrorMessage).text())
                    .col(ColumnDef::new(RawImports::CreatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_raw_imports_project")
                            .from(RawImports::Table, RawImports::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_raw_imports_scene")
                            .from(RawImports::Table, RawImports::SceneId)
                            .to(Scenes::Table, Scenes::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DialogueNodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DialogueNodes::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DialogueNodes::SceneId).text().not_null())
                    .col(ColumnDef::new(DialogueNodes::CharacterId).text().not_null())
                    .col(ColumnDef::new(DialogueNodes::PreviousId).text())
                    .col(ColumnDef::new(DialogueNodes::NextId).text())
                    .col(
                        ColumnDef::new(DialogueNodes::OrderIndex)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DialogueNodes::Type).text().not_null())
                    .col(ColumnDef::new(DialogueNodes::Text).text().not_null())
                    .col(ColumnDef::new(DialogueNodes::RawText).text())
                    .col(ColumnDef::new(DialogueNodes::Emotion).text())
                    .col(ColumnDef::new(DialogueNodes::Intensity).integer())
                    .col(
                        ColumnDef::new(DialogueNodes::IsEnabled)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(DialogueNodes::BeforeDelayMs)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(DialogueNodes::AfterDelayMs)
                            .integer()
                            .default(0),
                    )
                    .col(ColumnDef::new(DialogueNodes::CreatedAt).text().not_null())
                    .col(ColumnDef::new(DialogueNodes::UpdatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dialogue_nodes_scene")
                            .from(DialogueNodes::Table, DialogueNodes::SceneId)
                            .to(Scenes::Table, Scenes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dialogue_nodes_character")
                            .from(DialogueNodes::Table, DialogueNodes::CharacterId)
                            .to(Characters::Table, Characters::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dialogue_nodes_scene")
                    .table(DialogueNodes::Table)
                    .col(DialogueNodes::SceneId)
                    .col(DialogueNodes::OrderIndex)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DialogueTtsTags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DialogueTtsTags::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DialogueTtsTags::DialogueNodeId)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DialogueTtsTags::Tag).text().not_null())
                    .col(ColumnDef::new(DialogueTtsTags::Position).text().not_null())
                    .col(
                        ColumnDef::new(DialogueTtsTags::OrderIndex)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(DialogueTtsTags::Source).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dialogue_tts_tags_node")
                            .from(DialogueTtsTags::Table, DialogueTtsTags::DialogueNodeId)
                            .to(DialogueNodes::Table, DialogueNodes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dialogue_tts_tags_node")
                    .table(DialogueTtsTags::Table)
                    .col(DialogueTtsTags::DialogueNodeId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DialogueTtsTags::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DialogueNodes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RawImports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RawImports {
    Table,
    Id,
    ProjectId,
    SceneId,
    SourceType,
    SourceFilePath,
    OriginalText,
    ProcessedJson,
    Status,
    ErrorMessage,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum DialogueNodes {
    Table,
    Id,
    SceneId,
    CharacterId,
    PreviousId,
    NextId,
    OrderIndex,
    Type,
    Text,
    RawText,
    Emotion,
    Intensity,
    IsEnabled,
    BeforeDelayMs,
    AfterDelayMs,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum DialogueTtsTags {
    Table,
    Id,
    DialogueNodeId,
    Tag,
    Position,
    OrderIndex,
    Source,
}
