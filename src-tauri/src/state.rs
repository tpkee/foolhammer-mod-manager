use crate::{
    defaults::{games, system},
    join_path, resolve_existing_path,
};
use notify::{Event, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{mpsc, Mutex},
};
pub type AppState<'a> = tauri::State<'a, Mutex<State>>;
pub type UserSettings = HashMap<SettingKey, serde_json::Value>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SettingKey {
    GameId,
    GamePath,
    SteamWorkshopPath,
}

impl SettingKey {
    pub fn get(&self) -> String {
        match self {
            Self::GameId => "game_id".to_string(),
            Self::GamePath => "game_path".to_string(),
            Self::SteamWorkshopPath => "steam_workshop_path".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "game_path" => Ok(Self::GamePath),
            "steam_workshop_path" => Ok(Self::SteamWorkshopPath),
            "game_id" => Ok(Self::GameId),
            _ => Err("Invalid SettingKey"),
        }
    }
}

#[derive(Debug)]
pub struct FolderWatcher {
    // drop to unwatch
    pub path: PathBuf,
    watcher: notify::RecommendedWatcher,
    //rx: mpsc::Receiver<notify::Result<Event>>,
}

impl FolderWatcher {
    pub fn new(path: PathBuf, callback: fn(event: Result<Event, notify::Error>)) -> Self {
        // let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(callback).expect(&format!(
            "Failed to create watcher forrecommended_watcher folder {:?}",
            path
        ));
        let config = notify::Config::default();
        watcher.configure(config);

        watcher
            .watch(&path, RecursiveMode::Recursive)
            .expect(&format!("Failed to watch folder {:?}", path));

        FolderWatcher { path, watcher }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    #[serde(skip)]
    pub game_folder: Option<FolderWatcher>,
    #[serde(skip)]
    pub steam_workshop_folder: Option<FolderWatcher>,
    pub user_settings: UserSettings,
}

impl State {
    pub fn set_settings_from_store(&mut self, entries: Vec<(String, serde_json::Value)>) {
        let mut user_settings: UserSettings = HashMap::new();
        let set_folder_watcher = |ptr: &mut Option<FolderWatcher>, path: &serde_json::Value| {
            if let Some(path_str) = path.as_str() {
                *ptr = Some(FolderWatcher::new(PathBuf::from(path_str), |event| {
                    println!("Folder event: {:?}", event);
                }));
            }
        };

        for (k, v) in entries {
            let Ok(setting_key) = SettingKey::from_str(&k) else {
                eprintln!("Settings key {} not found", k);
                continue;
            };

            match setting_key {
                SettingKey::GamePath => {
                    set_folder_watcher(&mut self.game_folder, &v);
                }
                SettingKey::SteamWorkshopPath => {
                    set_folder_watcher(&mut self.steam_workshop_folder, &v);
                }
                _ => {}
            }

            user_settings.insert(setting_key, v);
        }

        println!("User settings loaded from store: {:?}", user_settings);

        self.user_settings = user_settings;
    }
}

impl Default for State {
    fn default() -> Self {
        println!(
            "System program files path: {:?}",
            &*system::PROGRAM_FILES_PATH,
        );
        Self {
            game_folder: None,
            steam_workshop_folder: None,
            user_settings: default_user_settings(),
        }
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
        });

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
                default_game.executable_name
            ))
            .into(),
        ),
        (
            SettingKey::SteamWorkshopPath,
            pathbuf_to_string(resolve_existing_path!(
                &*system::PROGRAM_FILES_PATH,
                default_game.steam_workshop_path.unwrap_or(""),
                default_game.game_id
            ))
            .into(),
        ),
    ])
}

fn pathbuf_to_string(path: Option<PathBuf>) -> Option<String> {
    path.and_then(|p| Some(p.to_string_lossy().into_owned()))
}
