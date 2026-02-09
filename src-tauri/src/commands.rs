use tauri::Manager;

use crate::{
    dto,
    stores::{self, games::Profile},
};

use crate::{AppState, mods, state::user_settings::SettingKey, utils};

#[tauri::command]
pub fn check_path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn create_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, u16> {
    let default_game = stores::games::GameStore::new(&payload.game_id)
        .ok_or(404)
        .unwrap()
        .to_hashmap()
        .or(Err(500))
        .unwrap();

    let game_conf_path =
        utils::generate_store_path(&app_handle, format!("{}.json", payload.game_id).as_str());

    let store = tauri_plugin_store::StoreBuilder::new(&app_handle, game_conf_path)
        .defaults(default_game)
        .build()
        .or(Err(500))
        .unwrap();

    // if the store is new should we start the watcher? not sure, probs better to handle this logic separately

    let mut profiles = store
        .get("profiles")
        .ok_or(500)
        .unwrap()
        .as_array()
        .ok_or(500)
        .unwrap()
        .to_vec();

    if profiles
        .iter()
        .find(|&p| p.get("name") == Some(&payload.name.clone().into()))
        .is_some()
    {
        return Err(409);
    }

    let profile = serde_json::json!(Profile::from_dto(payload));

    profiles.push(profile.clone());

    store.set("profiles", serde_json::Value::Array(profiles.to_vec()));

    match store.save() {
        Err(e) => {
            eprintln!("Failed to save profile: {:?}", e);
            return Err(500);
        }
        Ok(_) => {}
    }

    Ok(profile)
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
            mods::helpers::retrieve_mods(&game_mods_path).ok()
        });

    let workshop_mods = &state
        .user_settings
        .get(&SettingKey::SteamWorkshopPath)
        .and_then(|workshop_path| {
            let workshop_pathbuf = std::path::PathBuf::from(workshop_path.as_str().unwrap());
            mods::helpers::retrieve_workshop_mods(&workshop_pathbuf, &game_id).ok()
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
