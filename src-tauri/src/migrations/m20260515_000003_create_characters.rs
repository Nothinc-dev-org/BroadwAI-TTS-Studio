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
                    .table(Characters::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Characters::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Characters::ProjectId).text().not_null())
                    .col(ColumnDef::new(Characters::Name).text().not_null())
                    .col(ColumnDef::new(Characters::Role).text().not_null())
                    .col(ColumnDef::new(Characters::Description).text())
                    .col(ColumnDef::new(Characters::Color).text())
                    .col(ColumnDef::new(Characters::VoiceProvider).text())
                    .col(ColumnDef::new(Characters::VoiceId).text())
                    .col(ColumnDef::new(Characters::DefaultStylePrompt).text())
                    .col(ColumnDef::new(Characters::CreatedAt).text().not_null())
                    .col(ColumnDef::new(Characters::UpdatedAt).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_characters_project")
                            .from(Characters::Table, Characters::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_characters_project")
                    .table(Characters::Table)
                    .col(Characters::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CharacterAliases::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CharacterAliases::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CharacterAliases::CharacterId)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CharacterAliases::Alias).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_character_aliases_character")
                            .from(CharacterAliases::Table, CharacterAliases::CharacterId)
                            .to(Characters::Table, Characters::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_character_aliases_character")
                    .table(CharacterAliases::Table)
                    .col(CharacterAliases::CharacterId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CharacterAliases::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Characters::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Characters {
    Table,
    Id,
    ProjectId,
    Name,
    Role,
    Description,
    Color,
    VoiceProvider,
    VoiceId,
    DefaultStylePrompt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum CharacterAliases {
    Table,
    Id,
    CharacterId,
    Alias,
}
