use crate::commands::helpers::{get_game_response_from_store, modify_profile};
use crate::dto::games::GameResponseDto;
use crate::dto::mods::ModRequestDto;
use crate::join_path;
use crate::launchers::{GameManager, linux::LinuxLauncher};
use crate::mods;
use crate::state::app_state::AppState;
use crate::stores::games::ModInfo;
use crate::utils::ErrorCode;

#[tauri::command]
pub fn set_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
    mods: Vec<ModRequestDto>,
) -> Result<serde_json::Value, ErrorCode> {
    modify_profile(&app_handle, game_id, profile_name, |profile| {
        profile.mods = mods.into_iter().map(ModInfo::from).collect();
        Ok(serde_json::json!(&profile.mods))
    })
}

#[tauri::command]
pub fn add_profile_mods(
    app_handle: tauri::AppHandle,
    game_id: &str,
    profile_name: &str,
    mods: Vec<ModRequestDto>,
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
