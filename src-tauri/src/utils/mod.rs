use std::path::PathBuf;

use tauri::Manager;

use crate::{
    defaults::system::STEAMDIR_INSTANCE, join_path, pathbuf_to_string, resolve_existing_path,
};

pub mod custom_macro;

pub fn generate_store_path(app: &tauri::AppHandle, relative_path: &str) -> std::path::PathBuf {
    app.path()
        .config_dir()
        .expect("Failed to get config directory")
        .join(format!("foolhamer-mod-manager/{}", relative_path))
}

pub fn retrieve_saves_absolute_path(game_id: &str) -> Option<PathBuf> {
    // The default saves path needs to be handled differently because on Linux we have to access the wine pfx
    let data_dir = dirs::data_dir().expect("Failed to get data directory");

    match std::env::consts::OS {
        "windows" => Some(data_dir),
        _ => {
            if let Some(steam_dir) = &*STEAMDIR_INSTANCE {
                return Some(PathBuf::from(join_path!(
                    steam_dir.path(),
                    "steamapps",
                    "compatdata",
                    game_id,
                    "pfx",
                    "drive_c",
                    "users",
                    "steamuser",
                    "AppData",
                    "Roaming"
                )));
            }

            None
        }
    }
}

pub fn retrieve_steam_workshop_path(game_id: &str) -> Option<String> {
    match &*STEAMDIR_INSTANCE {
        Some(steam_dir) => {
            if let Some(p) = resolve_existing_path!(
                steam_dir.path(),
                "steamapps",
                "workshop",
                "content",
                game_id,
            ) {
                return pathbuf_to_string!(p);
            }

            return None;
        }
        _ => None,
    }
}
