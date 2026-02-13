use crate::{
    dto::{self, games::GameResponseDto},
    stores::games::{GameStore, Profile},
};

use utils::ErrorCode;

use crate::utils;

/*
    TODO/notes:
    - I don't like that the keys for the store are basically hardcoded here, they should be tied to an enum.
*/

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

    let game_response = GameResponseDto::from_store(GameStore::from_entries(store.entries())?);

    Ok(serde_json::json!(game_response))
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
pub fn update_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    println!("Updating profile: {:?}", payload);
    let store = GameStore::new(&app_handle, &payload.game_id)?;

    let mut profiles: Vec<serde_json::Value> = store
        .get("profiles")
        .ok_or(ErrorCode::InternalError)?
        .as_array()
        .ok_or(ErrorCode::InternalError)?
        .to_vec();

    let profile_index = profiles
        .iter()
        .position(|p| p.get("name") == Some(&payload.name.clone().into()))
        .ok_or(ErrorCode::NotFound)?;

    let profile = serde_json::json!(Profile::from_dto(payload));

    profiles[profile_index] = profile.clone();

    store.set("profiles", serde_json::Value::Array(profiles));

    match store.save() {
        Err(e) => {
            eprintln!("Failed to update profile: {:?}", e);
            return Err(ErrorCode::InternalError);
        }
        Ok(_) => {}
    }

    Ok(profile)
}
