use std::path::PathBuf;

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

fn modify_profiles<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut Vec<serde_json::Value>) -> Result<T, ErrorCode>,
{
    let store = GameStore::new(app_handle, game_id)?;

    let mut profiles: Vec<serde_json::Value> = store
        .get("profiles")
        .ok_or(ErrorCode::InternalError)?
        .as_array()
        .ok_or(ErrorCode::InternalError)?
        .to_vec();

    let result = modify_fn(&mut profiles)?;

    store.set("profiles", serde_json::Value::Array(profiles));

    store.save().map_err(|e| {
        eprintln!("Failed to save profiles: {:?}", e);
        ErrorCode::InternalError
    })?;

    Ok(result)
}

#[tauri::command]
pub fn create_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id.clone();

    modify_profiles(&app_handle, &game_id, |profiles| {
        if profiles
            .iter()
            .any(|p| p.get("name") == Some(&payload.name.clone().into()))
        {
            return Err(ErrorCode::Conflict);
        }

        let profile = serde_json::json!(Profile::from_dto(payload));
        profiles.push(profile.clone());

        Ok(profile)
    })
}

#[tauri::command]
pub fn update_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    println!("Updating profile: {:?}", payload);
    let game_id = payload.game_id.clone();

    modify_profiles(&app_handle, &game_id, |profiles| {
        let profile_index = profiles
            .iter()
            .position(|p| p.get("name") == Some(&payload.name.clone().into()))
            .ok_or(ErrorCode::NotFound)?;

        let profile = serde_json::json!(Profile::from_dto(payload));
        profiles[profile_index] = profile.clone();

        Ok(profile)
    })
}

#[tauri::command]
pub fn rename_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    old_name: &str,
    new_name: &str,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        if profiles
            .iter()
            .any(|p| p.get("name") == Some(&new_name.into()))
        {
            return Err(ErrorCode::Conflict);
        }

        let profile_index = profiles
            .iter()
            .position(|p| p.get("name") == Some(&old_name.into()))
            .ok_or(ErrorCode::NotFound)?;

        if let Some(profile_obj) = profiles[profile_index].as_object_mut() {
            profile_obj.insert("name".to_string(), new_name.into());
        } else {
            return Err(ErrorCode::InternalError);
        }

        Ok(profiles[profile_index].clone())
    })
}

#[tauri::command]
pub fn set_default_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
) -> Result<Vec<serde_json::Value>, ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        for profile in profiles.iter_mut() {
            if let Some(profile_obj) = profile.as_object_mut() {
                profile_obj.insert(
                    "default".to_string(),
                    serde_json::Value::Bool(profile_obj.get("name") == Some(&profile_name.into())),
                );
            }
        }

        Ok(profiles.clone())
    })
}

#[tauri::command]
pub fn delete_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
) -> Result<(), ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        let profile_index = profiles
            .iter()
            .position(|p| p.get("name") == Some(&profile_name.into()))
            .ok_or(ErrorCode::NotFound)?;

        profiles.remove(profile_index);

        Ok(())
    })
}

#[tauri::command]
pub fn start_game(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
    save_name: Option<&str>,
) -> Result<(), ErrorCode> {
    let store = GameStore::new(&app_handle, game_id)?;

    let game_store = GameResponseDto::from_store(GameStore::from_entries(store.entries())?);

    let profile = game_store
        .profiles
        .iter()
        .find(|p| p.name == profile_name)
        .ok_or(ErrorCode::NotFound)?;

    utils::game_launcher::launch_game(&game_store, &profile.mods, save_name)?;

    Ok(())
}
