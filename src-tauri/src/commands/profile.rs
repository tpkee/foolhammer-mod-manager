use crate::commands::helpers::{modify_profile, modify_profiles};
use crate::dto::profiles::ProfileRequestDto;
use crate::stores::games::Profile;
use crate::utils::ErrorCode;

#[tauri::command]
pub fn create_profile(
    app_handle: tauri::AppHandle,
    payload: ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id.clone();

    modify_profiles(&app_handle, &game_id, |profiles| {
        if profiles.iter().any(|p| p.name == payload.name) {
            return Err(ErrorCode::Conflict);
        }

        let profile: Profile = Profile::from_dto(None, payload);
        profiles.push(profile.clone());

        Ok(serde_json::json!(profile))
    })
}

#[tauri::command]
pub fn update_profile(
    app_handle: tauri::AppHandle,
    profile_id: uuid::Uuid,
    payload: ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id.clone();

    modify_profile(&app_handle, &game_id, profile_id, |profile| {
        if profile.name != payload.name {
            return Err(ErrorCode::Conflict);
        }

        *profile = Profile::from_dto(Some(profile_id), payload);

        Ok(serde_json::json!(profile))
    })
}

#[tauri::command]
pub fn rename_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_id: uuid::Uuid,
    new_name: &str,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        if profiles.iter().any(|p| p.name == new_name) {
            return Err(ErrorCode::Conflict);
        }

        let profile = profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or(ErrorCode::NotFound)?;

        profile.name = new_name.to_string();

        Ok(serde_json::json!(&*profile))
    })
}

#[tauri::command]
pub fn delete_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_id: uuid::Uuid,
) -> Result<(), ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        let idx = profiles
            .iter()
            .position(|p| p.id == profile_id)
            .ok_or(ErrorCode::NotFound)?;

        profiles.remove(idx);

        Ok(())
    })
}

#[tauri::command]
pub fn toggle_manual_mode(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_id: uuid::Uuid,
) -> Result<(), ErrorCode> {
    modify_profile(&app_handle, game_id, profile_id, |profile| {
        profile.manual_mode = !profile.manual_mode;

        Ok(())
    })
}
