use crate::dto::settings::{UpdateUserSettingsDto, UserSettingsResponseDto};
use crate::events::AppEvent;
use crate::stores::settings::{SettingsKey, SettingsStore};
use crate::supported_games::SupportedGames;
use crate::utils::ErrorCode;
use tauri::Emitter;
use tauri::Manager;

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
    game_id: SupportedGames,
) -> Result<(), ErrorCode> {
    let store = SettingsStore::get_store(&app_handle)?;

    store.set(SettingsKey::DefaultGame, serde_json::json!(game_id));
    store.save().map_err(|e| {
        log::error!("Failed to save settings store: {:?}", e);
        ErrorCode::InternalError
    })?;

    app_handle
        .emit(AppEvent::UpdateUserSettings.into(), ())
        .expect("Failed to emit update_user_settings event");

    Ok(())
}

#[tauri::command]
pub async fn set_invert_mod_names(
    app_handle: tauri::AppHandle,
    invert: bool,
) -> Result<(), ErrorCode> {
    let store = SettingsStore::get_store(&app_handle)?;

    store.set(SettingsKey::InvertModNames, serde_json::json!(invert));
    store.save().map_err(|e| {
        log::error!("Failed to save settings store: {:?}", e);
        ErrorCode::InternalError
    })?;

    app_handle
        .emit(AppEvent::UpdateUserSettings.into(), ())
        .expect("Failed to emit update_user_settings event");

    Ok(())
}

#[tauri::command]
pub async fn update_settings(
    app_handle: tauri::AppHandle,
    payload: UpdateUserSettingsDto,
) -> Result<(), ErrorCode> {
    log::info!(
        "update_settings: steam_path={:?}, steam_library_path={:?}",
        payload.steam_path,
        payload.steam_library_path
    );

    let store = SettingsStore::get_store(&app_handle)?;

    store.set(
        SettingsKey::SteamPath,
        serde_json::to_value(payload.steam_path).map_err(|_| ErrorCode::InternalError)?,
    );
    store.set(
        SettingsKey::SteamLibraryPath,
        serde_json::to_value(payload.steam_library_path).map_err(|_| ErrorCode::InternalError)?,
    );
    store.save().map_err(|e| {
        log::error!("Failed to save settings store: {:?}", e);
        ErrorCode::InternalError
    })?;

    app_handle
        .emit(AppEvent::UpdateUserSettings.into(), ())
        .expect("Failed to emit update_user_settings event");

    Ok(())
}

#[tauri::command]
pub fn get_log_directory(app: tauri::AppHandle) -> Result<String, String> {
    app.path()
        .app_log_dir()
        .map(|p| p.display().to_string())
        .map_err(|e| e.to_string())
}
