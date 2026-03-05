use crate::commands::helpers::{get_game_response_from_store, modify_game};
use crate::defaults::games::{DefaultGameInfo, SUPPORTED_GAMES};
use crate::dto::games::{GameRequestDto, GameResponseDto};
use crate::state::AppState;
use crate::utils::ErrorCode;
use std::path::PathBuf;

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
// getting a game will also start its watchers.
pub async fn get_game(
    app_handle: tauri::AppHandle,
    app_state: AppState<'_>,
    game_id: &str,
) -> Result<serde_json::Value, ErrorCode> {
    let game_response = get_game_response_from_store(&app_handle, game_id)?;

    start_game_watchers(
        app_state,
        game_response.mods_path.clone(),
        &game_response.workshop_path,
        &game_response.saves_path,
    )
    .await;

    Ok(serde_json::json!(game_response))
}

#[tauri::command]
pub async fn update_game(
    app_handle: tauri::AppHandle,
    app_state: AppState<'_>,
    game_id: &str,
    payload: GameRequestDto,
) -> Result<(), ErrorCode> {
    let g = modify_game(&app_handle, game_id, |game| {
        game.saves_path = payload.saves_path;
        game.mods_path = payload.mods_path;
        game.game_path = payload.game_path;

        Ok(GameResponseDto::from_store(game.clone()))
    })?;

    start_game_watchers(app_state, g.mods_path, &g.workshop_path, &g.saves_path).await;

    Ok(())
}

async fn start_game_watchers(
    app_state: AppState<'_>,
    mods_folder: PathBuf,
    workshop_folder: &Option<PathBuf>,
    saves_folder: &Option<PathBuf>,
) {
    let mut folders = vec![mods_folder];

    if let Some(saves) = saves_folder {
        folders.push(saves.clone());
    }

    if let Some(workshop) = workshop_folder {
        folders.push(workshop.clone());
    }

    let mut state = app_state.lock().await;

    state.folder_watcher.set_watchers(&folders);
}
