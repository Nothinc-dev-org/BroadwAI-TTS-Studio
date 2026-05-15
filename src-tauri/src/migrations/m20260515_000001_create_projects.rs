use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Projects::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Projects::Title).text().not_null())
                    .col(ColumnDef::new(Projects::Description).text())
                    .col(
                        ColumnDef::new(Projects::Language)
                            .text()
                            .not_null()
                            .default("es-MX"),
                    )
                    .col(ColumnDef::new(Projects::RootPath).text().not_null())
                    .col(ColumnDef::new(Projects::CreatedAt).text().not_null())
                    .col(ColumnDef::new(Projects::UpdatedAt).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Projects {
    Table,
    Id,
    Title,
    Description,
    Language,
    RootPath,
    CreatedAt,
    UpdatedAt,
}
