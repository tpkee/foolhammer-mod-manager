use std::sync::Mutex;

use tauri::Manager;

pub mod commands;
pub mod defaults;
pub mod dto;
pub mod mods;
pub mod state;
pub mod stores;
pub mod utils;

type AppState<'a> = tauri::State<'a, Mutex<state::app_state::State>>;

#[tauri::command]
fn get_state(state: AppState) -> serde_json::Value {
    let state = state.lock().unwrap();
    println!("Getting state: {:?}", state);
    serde_json::json!(&state.user_settings)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(Mutex::new(state::app_state::State::default()))
        .setup(move |app| {
            let app_handle = app.handle();
            let state: tauri::State<'_, Mutex<state::app_state::State>> =
                app.state::<Mutex<state::app_state::State>>();
            let mut locked_state: std::sync::MutexGuard<'_, state::app_state::State> =
                state.lock().unwrap();
            let path = utils::path::generate_store_path(&app_handle, "settings.json");

            let store = tauri_plugin_store::StoreBuilder::new(app_handle, path.as_path())
                .defaults(
                    locked_state
                        .user_settings
                        .iter()
                        .map(|(k, v)| (k.get(), v.clone()))
                        .collect(),
                ) // the user_settings here should be the default one since locked_state is just created... hopefully.
                .auto_save(std::time::Duration::from_millis(500))
                .build()
                .expect("Failed to build store");

            println!("User settings path: {:?}", path.as_path());

            locked_state.set_settings_from_store(app_handle, store.entries());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            commands::check_path_exists,
            commands::create_profile,
            commands::get_supported_games,
            commands::get_game,
            commands::update_profile,
            commands::rename_profile,
            commands::set_default_profile,
            commands::delete_profile,
            commands::start_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
