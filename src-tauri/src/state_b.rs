use crate::{
    defaults::{games, system},
    join_path, resolve_existing_path,
};
use notify::{Event, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{self, Path, PathBuf},
    sync::Mutex,
};
pub type AppState<'a> = tauri::State<'a, Mutex<State>>;
pub type UserSettings = HashMap<SettingKey, serde_json::Value>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SettingKey {
    GameId,
    GamePath,
    SteamWorkshopPath,
    SavesPath,
    ModsPath,
}

impl SettingKey {
    pub fn get(&self) -> String {
        match self {
            Self::GameId => "game_id".to_string(),
            Self::GamePath => "game_path".to_string(),
            Self::SteamWorkshopPath => "steam_workshop_path".to_string(),
            Self::SavesPath => "saves_path".to_string(),
            Self::ModsPath => "mods_path".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "game_path" => Ok(Self::GamePath),
            "steam_workshop_path" => Ok(Self::SteamWorkshopPath),
            "game_id" => Ok(Self::GameId),
            "saves_path" => Ok(Self::SavesPath),
            "mods_path" => Ok(Self::ModsPath),
            _ => Err("Invalid SettingKey"),
        }
    }

    pub fn is_path_setting(&self) -> bool {
        matches!(
            self,
            Self::GamePath | Self::ModsPath | Self::SavesPath | Self::SteamWorkshopPath
        )
    }
}

fn default_user_settings() -> UserSettings {
    let default_game = games::DEFAULT_GAMES_DATA
        .iter()
        .find(|game| game.game_id == games::DEFAULT_GAME_ID)
        .unwrap_or(&games::DefaultGameInfo {
            game_id: "",
            game_path: "",
            executable_name: "",
            steam_workshop_path: None,
            mods_path: "data",
            saves_path: "",
        });

    let data_dir = dirs::data_dir().expect("Failed to get data directory");

    HashMap::from([
        (
            SettingKey::GameId,
            serde_json::Value::String(default_game.game_id.to_string()), // Warhammer 3 on Steam
        ),
        (
            SettingKey::GamePath,
            pathbuf_to_string(resolve_existing_path!(
                &*system::PROGRAM_FILES_PATH,
                default_game.game_path,
            ))
            .into(),
        ),
        (
            SettingKey::SteamWorkshopPath,
            pathbuf_to_string(resolve_existing_path!(
                &*system::PROGRAM_FILES_PATH,
                default_game.steam_workshop_path.unwrap_or(""),
                default_game.game_id,
            ))
            .into(),
        ),
        (
            SettingKey::SavesPath,
            pathbuf_to_string(resolve_existing_path!(&data_dir, default_game.saves_path,)).into(),
        ),
        (
            SettingKey::ModsPath,
            pathbuf_to_string(resolve_existing_path!(
                &*system::PROGRAM_FILES_PATH,
                default_game.game_path,
                default_game.mods_path,
            ))
            .into(),
        ),
    ])
}

fn pathbuf_to_string(path: Option<PathBuf>) -> Option<String> {
    path.and_then(|p| Some(p.to_string_lossy().into_owned()))
}

fn folders_event_governor(event: Result<Event, notify::Error>) {
    match event {
        Ok(e) => match e.kind {
            notify::EventKind::Create(_)
            | notify::EventKind::Modify(_)
            | notify::EventKind::Remove(_) => {
                println!("Folder change detected: {:?}", e);
            }
            _ => {}
        },
        Err(e) => {
            eprintln!("Error watching folder: {:?}", e); // TODO: uhm, find what to do when this happens...? Stop the watcher or dunno
        }
    }
}
