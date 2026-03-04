use crate::dto::games::GameResponseDto;
use crate::stores::games::{GameStore, Profile};
use crate::utils::ErrorCode;

pub fn get_game_response_from_store(
    app_handler: &tauri::AppHandle,
    game_id: &str,
) -> Result<GameResponseDto, ErrorCode> {
    let store = GameStore::new(app_handler, game_id)?;

    let game_store = GameResponseDto::from_store(GameStore::from_entries(store.entries())?);

    Ok(game_store)
}

pub fn modify_game<F, T>(
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

pub fn modify_profiles<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut Vec<Profile>) -> Result<T, ErrorCode>,
{
    modify_game(app_handle, game_id, |game| modify_fn(&mut game.profiles))
}

pub fn modify_profile<F, T>(
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
