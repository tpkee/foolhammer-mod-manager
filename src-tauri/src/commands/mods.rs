use crate::{
    commands::helpers::modify_profile, dto::mods::ModRequestDto, stores::games::ModInfo,
    supported_games::SupportedGames, utils::ErrorCode,
};

#[tauri::command]
pub async fn set_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
    mods: Vec<ModRequestDto>,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profile(&app_handle, game_id, profile_id, |profile| {
        profile.mods = mods.into_iter().map(ModInfo::from).collect();
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
            let new_mods: Vec<ModInfo> = mods
                .into_iter()
                .enumerate()
                .map(|(i, m)| ModInfo {
                    name: m.name,
                    enabled: m.enabled,
                    order: m.order.unwrap_or(u32::try_from(old_len + i).unwrap_or(0)),
                })
                .collect();

            profile.mods.extend(new_mods);
        } else {
            profile.mods.extend(mods.into_iter().map(ModInfo::from));
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
