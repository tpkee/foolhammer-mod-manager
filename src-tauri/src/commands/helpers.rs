use crate::dto::games::GameResponseDto;
use crate::stores::games::{GameStore, Profile};
use crate::supported_games::SupportedGames;
use crate::utils::ErrorCode;

pub fn get_game_response_from_store(
    app_handler: &tauri::AppHandle,
    game_id: SupportedGames,
) -> Result<GameResponseDto, ErrorCode> {
    let store = GameStore::get_store(app_handler, game_id)?;

    let game_store = GameResponseDto::from_store(GameStore::from_entries(store.entries())?);

    Ok(game_store)
}

pub async fn modify_game<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: SupportedGames,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut GameStore) -> Result<T, ErrorCode>,
{
    let store = GameStore::get_store(app_handle, game_id)?;
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

pub async fn modify_profiles<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: SupportedGames,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut Vec<Profile>) -> Result<T, ErrorCode>,
{
    modify_game(app_handle, game_id, |game| modify_fn(&mut game.profiles)).await
}

pub async fn modify_profile<F, T>(
    app_handle: &tauri::AppHandle,
    game_id: SupportedGames,
    profile_id: uuid::Uuid,
    modify_fn: F,
) -> Result<T, ErrorCode>
where
    F: FnOnce(&mut Profile) -> Result<T, ErrorCode>,
{
    modify_profiles(app_handle, game_id, |profiles| {
        let profile = profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or(ErrorCode::NotFound)?;

        modify_fn(profile)
    })
    .await
}
