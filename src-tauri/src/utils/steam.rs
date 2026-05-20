use std::path::PathBuf;

use steamlocate::SteamDir;

use crate::{
    defaults::system::STEAMDIR_INSTANCE, resolve_existing_path, stores::settings::SettingsStore,
    supported_games::SupportedGames, utils::ErrorCode,
};

#[derive(Debug, Clone)]
pub struct SteamConfig {
    pub steam_path: Option<PathBuf>,
    pub steam_library_path: Option<PathBuf>,
}

impl SteamConfig {
    pub fn from_settings(settings: &SettingsStore) -> Self {
        Self {
            steam_path: settings.steam_path.clone(),
            steam_library_path: settings.steam_library_path.clone(),
        }
    }

    pub fn from_app_handle(app_handle: &tauri::AppHandle) -> Result<Self, ErrorCode> {
        let store = SettingsStore::get_store(app_handle)?;
        let settings = SettingsStore::from_entries(store.entries())?;
        Ok(Self::from_settings(&settings))
    }

    /// User-configured Steam installation path, or the auto-detected default.
    pub fn get_steam_path(&self) -> Option<PathBuf> {
        if let Some(path) = &self.steam_path {
            return Some(path.clone());
        }

        STEAMDIR_INSTANCE
            .as_ref()
            .map(|steam_dir| steam_dir.path().to_path_buf())
    }

    /// User-configured library path, then installation path, then auto-detected default.
    pub fn get_steam_library_path(&self) -> Option<PathBuf> {
        if let Some(path) = &self.steam_library_path {
            return Some(path.clone());
        }

        self.get_steam_path()
    }

    /// SteamDir from user installation path, or the auto-detected default instance.
    pub fn get_steam_dir(&self) -> Option<SteamDir> {
        if let Some(path) = &self.steam_path
            && let Ok(steam_dir) = SteamDir::from_dir(path)
        {
            return Some(steam_dir);
        }

        STEAMDIR_INSTANCE.clone()
    }

    pub fn retrieve_steam_workshop_path(&self, game_id: SupportedGames) -> Option<PathBuf> {
        let game_id_str: String = game_id.into();

        if let Some(library_path) = self.get_steam_library_path() {
            return resolve_existing_path!(
                &library_path,
                "steamapps",
                "workshop",
                "content",
                &game_id_str,
            );
        }

        None
    }

    pub fn retrieve_wine_pfx_path(&self, game_id: SupportedGames) -> Option<PathBuf> {
        let game_id_str: String = game_id.into();

        if let Some(library_path) = self.get_steam_library_path() {
            return resolve_existing_path!(&library_path, "steamapps", "compatdata", &game_id_str);
        }

        None
    }
}
