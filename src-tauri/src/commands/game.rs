use std::path::PathBuf;

use crate::commands::helpers::get_game_response_from_store;
use crate::defaults::games::{DefaultGameInfo, SUPPORTED_GAMES};
use crate::utils::ErrorCode;

#[tauri::command]
pub fn check_path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn get_supported_games() -> serde_json::Value {
    SUPPORTED_GAMES.map(|game| game.game_id).into()
}

#[tauri::command]
pub fn get_saves(game_id: &str) -> Result<Vec<String>, ErrorCode> {
    let game_info = DefaultGameInfo::find_by_id(game_id).ok_or(ErrorCode::NotFound)?;

    let saves_path = PathBuf::from(&game_info.saves_path);

    if !saves_path.exists() {
        return Err(ErrorCode::NotFound);
    }

    let entries = std::fs::read_dir(saves_path)
        .map_err(|e| {
            eprintln!("Failed to read saves directory: {:?}", e);
            ErrorCode::InternalError
        })?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_file() {
                    e.file_name().into_string().ok()
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(entries)
}

#[tauri::command]
pub fn get_game(
    app_handle: tauri::AppHandle,
    game_id: &str,
) -> Result<serde_json::Value, ErrorCode> {
    let game_response = get_game_response_from_store(&app_handle, game_id)?;

    Ok(serde_json::json!(game_response))
}
