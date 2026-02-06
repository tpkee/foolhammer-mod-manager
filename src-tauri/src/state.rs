use crate::defaults::games::{DefaultGameInfo, DEFAULT_GAMES_DATA, DEFAULT_GAME_ID};
use notify::{Event, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::Path,
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
    pub path: String,
    watcher: notify::RecommendedWatcher,
    rx: mpsc::Receiver<notify::Result<Event>>,
}

impl FolderWatcher {
    pub fn new(path: &str) -> Self {
        let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

        let watcher = notify::recommended_watcher(tx).unwrap();

        FolderWatcher {
            path: path.to_string(),
            watcher,
            rx,
        }
    }

    pub fn watch(&mut self, callback: fn(event: Event)) {
        self.watcher
            .watch(Path::new(&self.path), RecursiveMode::Recursive)
            .unwrap(); // TODO: Handle errors properly instead of unwrapping everywhere

        for res in &self.rx {
            match res {
                Ok(event) => callback(event),
                Err(e) => println!("watch error: {:?}", e), // we should cleanup or dunno
            }
        }
    }

    pub fn unwatch(mut self) {
        self.watcher.unwatch(Path::new(&self.path)).unwrap();
    }

    pub fn check_path(&self, path: &str) -> bool {
        self.path == path
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
                *ptr = Some(initialize_watcher(path_str));
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

        self.user_settings = user_settings;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            game_folder: None,
            steam_workshop_folder: None,
            user_settings: default_user_settings(),
        }
    }
}

fn default_user_settings() -> UserSettings {
    let default_game = DEFAULT_GAMES_DATA
        .iter()
        .find(|game| game.game_id == DEFAULT_GAME_ID)
        .unwrap_or(&DefaultGameInfo {
            game_id: "",
            game_path: "",
            executable_name: "",
            workshop_path: None,
        });

    [
        (
            SettingKey::GameId,
            serde_json::Value::String(default_game.game_id.to_string()), // Warhammer 3 on Steam
        ),
        (SettingKey::GamePath, default_game.game_path.into()),
        (
            SettingKey::SteamWorkshopPath,
            default_game.workshop_path.into(),
        ),
    ]
    .into_iter()
    .collect()
}

fn initialize_watcher(path: &str) -> FolderWatcher {
    let mut folder_watcher = FolderWatcher::new(path);
    folder_watcher.watch(|event| {
        println!("Folder event: {:?}", event); // TODO: emit an event? idk
    });
    folder_watcher
}
