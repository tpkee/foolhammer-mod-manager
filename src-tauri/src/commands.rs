use crate::{AppState, mods, state::user_settings::SettingKey};

#[tauri::command]
pub fn check_path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn get_mods(state: AppState) -> serde_json::Value {
    let state = state.lock().unwrap();

    let game_id = state
        .user_settings
        .get(&SettingKey::GameId)
        .unwrap()
        .as_str()
        .unwrap();

    let data_mods = &state
        .user_settings
        .get(&SettingKey::ModsPath)
        .and_then(|game_path| {
            let game_mods_path = std::path::PathBuf::from(game_path.as_str().unwrap());
            mods::retrieve_mods(&game_mods_path).ok()
        });

    let workshop_mods = &state
        .user_settings
        .get(&SettingKey::SteamWorkshopPath)
        .and_then(|workshop_path| {
            let workshop_pathbuf = std::path::PathBuf::from(workshop_path.as_str().unwrap());
            mods::retrieve_workshop_mods(&workshop_pathbuf, &game_id).ok()
        });

    let mut mods = vec![];

    if let Some(data_mods) = data_mods {
        mods.extend(data_mods);
    }

    if let Some(workshop_mods) = workshop_mods {
        mods.extend(workshop_mods);
    }

    serde_json::json!(mods)
}
