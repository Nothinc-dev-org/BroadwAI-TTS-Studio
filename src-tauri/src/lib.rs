pub mod commands;
pub mod db;
pub mod entities;
pub mod error;
pub mod migrations;
pub mod paths;
pub mod services;
pub mod state;

use tauri::Manager;
use tracing_subscriber::EnvFilter;

use crate::state::AppState;

pub fn run() {
    init_tracing();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let state = tauri::async_runtime::block_on(async move {
                AppState::initialize(&handle)
                    .await
                    .expect("Failed to initialize AppState")
            });
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Projects
            commands::project::create_project,
            commands::project::open_project,
            commands::project::list_recent_projects,
            commands::project::update_project,
            commands::project::delete_project,
            commands::project::export_project,
            commands::project::import_project,
            // Scenes
            commands::scene::create_scene,
            commands::scene::get_scene,
            commands::scene::list_scenes,
            commands::scene::update_scene,
            commands::scene::delete_scene,
            commands::scene::reorder_scenes,
            // Characters
            commands::character::create_character,
            commands::character::list_characters,
            commands::character::update_character,
            commands::character::delete_character,
            commands::character::add_character_alias,
            commands::character::remove_character_alias,
            commands::character::assign_character_voice,
            // Dialogues
            commands::dialogue::create_dialogue_node,
            commands::dialogue::list_dialogue_nodes,
            commands::dialogue::update_dialogue_node,
            commands::dialogue::delete_dialogue_node,
            commands::dialogue::split_dialogue_node,
            commands::dialogue::merge_dialogue_nodes,
            commands::dialogue::reorder_dialogue_nodes,
            commands::dialogue::update_dialogue_tts_tags,
            // Import
            commands::import::import_text,
            commands::import::import_file,
            commands::import::process_import_with_deepseek,
            commands::import::validate_import_result,
            commands::import::create_scene_from_import,
            // TTS
            commands::tts::generate_dialogue_audio,
            commands::tts::generate_scene_audio,
            commands::tts::regenerate_outdated_audio,
            commands::tts::play_dialogue_audio,
            commands::tts::play_scene_audio,
            commands::tts::list_generated_audio_for_scene,
            commands::tts::generated_audio_bytes,
            commands::tts::preview_voice_sample,
            commands::tts::preview_voice_sample_bytes,
            commands::tts::optimize_scene_tts_tags,
            // Timeline
            commands::timeline::create_timeline_track,
            commands::timeline::list_timeline_tracks,
            commands::timeline::update_timeline_track,
            commands::timeline::delete_timeline_track,
            commands::timeline::list_timeline_events,
            commands::timeline::create_timeline_event,
            commands::timeline::update_timeline_event,
            commands::timeline::delete_timeline_event,
            commands::timeline::render_timeline,
            // Assets
            commands::assets::import_audio_asset,
            commands::assets::list_audio_assets,
            commands::assets::update_audio_asset,
            commands::assets::delete_audio_asset,
            commands::assets::preview_audio_asset,
            // Settings / credentials
            commands::settings::set_api_key,
            commands::settings::delete_api_key,
            commands::settings::test_api_key,
            commands::settings::get_api_key_status,
            commands::settings::get_app_settings,
            commands::settings::update_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,broadwai_lib=debug"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}
