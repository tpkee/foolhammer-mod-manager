use crate::commands::helpers::{modify_group, modify_groups, modify_profiles};
use crate::dto::groups::GroupRequestDto;
use crate::stores::games::Group;
use crate::supported_games::SupportedGames;
use crate::utils::ErrorCode;

#[tauri::command]
pub async fn create_group(
    app_handle: tauri::AppHandle,
    payload: GroupRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id;

    modify_groups(&app_handle, game_id, |groups| {
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

    modify_group(&app_handle, game_id, group_id, |group| {
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
    modify_groups(&app_handle, game_id, |groups| {
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
    modify_profiles(&app_handle, game_id, |profiles| {
        for profile in profiles.iter_mut() {
            profile.groups.retain(|g| g != &group_id);
        }

        Ok(())
    })
    .await?;

    modify_groups(&app_handle, game_id, |groups| {
        let idx = groups
            .iter()
            .position(|g| g.id == group_id)
            .ok_or(ErrorCode::NotFound)?;

        groups.remove(idx);

        Ok(())
    })
    .await
}
