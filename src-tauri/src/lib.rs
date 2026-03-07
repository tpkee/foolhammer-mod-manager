use tauri::{Manager, async_runtime::Mutex};

pub mod commands;
pub mod defaults;
pub mod dto;
pub mod events;
pub mod launchers;
pub mod mods;
pub mod state;
pub mod stores;
pub mod supported_games;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        // .manage(Mutex::new())
        .setup(move |app| {
            let app_handle = app.handle();

            let default_state = state::State::new(app_handle.clone());

            stores::settings::SettingsStore::get_store(app_handle)
                .expect("Failed to build settings store");

            app.manage(Mutex::new(default_state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::games::start_game,
            commands::games::stop_game,
            commands::games::get_saves,
            commands::games::update_game,
            commands::games::check_path_exists,
            commands::games::get_supported_games,
            commands::games::get_game,
            commands::games::set_default_profile,
            commands::profiles::create_profile,
            commands::profiles::update_profile,
            commands::profiles::rename_profile,
            commands::profiles::delete_profile,
            commands::profiles::toggle_manual_mode,
            commands::profiles::set_profile_mods,
            commands::profiles::add_profile_mods,
            commands::profiles::remove_profile_mods,
            commands::settings::get_user_settings,
            commands::settings::set_default_game,
            commands::groups::create_group,
            commands::groups::update_group,
            commands::groups::rename_group,
            commands::groups::delete_group,
            commands::groups::add_group_profile,
            commands::groups::remove_group_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
