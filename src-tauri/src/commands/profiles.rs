use crate::{
    commands::helpers::{modify_profile, modify_profiles},
    dto::{mods::ModRequestDto, profiles::ProfileRequestDto},
    stores::games::{Profile, ProfileModInfo},
    supported_games::SupportedGames,
    utils::ErrorCode,
};

#[tauri::command]
pub async fn create_profile(
    app_handle: tauri::AppHandle,
    payload: ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id;

    modify_profiles(&app_handle, game_id, |profiles| {
        if profiles.iter().any(|p| p.name == payload.name) {
            return Err(ErrorCode::Conflict);
        }

        let profile: Profile = Profile::from(payload);
        profiles.push(profile.clone());

        Ok(serde_json::json!(profile))
    })
    .await
}

#[tauri::command]
pub async fn update_profile(
    app_handle: tauri::AppHandle,
    profile_id: uuid::Uuid,
    mut payload: ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id;

    modify_profile(&app_handle, game_id, profile_id, |profile| {
        payload.id = Some(profile_id);

        *profile = Profile::from(payload);

        Ok(serde_json::json!(profile))
    })
    .await
}

#[tauri::command]
pub async fn rename_profile(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
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
    .await
}

#[tauri::command]
pub async fn delete_profile(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
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
    .await
}

#[tauri::command]
pub async fn toggle_manual_mode(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
) -> Result<(), ErrorCode> {
    modify_profile(&app_handle, game_id, profile_id, |profile| {
        profile.manual_mode = !profile.manual_mode;

        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn set_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
    mods: Vec<ModRequestDto>,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profile(&app_handle, game_id, profile_id, |profile| {
        profile.mods = mods.into_iter().map(ProfileModInfo::from).collect();
        Ok(serde_json::json!(&profile.mods))
    })
    .await
}

#[tauri::command]
pub async fn add_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
    mods: Vec<ModRequestDto>,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profile(&app_handle, game_id, profile_id, |profile| {
        if profile.manual_mode {
            let old_len = profile.mods.len();
            let new_mods: Vec<ProfileModInfo> = mods
                .into_iter()
                .enumerate()
                .map(|(i, m)| ProfileModInfo {
                    name: m.name,
                    enabled: m.enabled,
                    order: m.order.unwrap_or(u32::try_from(old_len + i).unwrap_or(0)),
                })
                .collect();

            profile.mods.extend(new_mods);
        } else {
            profile
                .mods
                .extend(mods.into_iter().map(ProfileModInfo::from));
        }
        Ok(serde_json::json!(&profile.mods))
    })
    .await
}

#[tauri::command]
pub async fn remove_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
    mods: Vec<String>,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profile(&app_handle, game_id, profile_id, |profile| {
        profile.mods.retain(|m| !mods.contains(&m.name));
        Ok(serde_json::json!(&profile.mods))
    })
    .await
}
