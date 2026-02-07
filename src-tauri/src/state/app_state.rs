use crate::{
    defaults::{games, system},
    resolve_existing_path,
};
use notify::{Event, RecursiveMode, Watcher};
use serde::Serialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::state::user_settings;

#[derive(Debug, Serialize)]
pub struct State {
    pub user_settings: user_settings::UserSettings,
    #[serde(skip)]
    watcher: notify::RecommendedWatcher,
}

impl State {
    pub fn set_settings_from_store(&mut self, entries: Vec<(String, serde_json::Value)>) {
        let mut watcher_paths = self.watcher.paths_mut();
        let mut user_settings: user_settings::UserSettings = HashMap::new();
        let mut folders_to_watch: Vec<String> = vec![];

        for (k, v) in entries {
            let Ok(setting_key) = user_settings::SettingKey::from_str(&k) else {
                eprintln!("Settings key {} not found", k);
                continue;
            };

            if setting_key.is_path_setting()
                && setting_key != user_settings::SettingKey::GamePath
                && let Some(path_str) = v.as_str()
            {
                // no need to watch the gamepath since we don't care if the exe will be changed
                folders_to_watch.push(path_str.to_string());
            }

            user_settings.insert(setting_key, v);
        }

        for folder in &folders_to_watch {
            let p = Path::new(folder);
            if !p.exists() {
                continue;
            }

            watcher_paths
                .add(p, RecursiveMode::Recursive)
                .expect("Failed to watch folder from State::set_settings_from_store");
        }

        watcher_paths
            .commit()
            .is_ok()
            .then(|| println!("Watching folders: {:?}", folders_to_watch));

        self.update_user_setting(user_settings);
    }

    pub fn update_user_setting(&mut self, settings: user_settings::UserSettings) {
        self.user_settings = settings;
        // TODO: emit an event or idk do something
    }
}

impl Default for State {
    fn default() -> Self {
        let user_settings = default_user_settings();
        let watcher = notify::recommended_watcher(folders_event_governor)
            .expect("Failed to create watcher for State default");

        Self {
            watcher,
            user_settings,
        }
    }
}

fn default_user_settings() -> user_settings::UserSettings {
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
            user_settings::SettingKey::GameId,
            serde_json::Value::String(default_game.game_id.to_string()), // Warhammer 3 on Steam
        ),
        (
            user_settings::SettingKey::GamePath,
            pathbuf_to_string(resolve_existing_path!(
                &*system::PROGRAM_FILES_PATH,
                default_game.game_path,
            ))
            .into(),
        ),
        (
            user_settings::SettingKey::SteamWorkshopPath,
            pathbuf_to_string(resolve_existing_path!(
                &*system::PROGRAM_FILES_PATH,
                default_game.steam_workshop_path.unwrap_or(""),
                default_game.game_id,
            ))
            .into(),
        ),
        (
            user_settings::SettingKey::SavesPath,
            pathbuf_to_string(resolve_existing_path!(&data_dir, default_game.saves_path,)).into(),
        ),
        (
            user_settings::SettingKey::ModsPath,
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
