pub use sea_orm_migration::prelude::*;

mod m20260515_000001_create_projects;
mod m20260515_000002_create_scenes;
mod m20260515_000003_create_characters;
mod m20260515_000004_create_dialogue_nodes;
mod m20260515_000005_create_audio_tables;
mod m20260515_000006_create_timeline_tables;
mod m20260515_000007_create_render_jobs;
mod m20260515_000008_create_app_settings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260515_000001_create_projects::Migration),
            Box::new(m20260515_000002_create_scenes::Migration),
            Box::new(m20260515_000003_create_characters::Migration),
            Box::new(m20260515_000004_create_dialogue_nodes::Migration),
            Box::new(m20260515_000005_create_audio_tables::Migration),
            Box::new(m20260515_000006_create_timeline_tables::Migration),
            Box::new(m20260515_000007_create_render_jobs::Migration),
            Box::new(m20260515_000008_create_app_settings::Migration),
        ]
    }
}
