use crate::{
    defaults::{self},
    supported_games::SupportedGames,
    utils::{self, ErrorCode},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tauri::Wry;

#[derive(Debug)]
pub enum SettingsKey {
    DefaultGame,
    SteamPath,
    SteamLibraryPath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsStore {
    pub default_game: SupportedGames,
    pub steam_path: Option<PathBuf>,
    pub steam_library_path: Option<PathBuf>,
}

impl From<SettingsKey> for String {
    fn from(val: SettingsKey) -> Self {
        match val {
            SettingsKey::DefaultGame => "default_game".to_string(),
            SettingsKey::SteamPath => "steam_path".to_string(),
            SettingsKey::SteamLibraryPath => "steam_library_path".to_string(),
        }
    }
}

impl Default for SettingsStore {
    fn default() -> Self {
        Self {
            default_game: defaults::games::DEFAULT_GAME_ID,
            steam_path: None,
            steam_library_path: None,
        }
    }
}

impl SettingsStore {
    pub fn get_store(
        app_handle: &tauri::AppHandle,
    ) -> Result<Arc<tauri_plugin_store::Store<Wry>>, ErrorCode> {
        let defaults = Self::default()
            .to_hashmap()
            .or(Err(ErrorCode::InternalError))?;

        let path = utils::path::generate_store_path(app_handle, "settings.json");

        let store = tauri_plugin_store::StoreBuilder::new(app_handle, path)
            .defaults(defaults)
            .auto_save(std::time::Duration::from_millis(500))
            .build()
            .or(Err(ErrorCode::InternalError))?;

        Ok(store)
    }

    pub fn to_hashmap(&self) -> Result<HashMap<String, Value>, serde_json::Error> {
        serde_json::from_value(self.serialize(serde_json::value::Serializer)?)
    }

    pub fn from_entries(entries: Vec<(String, Value)>) -> Result<Self, ErrorCode> {
        let hm: HashMap<String, Value> = entries.into_iter().collect();
        serde_json::from_value(serde_json::json!(hm)).or(Err(ErrorCode::InternalError))
    }
}
