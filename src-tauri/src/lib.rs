use std::sync::Mutex;

use tauri::Manager;
use tauri_plugin_store::{resolve_store_path, StoreExt};

use crate::state::get_default_state;

pub mod state;
pub mod utils;

#[tauri::command]
fn set_game_folder(state: state::AppState, path: &str) -> String {
    let mut state = state.lock().unwrap();
    let mut folder_watcher = state::FolderWatcher::new(path);

    // folder_watcher.watch();
    state.game_folder = Some(folder_watcher);

    format!("Game folder set to: {}", path)
}

#[tauri::command]
fn get_state(state: state::AppState) -> serde_json::Map<std::string::String, serde_json::Value> {
    return state.lock().unwrap().to_json();
}

#[tauri::command]
fn set_workshop_folder(app: tauri::AppHandle, state: state::AppState, path: &str) -> String {
    tauri::path::PathResolver::app_data_dir(app.path());
    app.path().app_data_dir();
    app.store("settings.json");
    let mut state = state.lock().unwrap();
    let mut folder_watcher = state::FolderWatcher::new(path);
    folder_watcher.watch(|event| {
        println!("Workshop folder event: {:?}", event);
    });
    state.game_workshop_folder = Some(folder_watcher);

    format!("Workshop folder set to: {}", path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(Mutex::new(get_default_state()))
        .setup(move |app| {
            let defaults = get_default_state().to_hashmap();
            let path = app
                .path()
                .config_dir()
                .unwrap()
                .join("modhammer-manager/settings.json");

            let store = tauri_plugin_store::StoreBuilder::new(app, path.as_path())
                .defaults(defaults.clone())
                .auto_save(std::time::Duration::from_millis(500))
                .build()
                .unwrap();

            println!("Store path: {:?}", path.as_path());

            let state = app.state::<Mutex<state::State>>();
            let mut locked_state = state.lock().unwrap();
            *locked_state = state::State::from_store(store.entries());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_game_folder,
            set_workshop_folder,
            get_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
