use crate::{
    dto::groups::GroupRequestDto,
    mods::pack,
    stores::games::{GameStore, Group, Profile, ProfileModInfo, Store},
    supported_games::SupportedGames,
    utils::{ErrorCode, path::retrieve_steam_workshop_path},
};
use std::collections::HashSet;

#[tauri::command]
pub async fn create_group(
    app_handle: tauri::AppHandle,
    payload: GroupRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id;

    Group::get_all(&app_handle, game_id, |groups| {
        if groups.iter().any(|g| g.name == payload.name) {
            return Err(ErrorCode::Conflict);
        }

        let group: Group = Group::from(payload);
        groups.push(group.clone());

        Ok(serde_json::json!(group))
    })
    .await
}

#[tauri::command]
pub async fn update_group(
    app_handle: tauri::AppHandle,
    group_id: uuid::Uuid,
    mut payload: GroupRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id;

    Group::get(&app_handle, game_id, group_id, |group| {
        payload.id = Some(group_id);

        *group = Group::from(payload);

        Ok(serde_json::json!(group))
    })
    .await
}

#[tauri::command]
pub async fn rename_group(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    group_id: uuid::Uuid,
    new_name: &str,
) -> Result<serde_json::Value, ErrorCode> {
    Group::get_all(&app_handle, game_id, |groups| {
        if groups.iter().any(|g| g.name == new_name) {
            return Err(ErrorCode::Conflict);
        }

        let group = groups
            .iter_mut()
            .find(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        group.name = new_name.to_string();

        Ok(serde_json::json!(&*group))
    })
    .await
}

#[tauri::command]
pub async fn delete_group(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    group_id: uuid::Uuid,
) -> Result<(), ErrorCode> {
    // TODO: if B fails we should revert the changes made by A,
    // but this is a bit tricky to implement so for now
    // yes, I still regret not using sql thank you very much
    Profile::get_all(&app_handle, game_id, |profiles| {
        for profile in profiles.iter_mut() {
            profile.groups.retain(|g| g != &group_id);
        }

        Ok(())
    })
    .await?;

    Group::get_all(&app_handle, game_id, |groups| {
        let idx = groups
            .iter()
            .position(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        groups.remove(idx);

        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn set_group_mods(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    group_id: uuid::Uuid,
    mods: Vec<String>,
) -> Result<serde_json::Value, ErrorCode> {
    Group::get(&app_handle, game_id, group_id, |group| {
        group.mods = mods;
        Ok(serde_json::json!(&group.mods))
    })
    .await
}

#[tauri::command]
pub async fn add_group_mods(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    group_id: uuid::Uuid,
    mods: Vec<String>,
) -> Result<serde_json::Value, ErrorCode> {
    Group::get(&app_handle, game_id, group_id, |group| {
        group.mods.extend(mods);
        Ok(serde_json::json!(&group.mods))
    })
    .await
}

#[tauri::command]
pub async fn remove_group_mods(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    group_id: uuid::Uuid,
    mods: Vec<String>,
) -> Result<serde_json::Value, ErrorCode> {
    Group::get(&app_handle, game_id, group_id, |group| {
        group.mods.retain(|m| !mods.contains(m));
        Ok(serde_json::json!(&group.mods))
    })
    .await
}

#[tauri::command]
pub async fn add_group_profile(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    group_id: uuid::Uuid,
    profile_id: uuid::Uuid,
) -> Result<serde_json::Value, ErrorCode> {
    GameStore::get(&app_handle, game_id, |game| {
        let profile = game // we need the mut reference
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or(ErrorCode::NotFound)?;

        let group = Group::find_by_id(&app_handle, game_id, group_id).ok_or(ErrorCode::NotFound)?;

        if profile.groups.contains(&group_id) {
            return Ok(serde_json::json!(&*profile));
        } else {
            profile.groups.push(group_id);
        }

        let workshop_path = retrieve_steam_workshop_path(game.game_id);
        let available_mods = pack::ModPack::retrieve_mods(&game.mods_path, &workshop_path);

        let available_mod_names: HashSet<String> =
            HashSet::from_iter(available_mods.iter().map(|m| m.name.clone()));

        for mod_name in &group.mods {
            if let Some(existing_mod) = profile.mods.iter_mut().find(|m| &m.name == mod_name) {
                if !existing_mod.groups.contains(&group_id) {
                    existing_mod.groups.push(group_id);
                }
            } else if available_mod_names.contains(mod_name) {
                profile.mods.push(ProfileModInfo {
                    name: mod_name.clone(),
                    enabled: false,
                    groups: vec![group_id],
                    order: 0,
                });
            }
        }

        Ok(serde_json::json!(&*profile))
    })
    .await
}

#[tauri::command]
pub async fn remove_group_profile(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    group_id: uuid::Uuid,
    profile_id: uuid::Uuid,
) -> Result<serde_json::Value, ErrorCode> {
    Profile::get(&app_handle, game_id, profile_id, |profile| {
        if !profile.groups.contains(&group_id) {
            return Ok(serde_json::json!(&*profile));
        }

        profile.mods.retain_mut(|m| {
            m.groups.retain(|g| g != &group_id);

            if m.groups.is_empty() {
                return false;
            }

            true
        });

        profile.groups.retain(|g| g != &group_id);

        Ok(serde_json::json!(&*profile))
    })
    .await
}

#[tauri::command]
pub async fn set_groups_profile(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
    groups: Vec<uuid::Uuid>,
) -> Result<serde_json::Value, ErrorCode> {
    let profile =
        Profile::find_by_id(&app_handle, game_id, profile_id).ok_or(ErrorCode::NotFound)?;

    let old_groups = &profile.groups;

    for group_id in &groups {
        if !old_groups.contains(group_id) {
            add_group_profile(app_handle.clone(), game_id, *group_id, profile_id).await?;
        }
    }

    for group_id in old_groups {
        if !groups.contains(group_id) {
            remove_group_profile(app_handle.clone(), game_id, *group_id, profile_id).await?;
        }
    }

    Ok(serde_json::json!(profile))
}
