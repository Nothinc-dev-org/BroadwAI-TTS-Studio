use sea_orm_migration::prelude::*;

use super::m20260515_000001_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Scenes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Scenes::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Scenes::ProjectId).text().not_null())
                    .col(ColumnDef::new(Scenes::Title).text().not_null())
                    .col(ColumnDef::new(Scenes::Description).text())
                    .col(
                        ColumnDef::new(Scenes::OrderIndex)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Scenes::CreatedAt).text().not_null())
                    .col(ColumnDef::new(Scenes::UpdatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_scenes_project")
                            .from(Scenes::Table, Scenes::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_scenes_project")
                    .table(Scenes::Table)
                    .col(Scenes::ProjectId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Scenes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Scenes {
    Table,
    Id,
    ProjectId,
    Title,
    Description,
    OrderIndex,
    CreatedAt,
    UpdatedAt,
}
