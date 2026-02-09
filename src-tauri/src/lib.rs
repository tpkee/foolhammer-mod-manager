use std::sync::Mutex;

use tauri::Manager;

pub mod commands;
pub mod defaults;
pub mod dto;
pub mod mods;
pub mod state;
pub mod utils;

type AppState<'a> = tauri::State<'a, Mutex<state::app_state::State>>;

#[tauri::command]
fn set_game_folder(_state: AppState, path: &str) -> String {
    // let mut state = state.lock().unwrap();
    // let mut folder_watcher = state::FolderWatcher::new(path);

    // // folder_watcher.watch();
    // state.game_folder = Some(folder_watcher);

    format!("Game folder set to: {}", path)
}

#[tauri::command]
fn get_state(state: AppState) -> serde_json::Value {
    let state = state.lock().unwrap();
    println!("Getting state: {:?}", state);
    serde_json::json!(&state.user_settings)
}

#[tauri::command]
fn set_workshop_folder(_app: tauri::AppHandle, _state: AppState, path: &str) -> String {
    // tauri::path::PathResolver::app_data_dir(app.path());
    // app.path().app_data_dir();
    // app.store("settings.json");
    // let mut state = state.lock().unwrap();
    // let mut folder_watcher = state::FolderWatcher::new(path);
    // folder_watcher.watch(|event| {
    //     println!("Workshop folder event: {:?}", event);
    // });
    // state.steam_workshop_folder = Some(folder_watcher);

    format!("Workshop folder set to: {}", path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(Mutex::new(state::app_state::State::default()))
        .setup(move |app| {
            let app_handle = app.handle();
            let state: tauri::State<'_, Mutex<state::app_state::State>> =
                app.state::<Mutex<state::app_state::State>>();
            let mut locked_state: std::sync::MutexGuard<'_, state::app_state::State> =
                state.lock().unwrap();
            let path = app // TODO: use the crate variable extractor instead of this
                .path()
                .config_dir()
                .expect("Failed to get config directory")
                .join("foolhamer-mod-manager/settings.json");

            let store = tauri_plugin_store::StoreBuilder::new(app, path.as_path())
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
            set_game_folder,
            set_workshop_folder,
            get_state,
            commands::get_mods,
            commands::check_path_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
