use tauri::{Manager, async_runtime::Mutex};

pub mod commands;
pub mod defaults;
pub mod dto;
pub mod launchers;
pub mod mods;
pub mod state;
pub mod stores;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        // .manage(Mutex::new())
        .setup(move |app| {
            let default_state = state::app_state::State::default();
            let app_handle = app.handle();

            stores::settings::SettingsStore::new(app_handle)
                .expect("Failed to build settings store");

            app.manage(Mutex::new(default_state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::game::check_path_exists,
            commands::profile::create_profile,
            commands::game::get_supported_games,
            commands::game::get_game,
            commands::profile::update_profile,
            commands::profile::rename_profile,
            commands::profile::set_default_profile,
            commands::profile::delete_profile,
            commands::mods::set_profile_mods,
            commands::mods::add_profile_mods,
            commands::mods::start_game,
            commands::mods::stop_game,
            commands::game::get_saves,
            commands::game::update_game,
            commands::settings::get_user_settings,
            commands::settings::set_default_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
