use crate::dto::games::GameResponseDto;
use crate::stores::games::GameStore;
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
