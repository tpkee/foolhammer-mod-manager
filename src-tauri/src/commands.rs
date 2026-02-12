use std::collections::HashMap;

use crate::{
    dto,
    stores::games::{GameStore, Profile},
};

use utils::ErrorCode;

use crate::{AppState, mods, state::user_settings::SettingKey, utils};

#[tauri::command]
pub fn check_path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn get_supported_games() -> serde_json::Value {
    crate::defaults::games::SUPPORTED_GAMES
        .map(|game| game.game_id)
        .into()
}

#[tauri::command]
pub fn get_game(
    app_handle: tauri::AppHandle,
    game_id: &str,
) -> Result<serde_json::Value, ErrorCode> {
    let store = GameStore::new(&app_handle, game_id)?;
    let map: HashMap<String, serde_json::Value> = store.entries().into_iter().collect();

    Ok(serde_json::json!(map))
}

#[tauri::command]
pub fn create_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let store = GameStore::new(&app_handle, &payload.game_id)?;

    let mut profiles: Vec<serde_json::Value> = store
        .get("profiles")
        .ok_or(ErrorCode::InternalError)?
        .as_array()
        .ok_or(ErrorCode::InternalError)?
        .to_vec();

    if profiles
        .iter()
        .find(|&p| p.get("name") == Some(&payload.name.clone().into()))
        .is_some()
    {
        return Err(ErrorCode::Conflict);
    }

    let profile = serde_json::json!(Profile::from_dto(payload));

    profiles.push(profile.clone());

    store.set("profiles", serde_json::Value::Array(profiles.to_vec()));

    match store.save() {
        Err(e) => {
            eprintln!("Failed to save profile: {:?}", e);
            return Err(ErrorCode::InternalError);
        }
        Ok(_) => {}
    }

    Ok(profile)
}

#[tauri::command]
pub fn get_mods(state: AppState) -> Result<serde_json::Value, ErrorCode> {
    let state = state.lock().unwrap();

    let game_id = state
        .user_settings
        .get(&SettingKey::GameId)
        .ok_or(ErrorCode::InternalError)?
        .as_str()
        .ok_or(ErrorCode::InternalError)?;

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

    Ok(serde_json::json!(mods))
}
