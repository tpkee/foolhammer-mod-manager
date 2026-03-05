use crate::dto::settings::UserSettingsResponseDto;
use crate::stores::settings::{SettingsKey, SettingsStore};
use crate::utils::ErrorCode;
use tauri::Emitter;

#[tauri::command]
pub async fn get_user_settings(
    app_handle: tauri::AppHandle,
) -> Result<UserSettingsResponseDto, ErrorCode> {
    let store = SettingsStore::get_store(&app_handle)?;
    let settings = SettingsStore::from_entries(store.entries())?;
    Ok(UserSettingsResponseDto::from(&settings))
}

#[tauri::command]
pub async fn set_default_game(
    app_handle: tauri::AppHandle,
    game_id: &str,
) -> Result<(), ErrorCode> {
    let store = SettingsStore::get_store(&app_handle)?;

    store.set(SettingsKey::DefaultGame, serde_json::json!(game_id));
    store.save().map_err(|e| {
        eprintln!("Failed to save settings store: {:?}", e);
        ErrorCode::InternalError
    })?;

    app_handle
        .emit("update_user_settings", ())
        .expect("Failed to emit update_user_settings event");

    Ok(())
}
