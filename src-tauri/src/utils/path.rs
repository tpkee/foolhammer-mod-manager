use std::path::PathBuf;

use tauri::Manager;

use crate::{
    join_path,
    supported_games::SupportedGames,
    utils::steam::SteamConfig,
};

pub fn generate_store_path(app: &tauri::AppHandle, relative_path: &str) -> std::path::PathBuf {
    app.path()
        .config_dir()
        .expect("Failed to get config directory")
        .join(format!("foolhammer-mod-manager/{}", relative_path))
}

pub fn retrieve_saves_absolute_path(
    game_id: SupportedGames,
    relative_path: &str,
    steam_config: &SteamConfig,
) -> Option<PathBuf> {
    // The default saves path needs to be handled differently because on Linux we have to access the wine pfx
    let data_dir = dirs::data_dir().expect("Failed to get data directory");
    let game_id_str: String = game_id.into();

    match std::env::consts::OS {
        "windows" => Some(data_dir),
        _ => {
            if let Some(steam_path) = steam_config.get_steam_path() {
                return Some(join_path!(
                    &steam_path,
                    "steamapps",
                    "compatdata",
                    &game_id_str,
                    "pfx",
                    "drive_c",
                    "users",
                    "steamuser",
                    "AppData",
                    "Roaming",
                    relative_path
                ));
            }

            None
        }
    }
}
