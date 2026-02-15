use crate::{
    defaults::{games, system::STEAMDIR_INSTANCE},
    pathbuf_to_string, resolve_existing_path,
    utils::path::retrieve_saves_absolute_path,
};
use notify::{Event, RecursiveMode, Watcher};
use serde::Serialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::Emitter;

use crate::state::user_settings;

#[derive(Debug, Serialize)]
pub struct State {
    pub user_settings: user_settings::UserSettings,

    #[serde(skip)]
    watcher: notify::RecommendedWatcher,
}

pub type AppState<'a> = tauri::State<'a, Mutex<State>>;

impl State {
    pub fn set_settings_from_store(
        &mut self,
        app_handle: &tauri::AppHandle,
        entries: Vec<(String, serde_json::Value)>,
    ) {
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

        self.update_user_setting(app_handle, user_settings);
    }

    pub fn update_user_setting(
        &mut self,
        app_handle: &tauri::AppHandle,
        settings: user_settings::UserSettings,
    ) {
        self.user_settings = settings;

        app_handle
            .emit("update/user-settings", &self.user_settings)
            .expect("Failed to emit update/user-settings event");
    }
}

impl Default for State {
    fn default() -> Self {
        let user_settings = default_user_settings();
        let watcher = notify::recommended_watcher(watcher_sentry)
            .expect("Failed to create watcher for State default");

        Self {
            watcher,
            user_settings,
        }
    }
}

fn default_user_settings() -> user_settings::UserSettings {
    let default_game = games::SUPPORTED_GAMES
        .iter()
        .find(|game| game.game_id == games::DEFAULT_GAME_ID)
        .unwrap_or(&games::DefaultGameInfo {
            game_id: "",
            executable_name: "",
            mods_path: "data",
            saves_path: "",
        });

    let game_path = default_game.get_game_path();

    println!("Default game path: {:?}", game_path);

    let saves_data_dir = match retrieve_saves_absolute_path(default_game.game_id) {
        Some(dir) => {
            pathbuf_to_string!(resolve_existing_path!(&dir, default_game.saves_path,)).into()
        }
        None => serde_json::Value::Null,
    };

    let workshop_folder: serde_json::Value = match &*STEAMDIR_INSTANCE {
        Some(steam_dir) => {
            match resolve_existing_path!(
                steam_dir.path(),
                "steamapps",
                "workshop",
                "content",
                default_game.game_id,
            ) {
                Some(p) => p.to_string_lossy().into_owned().into(),
                None => serde_json::Value::Null,
            }
        }
        _ => serde_json::Value::Null,
    };

    let mods_path = match game_path {
        Some(str) => pathbuf_to_string!(resolve_existing_path!(&str, default_game.mods_path)),
        _ => None,
    };

    HashMap::from([
        (
            user_settings::SettingKey::GameId,
            default_game.game_id.to_string().into(),
        ),
        (
            user_settings::SettingKey::GamePath,
            serde_json::json!(default_game.get_game_path().clone()),
        ),
        (
            user_settings::SettingKey::SteamWorkshopPath,
            workshop_folder,
        ),
        (user_settings::SettingKey::SavesPath, saves_data_dir),
        (
            user_settings::SettingKey::ModsPath,
            serde_json::json!(mods_path),
        ),
    ])
}

fn watcher_sentry(event: Result<Event, notify::Error>) {
    // call the governor if the event was an edit
    match event {
        Ok(e) => match e.kind {
            notify::EventKind::Create(_)
            | notify::EventKind::Modify(_)
            | notify::EventKind::Remove(_) => {
                folders_governor(e);
            }
            _ => {}
        },
        Err(e) => {
            eprintln!("Error watching folder: {:?}", e); // TODO: uhm, find what to do when this happens...? Stop the watcher or dunno
        }
    }
}

fn folders_governor(event: Event) {
    // TODO: should rescan and do things based on the folder path
    println!("Folder event detected: {:?}", event);
}
