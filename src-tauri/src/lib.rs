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
            let mut default_state = state::app_state::State::default();

            let store_default = default_state
                .user_settings
                .iter()
                .map(|(k, v)| (k.get(), v.clone()))
                .collect();

            let app_handle = app.handle();

            let path = utils::path::generate_store_path(&app_handle, "settings.json");

            let store = tauri_plugin_store::StoreBuilder::new(app_handle, path.as_path())
                .defaults(store_default)
                .auto_save(std::time::Duration::from_millis(500))
                .build()
                .expect("Failed to build store");

            default_state.set_settings_from_store(app_handle, store.entries());

            app.manage(Mutex::new(default_state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::check_path_exists,
            commands::create_profile,
            commands::get_supported_games,
            commands::get_game,
            commands::update_profile,
            commands::rename_profile,
            commands::set_default_profile,
            commands::delete_profile,
            commands::start_game,
            commands::stop_game,
            commands::get_saves,
            commands::get_user_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
