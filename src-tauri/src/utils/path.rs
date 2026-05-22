use std::path::PathBuf;

use tauri::Manager;

use crate::{join_path, supported_games::SupportedGames, utils::steam::SteamConfig};

pub fn generate_store_path(app: &tauri::AppHandle, relative_path: &str) -> std::path::PathBuf {
    let path = app
        .path()
        .config_dir()
        .expect("Failed to get config directory")
        .join(format!("foolhammer-mod-manager/{}", relative_path));
    log::debug!(
        "Store path for '{}': {}",
        relative_path,
        path.display()
    );
    path
}

pub fn retrieve_saves_absolute_path(
    game_id: SupportedGames,
    relative_path: &str,
    steam_config: &SteamConfig,
) -> Option<PathBuf> {
    let game_id_str: String = game_id.into();
    log::info!(
        "Resolving saves path for game {} (relative: {})",
        game_id_str,
        relative_path
    );

    let data_dir = dirs::data_dir().expect("Failed to get data directory");

    let path = match std::env::consts::OS {
        "windows" => {
            log::info!("Saves path (windows data dir): {}", data_dir.display());
            Some(data_dir)
        }
        _ => {
            if let Some(steam_path) = steam_config.get_steam_path() {
                let path = join_path!(
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
                );
                log::info!("Saves path (linux/proton): {}", path.display());
                Some(path)
            } else {
                log::warn!(
                    "Saves path unavailable for game {}: no Steam path configured",
                    game_id_str
                );
                None
            }
        }
    };

    if let Some(ref p) = path {
        if !p.exists() {
            log::warn!("Saves path does not exist: {}", p.display());
        }
    }

    path
}
