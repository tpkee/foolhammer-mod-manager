use std::path::PathBuf;

use crate::{
    defaults::games::{DefaultGameInfo, SUPPORTED_GAMES},
    dto::{self, games::GameResponseDto},
    join_path,
    launchers::{GameManager, linux::LinuxLauncher},
    mods::{self},
    state::app_state::AppState,
    stores::games::{GameStore, ModInfo, Profile},
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

fn modify_game<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut GameStore) -> Result<T, ErrorCode>,
{
    let store = GameStore::new(app_handle, game_id)?;
    let mut game = GameStore::from_entries(store.entries())?;

    let result = modify_fn(&mut game)?;

    for (k, v) in game.to_hashmap().or(Err(ErrorCode::InternalError))? {
        store.set(k, v);
    }

    store.save().map_err(|e| {
        eprintln!("Failed to save game: {:?}", e);
        ErrorCode::InternalError
    })?;

    Ok(result)
}

fn modify_profiles<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut Vec<Profile>) -> Result<T, ErrorCode>,
{
    modify_game(app_handle, game_id, |game| modify_fn(&mut game.profiles))
}

fn modify_profile<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut Profile) -> Result<T, ErrorCode>,
{
    modify_profiles(app_handle, game_id, |profiles| {
        let mut profile = profiles
            .iter_mut()
            .find(|p| p.name == profile_name)
            .ok_or(ErrorCode::NotFound)?;

        modify_fn(&mut profile)
    })
}

#[tauri::command]
pub fn create_profile(
    app_handle: tauri::AppHandle,
    payload: dto::profiles::ProfileRequestDto,
) -> Result<serde_json::Value, ErrorCode> {
    let game_id = payload.game_id.clone();

    modify_profiles(&app_handle, &game_id, |profiles| {
        if profiles.iter().any(|p| p.name == payload.name) {
            return Err(ErrorCode::Conflict);
        }

        let profile = Profile::from(payload);
        profiles.push(profile.clone());

        Ok(serde_json::json!(profile))
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
        let idx = profiles
            .iter()
            .position(|p| p.name == payload.name)
            .ok_or(ErrorCode::NotFound)?;

        let profile = Profile::from(payload);
        profiles[idx] = profile.clone();

        Ok(serde_json::json!(profile))
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
        if profiles.iter().any(|p| p.name == new_name) {
            return Err(ErrorCode::Conflict);
        }

        let profile = profiles
            .iter_mut()
            .find(|p| p.name == old_name)
            .ok_or(ErrorCode::NotFound)?;

        profile.name = new_name.to_string();

        Ok(serde_json::json!(&*profile))
    })
}

#[tauri::command]
pub fn set_default_profile(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
) -> Result<(), ErrorCode> {
    modify_game(&app_handle, game_id, |game| {
        if game.default_profile.as_deref() == Some(profile_name) {
            return Ok(());
        }

        if !game.profiles.iter().any(|p| p.name == profile_name) {
            return Err(ErrorCode::NotFound);
        }

        game.default_profile = Some(profile_name.to_string());

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
        let idx = profiles
            .iter()
            .position(|p| p.name == profile_name)
            .ok_or(ErrorCode::NotFound)?;

        profiles.remove(idx);

        Ok(())
    })
}

#[tauri::command]
// this mod doesn't add any custom order
pub fn set_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
    mods: Vec<dto::mods::ModRequestDto>,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profile(&app_handle, game_id, profile_name, |profile| {
        profile.mods = mods.into_iter().map(ModInfo::from).collect();
        Ok(serde_json::json!(&profile.mods))
    })
}

#[tauri::command]
// if manual order is enabled it will add the order from oldLen to newLen UNLESS a custom order was already provided.
pub fn add_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
    mods: Vec<dto::mods::ModRequestDto>,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profile(&app_handle, game_id, profile_name, |profile| {
        if profile.manual_mode {
            let old_len = profile.mods.len();
            let new_mods: Vec<ModInfo> = mods
                .into_iter()
                .enumerate()
                .map(|(i, m)| ModInfo {
                    name: m.name,
                    enabled: m.enabled,
                    order: m.order.unwrap_or(u32::try_from(old_len + i).unwrap_or(0)),
                })
                .collect();

            profile.mods.extend(new_mods);
        } else {
            profile.mods.extend(mods.into_iter().map(ModInfo::from));
        }
        Ok(serde_json::json!(&profile.mods))
    })
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
