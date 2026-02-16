use std::{collections::HashMap, path::PathBuf};

use crate::{
    defaults::games::{DefaultGameInfo, SUPPORTED_GAMES},
    dto::{self, games::GameResponseDto},
    join_path,
    launchers::{GameManager, linux::LinuxLauncher},
    mods,
    state::app_state::AppState,
    stores::games::{GameStore, Profile},
};
use utils::ErrorCode;

use crate::utils;

/*
    TODO/notes:
    - I don't like that the keys for the store are basically hardcoded here, they should be tied to an enum. Refactor this into the aforementioned enum and add a wrapper method for retrieval
*/

fn get_game_response_from_store(
    app_handler: &tauri::AppHandle,
    game_id: &str,
) -> Result<GameResponseDto, ErrorCode> {
    let store = GameStore::new(app_handler, game_id)?;

    let game_store = GameResponseDto::from_store(GameStore::from_entries(store.entries())?);

    Ok(game_store)
}

#[tauri::command]
pub fn check_path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn get_supported_games() -> serde_json::Value {
    SUPPORTED_GAMES.map(|game| game.game_id).into()
}

#[tauri::command]
pub async fn get_user_settings<'a>(state: AppState<'a>) -> Result<serde_json::Value, ErrorCode> {
    let state = state.lock().await;
    Ok(serde_json::json!(&state.user_settings))
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

fn modify_profiles<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut Vec<serde_json::Value>) -> Result<T, ErrorCode>,
{
    let store = GameStore::new(app_handle, game_id)?;

    let mut profiles: Vec<serde_json::Value> = store
        .get("profiles")
        .ok_or(ErrorCode::InternalError)?
        .as_array()
        .ok_or(ErrorCode::InternalError)?
        .to_vec();

    let result = modify_fn(&mut profiles)?;

    store.set("profiles", serde_json::Value::Array(profiles));

    store.save().map_err(|e| {
        eprintln!("Failed to save profiles: {:?}", e);
        ErrorCode::InternalError
    })?;

    Ok(result)
}

fn modify_game<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut HashMap<String, serde_json::Value>) -> Result<T, ErrorCode>,
{
    let store = GameStore::new(app_handle, game_id)?;

    let entries = store.entries();

    let mut map: HashMap<String, serde_json::Value> = HashMap::from_iter(entries);

    let result = modify_fn(&mut map)?;

    for (k, v) in map {
        store.set(k, v);
    }

    store.save().map_err(|e| {
        eprintln!("Failed to save profiles: {:?}", e);
        ErrorCode::InternalError
    })?;

    Ok(result)
}

#[tauri::command]
pub fn create_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id.clone();

    modify_profiles(&app_handle, &game_id, |profiles| {
        if profiles
            .iter()
            .any(|p| p.get("name") == Some(&payload.name.clone().into()))
        {
            return Err(ErrorCode::Conflict);
        }

        let profile = serde_json::json!(Profile::from_dto(payload));
        profiles.push(profile.clone());

        Ok(profile)
    })
}

#[tauri::command]
pub fn update_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    println!("Updating profile: {:?}", payload);
    let game_id = payload.game_id.clone();

    modify_profiles(&app_handle, &game_id, |profiles| {
        let profile_index = profiles
            .iter()
            .position(|p| p.get("name") == Some(&payload.name.clone().into()))
            .ok_or(ErrorCode::NotFound)?;

        let profile = serde_json::json!(Profile::from_dto(payload));
        profiles[profile_index] = profile.clone();

        Ok(profile)
    })
}

#[tauri::command]
pub fn rename_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    old_name: &str,
    new_name: &str,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        if profiles
            .iter()
            .any(|p| p.get("name") == Some(&new_name.into()))
        {
            return Err(ErrorCode::Conflict);
        }

        let profile_index = profiles
            .iter()
            .position(|p| p.get("name") == Some(&old_name.into()))
            .ok_or(ErrorCode::NotFound)?;

        if let Some(profile_obj) = profiles[profile_index].as_object_mut() {
            profile_obj.insert("name".to_string(), new_name.into());
        } else {
            return Err(ErrorCode::InternalError);
        }

        Ok(profiles[profile_index].clone())
    })
}

#[tauri::command]
pub fn set_default_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
) -> Result<(), ErrorCode> {
    modify_game(&app_handle, game_id, |game| {
        if game.get("defaultProfile") == Some(&profile_name.into()) {
            return Ok(());
        }

        if !game
            .get("profiles")
            .and_then(|p| p.as_array())
            .ok_or(ErrorCode::InternalError)?
            .iter()
            .any(|p| p.get("name") == Some(&profile_name.into()))
        {
            return Err(ErrorCode::NotFound);
        }

        game.insert("defaultProfile".to_string(), profile_name.into());

        Ok(())
    })
}

#[tauri::command]
pub fn delete_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
) -> Result<(), ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        let profile_index = profiles
            .iter()
            .position(|p| p.get("name") == Some(&profile_name.into()))
            .ok_or(ErrorCode::NotFound)?;

        profiles.remove(profile_index);

        Ok(())
    })
}

#[tauri::command]
pub async fn stop_game<'a>(state: AppState<'a>) -> Result<(), ErrorCode> {
    let mut local_state = state.lock().await;
    local_state.game_runner.take().unwrap().kill_game().unwrap();

    Ok(())
}

#[tauri::command]
// return the pid of the launched game if successful
pub async fn start_game<'a>(
    app_handler: tauri::AppHandle,
    state: AppState<'a>,
    game_id: &str,
    profile_name: &str,
    save_name: Option<&str>,
) -> Result<(), ErrorCode> {
    let game_store = get_game_response_from_store(&app_handler, game_id)?;

    let profile = game_store
        .profiles
        .iter()
        .find(|p| p.name == profile_name)
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

    println!("TODO: Using save game path: {:?}", savegame_path);

    if !game_path.exists() {
        return Err(ErrorCode::InternalError);
    }

    let mod_writer = mods::writer::ModWriter::new(&profile.mods, &mods_path, &workshop_path);

    mod_writer
        .write(txt_path)
        .expect("It wasn't possible to write the mod file");

    let mut runner = if cfg!(target_os = "linux") {
        LinuxLauncher::new(&app_handler).await
    } else {
        unimplemented!("Game launching is only implemented for Linux at the moment");
    };

    let _ = runner.launch_game(&game_id, &game_path, savegame_path.as_ref());

    let mut state = state.lock().await;

    state.game_runner = Some(Box::new(runner));

    Ok(())
}
