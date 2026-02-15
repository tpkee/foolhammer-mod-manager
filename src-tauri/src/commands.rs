use crate::{
    defaults::games::{DefaultGameInfo, SUPPORTED_GAMES},
    dto::{self, games::GameResponseDto},
    stores::games::{GameStore, Profile},
};

use tauri::Emitter;
use utils::ErrorCode;

use crate::utils;

/*
    TODO/notes:
    - I don't like that the keys for the store are basically hardcoded here, they should be tied to an enum.
*/

#[tauri::command]
pub fn check_path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn get_supported_games() -> serde_json::Value {
    SUPPORTED_GAMES.map(|game| game.game_id).into()
}

#[tauri::command]
pub fn get_game(
    app_handle: tauri::AppHandle,
    game_id: &str,
) -> Result<serde_json::Value, ErrorCode> {
    let store = GameStore::new(&app_handle, game_id)?;

    let game_response = GameResponseDto::from_store(GameStore::from_entries(store.entries())?);

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
) -> Result<Vec<serde_json::Value>, ErrorCode> {
    modify_profiles(&app_handle, game_id, |profiles| {
        for profile in profiles.iter_mut() {
            if let Some(profile_obj) = profile.as_object_mut() {
                profile_obj.insert(
                    "default".to_string(),
                    serde_json::Value::Bool(profile_obj.get("name") == Some(&profile_name.into())),
                );
            }
        }

        Ok(profiles.clone())
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

enum GameLaunchEvent {
    Start,
    Error,
    Success,
}

impl GameLaunchEvent {
    fn as_str(&self) -> &'static str {
        match self {
            GameLaunchEvent::Start => "start",
            GameLaunchEvent::Error => "error",
            GameLaunchEvent::Success => "success",
        }
    }
}

fn kill_process_by_exe(game_id: &str) -> Result<(), ErrorCode> {
    let game = DefaultGameInfo::find_by_id(game_id).ok_or(ErrorCode::NotFound)?;

    let exe_name = game.executable_name;
    let sys = sysinfo::System::new_all();

    for process in sys.processes_by_name(exe_name.as_ref()) {
        process.kill();
    }

    Ok(())
}

#[tauri::command]
pub fn stop_game(app_handle: tauri::AppHandle, game_id: &str) -> Result<(), ErrorCode> {
    if let Err(e) = kill_process_by_exe(game_id) {
        eprintln!("Failed to kill existing game  {}: {:?}", game_id, e);
        return Err(e);
    }

    app_handle.emit("game_closed", game_id).unwrap();

    Ok(())
}

#[tauri::command]
// return the pid of the launched game if successful
pub async fn start_game<'a>(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
    save_name: Option<&str>,
) -> Result<(), ErrorCode> {
    let emitter = |event: GameLaunchEvent| {
        let _ = &app_handle
            .emit("game_launch", event.as_str())
            .map_err(|e| eprintln!("Failed to emit event {}: {:?}", event.as_str(), e));
    };

    let error = |e: ErrorCode| {
        emitter(GameLaunchEvent::Error);
        eprintln!("{:?}", e);
        return Err(e);
    };

    emitter(GameLaunchEvent::Start);

    let store = GameStore::new(&app_handle, &game_id).map_err(error);

    let Ok(store) = store else {
        eprintln!(
            "Failed to create game store for game_id {}: {:?}",
            game_id,
            store.as_ref().err()
        );
        return Err(store.err().unwrap().unwrap());
    };

    let game_store = GameResponseDto::from_store(
        GameStore::from_entries(store.entries())
            .map_err(error)
            .unwrap(),
    );

    let profile = game_store.profiles.iter().find(|p| p.name == profile_name);

    let Some(profile) = profile else {
        eprintln!("Profile {} not found for game_id {}", profile_name, game_id);
        return Err(ErrorCode::NotFound);
    };

    utils::game_launcher::launch_game(
        &app_handle,
        &game_store,
        &profile.mods,
        save_name.as_deref(),
    )
    .await?;

    emitter(GameLaunchEvent::Success);

    Ok(())
}
