use crate::{
    commands::helpers::get_game_response_from_store,
    defaults::games::{DefaultGameInfo, SUPPORTED_GAMES},
    dto::games::{GameRequestDto, GameResponseDto},
    join_path,
    launchers::{self, GameManager},
    mods,
    state::AppState,
    stores::games::{GameStore, Profile, Store},
    supported_games::SupportedGames,
    utils::ErrorCode,
};
use std::collections::HashMap;
use std::path::PathBuf;

#[tauri::command]
pub fn check_path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn get_supported_games() -> serde_json::Value {
    serde_json::json!(SUPPORTED_GAMES.map(|game| game.game_id))
}

#[tauri::command]
pub fn get_saves(game_id: SupportedGames) -> Result<Vec<String>, ErrorCode> {
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
pub async fn stop_game<'a>(state: AppState<'a>) -> Result<(), ErrorCode> {
    let mut local_state = state.lock().await;
    local_state.game_runner.take().unwrap().kill_game().unwrap();

    Ok(())
}

#[tauri::command]
pub async fn start_game<'a>(
    app_handler: tauri::AppHandle,
    state: AppState<'a>,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
    save_name: Option<&str>,
) -> Result<(), ErrorCode> {
    let game_store = get_game_response_from_store(&app_handler, game_id)?;

    let profile = game_store
        .profiles
        .iter()
        .find(|p| p.id == profile_id)
        .ok_or(ErrorCode::NotFound)?;

    let GameResponseDto {
        game_path,
        saves_path,
        mods_path,
        workshop_path,
        game_id,
        ..
    } = game_store;

    let savegame_path = save_name
        .zip(saves_path.as_ref())
        .map(|(name, saves)| saves.join(name))
        .filter(|path| {
            if !path.exists() {
                eprintln!(
                    "Save game '{}' not found in saves directory. Ignoring save name.",
                    path.display()
                );
            }
            path.exists()
        });

    let txt_path = join_path!(&game_path, "used_mods.txt");

    if !game_path.exists() {
        return Err(ErrorCode::InternalError);
    }

    let mod_writer =
        mods::writer::ModWriter::new(game_id, &profile.mods, &mods_path, &workshop_path);

    mod_writer
        .write(txt_path)
        .expect("It wasn't possible to write the mod file");

    let mut runner = launchers::GameLauncher::create(&app_handler).await;

    let _ = runner.launch_game(game_id, &game_path, savegame_path.as_ref());

    let mut state = state.lock().await;

    state.game_runner = Some(Box::new(runner));

    Ok(())
}

#[tauri::command]
// getting a game will also start its watchers.
// TODO: heavy, too heavy... gotta trim it down or optimise it somehow. it's not the watchers
pub async fn get_game(
    app_handle: tauri::AppHandle,
    app_state: AppState<'_>,
    game_id: SupportedGames,
) -> Result<serde_json::Value, ErrorCode> {
    let game_response = get_game_response_from_store(&app_handle, game_id)?;

    if let Some(profile_id) = game_response.default_profile {
        let response_mods: HashMap<&String, u32> = game_response
            .profiles
            .iter()
            .find(|p| p.id == profile_id)
            .ok_or(ErrorCode::NotFound)?
            .mods
            .iter()
            .map(|m| (&m.name, m.order))
            .collect();

        Profile::get(&app_handle, game_id, profile_id, |profile| {
            if !profile.manual_mode {
                for profile_mod in &mut profile.mods {
                    if let Some(order) = response_mods.get(&profile_mod.name) {
                        profile_mod.order = *order;
                    }
                }
            }

            Ok(())
        })
        .await?;
    }

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
    game_id: SupportedGames,
    payload: GameRequestDto,
) -> Result<(), ErrorCode> {
    let g = GameStore::get(&app_handle, game_id, |game| {
        game.saves_path = payload.saves_path;
        game.mods_path = payload.mods_path;
        game.game_path = payload.game_path;

        Ok(GameResponseDto::from_store(game.clone()))
    })
    .await?;

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

#[tauri::command]
pub async fn set_default_profile(
    app_handle: tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
) -> Result<(), ErrorCode> {
    GameStore::get(&app_handle, game_id, |game| {
        if game.default_profile == Some(profile_id) {
            return Ok(());
        }

        if !game.profiles.iter().any(|p| p.id == profile_id) {
            return Err(ErrorCode::NotFound);
        }

        game.default_profile = Some(profile_id);

        Ok(())
    })
    .await
}
