use sea_orm_migration::prelude::*;

use super::m20260515_000002_create_scenes::Scenes;
use super::m20260515_000004_create_dialogue_nodes::DialogueNodes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RenderJobs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RenderJobs::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RenderJobs::SceneId).text())
                    .col(ColumnDef::new(RenderJobs::DialogueNodeId).text())
                    .col(ColumnDef::new(RenderJobs::Type).text().not_null())
                    .col(ColumnDef::new(RenderJobs::Provider).text().not_null())
                    .col(ColumnDef::new(RenderJobs::Model).text())
                    .col(ColumnDef::new(RenderJobs::Status).text().not_null())
                    .col(ColumnDef::new(RenderJobs::InputPayload).text().not_null())
                    .col(ColumnDef::new(RenderJobs::OutputPath).text())
                    .col(ColumnDef::new(RenderJobs::ErrorMessage).text())
                    .col(ColumnDef::new(RenderJobs::CreatedAt).text().not_null())
                    .col(ColumnDef::new(RenderJobs::UpdatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_render_jobs_scene")
                            .from(RenderJobs::Table, RenderJobs::SceneId)
                            .to(Scenes::Table, Scenes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_render_jobs_node")
                            .from(RenderJobs::Table, RenderJobs::DialogueNodeId)
                            .to(DialogueNodes::Table, DialogueNodes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_render_jobs_status")
                    .table(RenderJobs::Table)
                    .col(RenderJobs::Status)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RenderJobs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RenderJobs {
    Table,
    Id,
    SceneId,
    DialogueNodeId,
    Type,
    Provider,
    Model,
    Status,
    InputPayload,
    OutputPath,
    ErrorMessage,
    CreatedAt,
    UpdatedAt,
}
