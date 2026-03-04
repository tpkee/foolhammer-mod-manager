use crate::dto::settings::UserSettingsResponseDto;
use crate::state::{app_state::AppState, user_settings::SettingKey};
use crate::utils::{ErrorCode, path::generate_store_path};
use tauri::Emitter;

#[tauri::command]
pub async fn get_user_settings<'a>(
    state: AppState<'a>,
) -> Result<UserSettingsResponseDto, ErrorCode> {
    let state = state.lock().await;
    Ok(UserSettingsResponseDto::from(&state.user_settings))
}

#[tauri::command]
pub async fn set_default_game<'a>(
    app_handle: tauri::AppHandle,
    state: AppState<'a>,
    game_id: &str,
) -> Result<(), ErrorCode> {
    let store_path = generate_store_path(&app_handle, "settings.json");

    let store = tauri_plugin_store::StoreBuilder::new(&app_handle, store_path)
        .build()
        .map_err(|e| {
            eprintln!("Failed to open settings store: {:?}", e);
            ErrorCode::InternalError
        })?;

    store.set(SettingKey::GameId.get(), serde_json::json!(game_id));
    store.save().map_err(|e| {
        eprintln!("Failed to save settings store: {:?}", e);
        ErrorCode::InternalError
    })?;

    let mut state = state.lock().await;
    state
        .user_settings
        .insert(SettingKey::GameId, serde_json::json!(game_id));

    app_handle
        .emit("update_user_settings", &state.user_settings)
        .expect("Failed to emit update_user_settings event");

    Ok(())
}
