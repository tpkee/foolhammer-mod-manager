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
        if groups
            .iter()
            .any(|g| g.name == new_name && g.id != group_id)
        {
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
    GameStore::get(&app_handle, game_id, |game| {
        let idx = game
            .groups
            .iter()
            .position(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        for profile in game.profiles.iter_mut() {
            profile_unlink_group(profile, group_id);
        }

        game.groups.remove(idx);

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
    GameStore::get(&app_handle, game_id, |game| {
        let group = game
            .groups
            .iter_mut()
            .find(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        let old_mods = std::mem::replace(&mut group.mods, mods.clone());
        let added: Vec<String> = mods
            .iter()
            .filter(|m| !old_mods.contains(*m))
            .cloned()
            .collect();
        let removed: Vec<String> = old_mods.into_iter().filter(|m| !mods.contains(m)).collect();

        if added.is_empty() && removed.is_empty() {
            return Ok(serde_json::json!(&group.mods));
        }

        let available_mod_names: HashSet<String> = if !added.is_empty() {
            let workshop_path = retrieve_steam_workshop_path(game_id);
            let available_mods =
                pack::ModPack::retrieve_mods(game.game_id, &game.mods_path, &workshop_path);
            available_mods.into_iter().map(|m| m.name).collect()
        } else {
            HashSet::new()
        };

        for profile in game.profiles.iter_mut() {
            if !profile.groups.contains(&group_id) {
                continue;
            }

            for mod_name in &added {
                if available_mod_names.contains(mod_name) {
                    profile_link_mod(profile, mod_name, group_id);
                }
            }

            profile_unlink_mods(profile, group_id, &removed);
        }

        let group = game
            .groups
            .iter()
            .find(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;
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
    GameStore::get(&app_handle, game_id, |game| {
        let group = game
            .groups
            .iter_mut()
            .find(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        let added: Vec<String> = mods
            .iter()
            .filter(|m| !group.mods.contains(*m))
            .cloned()
            .collect();

        if added.is_empty() {
            return Ok(serde_json::json!(&group.mods));
        }

        let workshop_path = retrieve_steam_workshop_path(game.game_id);
        let available_mods =
            pack::ModPack::retrieve_mods(game.game_id, &game.mods_path, &workshop_path);
        let available_mod_names: HashSet<String> =
            available_mods.into_iter().map(|m| m.name).collect();

        for profile in &mut game.profiles {
            if !profile.groups.contains(&group_id) {
                continue;
            }
            for mod_name in &added {
                if available_mod_names.contains(mod_name) {
                    profile_link_mod(profile, mod_name, group_id);
                }
            }
        }

        // Re-borrow group after profiles mutation
        let group = game.groups.iter_mut().find(|g| g.id == group_id).unwrap();

        group.mods.extend(added);

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
    GameStore::get(&app_handle, game_id, |game| {
        let group = game
            .groups
            .iter_mut()
            .find(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        let removed: Vec<String> = mods
            .iter()
            .filter(|m| group.mods.contains(*m))
            .cloned()
            .collect();

        for profile in &mut game.profiles {
            if !profile.groups.contains(&group_id) {
                continue;
            }
            profile_unlink_mods(profile, group_id, &removed);
        }

        group.mods.retain(|m| !removed.contains(m));

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
        let group = game
            .groups
            .iter()
            .find(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        let group_mods = group.mods.clone();

        let profile = game
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or(ErrorCode::NotFound)?;

        if profile.groups.contains(&group_id) {
            return Ok(serde_json::json!(&*profile));
        }
        profile.groups.push(group_id);

        let workshop_path = retrieve_steam_workshop_path(game.game_id);
        let available_mods =
            pack::ModPack::retrieve_mods(game.game_id, &game.mods_path, &workshop_path);
        let available_mod_names: HashSet<String> =
            available_mods.iter().map(|m| m.name.clone()).collect();

        for mod_name in &group_mods {
            let in_profile = profile.mods.iter().any(|m| &m.name == mod_name);
            if in_profile || available_mod_names.contains(mod_name) {
                profile_link_mod(profile, mod_name, group_id);
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

        profile_unlink_group(profile, group_id);

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
    GameStore::get(&app_handle, game_id, |game| {
        let profile = game
            .profiles
            .iter()
            .find(|p| p.id == profile_id)
            .ok_or(ErrorCode::NotFound)?;

        let old_groups = profile.groups.clone();
        let groups_to_add: Vec<uuid::Uuid> = groups
            .iter()
            .filter(|g| !old_groups.contains(g))
            .copied()
            .collect();
        let groups_to_remove: Vec<uuid::Uuid> = old_groups
            .iter()
            .filter(|g| !groups.contains(g))
            .copied()
            .collect();

        let workshop_path = retrieve_steam_workshop_path(game.game_id);
        let available_mod_names: HashSet<String> = if !groups_to_add.is_empty() {
            let available_mods =
                pack::ModPack::retrieve_mods(game.game_id, &game.mods_path, &workshop_path);
            available_mods.into_iter().map(|m| m.name).collect()
        } else {
            HashSet::new()
        };

        // Link new groups
        for &group_id in &groups_to_add {
            let group_mods: Vec<String> = game
                .groups
                .iter()
                .find(|g| g.id == group_id)
                .ok_or(ErrorCode::NotFound)?
                .mods
                .clone();

            let profile = game
                .profiles
                .iter_mut()
                .find(|p| p.id == profile_id)
                .unwrap();

            if !profile.groups.contains(&group_id) {
                profile.groups.push(group_id);
            }

            for mod_name in &group_mods {
                let in_profile = profile.mods.iter().any(|m| &m.name == mod_name);
                if in_profile || available_mod_names.contains(mod_name) {
                    profile_link_mod(profile, mod_name, group_id);
                }
            }
        }

        // Unlink removed groups
        for &group_id in &groups_to_remove {
            let profile = game
                .profiles
                .iter_mut()
                .find(|p| p.id == profile_id)
                .unwrap();
            profile_unlink_group(profile, group_id);
        }

        let profile = game.profiles.iter().find(|p| p.id == profile_id).unwrap();
        Ok(serde_json::json!(profile))
    })
    .await
}

fn profile_link_mod(profile: &mut Profile, mod_name: &str, group_id: uuid::Uuid) {
    if let Some(existing) = profile.mods.iter_mut().find(|m| m.name == mod_name) {
        let groups = existing.groups.get_or_insert_with(Vec::new);
        if !groups.contains(&group_id) {
            groups.push(group_id);
        }
    } else {
        profile.mods.push(ProfileModInfo {
            name: mod_name.to_owned(),
            enabled: false,
            groups: Some(vec![group_id]),
            order: 0,
        });
    }
}

fn profile_unlink_mods(profile: &mut Profile, group_id: uuid::Uuid, mod_names: &[String]) {
    profile.mods.retain_mut(|m| {
        if mod_names.contains(&m.name)
            && let Some(groups) = &mut m.groups
        {
            groups.retain(|g| g != &group_id);
            if groups.is_empty() {
                m.groups = None;
                return false;
            }
        }

        true
    })
}

fn profile_unlink_group(profile: &mut Profile, group_id: uuid::Uuid) {
    profile.groups.retain(|g| g != &group_id);

    profile.mods.retain_mut(|m| {
        if let Some(groups) = &mut m.groups {
            groups.retain(|g| g != &group_id);

            if groups.is_empty() {
                m.groups = None;
                return false;
            }
        }

        true
    })
}
