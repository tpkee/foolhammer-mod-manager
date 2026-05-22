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
        let config = Self {
            steam_path: settings.steam_path.clone(),
            steam_library_path: settings.steam_library_path.clone(),
        };
        log::debug!(
            "SteamConfig from settings: steam_path={:?}, steam_library_path={:?}",
            config.steam_path.as_ref().map(|p| p.display().to_string()),
            config
                .steam_library_path
                .as_ref()
                .map(|p| p.display().to_string())
        );
        config
    }

    pub fn from_app_handle(app_handle: &tauri::AppHandle) -> Result<Self, ErrorCode> {
        let store = SettingsStore::get_store(app_handle)?;
        let settings = SettingsStore::from_entries(store.entries())?;
        Ok(Self::from_settings(&settings))
    }

    /// User-configured Steam installation path, or the auto-detected default.
    pub fn get_steam_path(&self) -> Option<PathBuf> {
        if let Some(path) = &self.steam_path {
            log::trace!("Using configured steam_path: {}", path.display());
            return Some(path.clone());
        }

        STEAMDIR_INSTANCE
            .as_ref()
            .map(|steam_dir| steam_dir.path().to_path_buf())
    }

    /// User-configured library path, then installation path, then auto-detected default.
    pub fn get_steam_library_path(&self) -> Option<PathBuf> {
        if let Some(path) = &self.steam_library_path {
            log::trace!(
                "Using configured steam_library_path: {}",
                path.display()
            );
            return Some(path.clone());
        }

        self.get_steam_path()
    }

    /// SteamDir from user installation path, or the auto-detected default instance.
    pub fn get_steam_dir(&self) -> Option<SteamDir> {
        if let Some(path) = &self.steam_path
            && let Ok(steam_dir) = SteamDir::from_dir(path)
        {
            log::debug!("SteamDir from configured path: {}", path.display());
            return Some(steam_dir);
        }

        STEAMDIR_INSTANCE.clone()
    }

    pub fn retrieve_steam_workshop_path(&self, game_id: SupportedGames) -> Option<PathBuf> {
        let game_id_str: String = game_id.into();
        log::info!("Resolving workshop path for game {}", game_id_str);

        if let Some(library_path) = self.get_steam_library_path() {
            let path = resolve_existing_path!(
                &library_path,
                "steamapps",
                "workshop",
                "content",
                &game_id_str,
            );
            match &path {
                Some(p) => log::info!("Workshop path: {}", p.display()),
                None => log::warn!(
                    "Workshop path not found under library {}",
                    library_path.display()
                ),
            }
            return path;
        }

        log::warn!(
            "Workshop path unavailable for game {}: no Steam library path",
            game_id_str
        );
        None
    }

    pub fn retrieve_wine_pfx_path(&self, game_id: SupportedGames) -> Option<PathBuf> {
        let game_id_str: String = game_id.into();
        log::info!("Resolving wine prefix path for game {}", game_id_str);

        if let Some(library_path) = self.get_steam_library_path() {
            let path =
                resolve_existing_path!(&library_path, "steamapps", "compatdata", &game_id_str);
            match &path {
                Some(p) => log::info!("Wine prefix path: {}", p.display()),
                None => log::warn!(
                    "Wine prefix not found under library {}",
                    library_path.display()
                ),
            }
            return path;
        }

        log::warn!(
            "Wine prefix unavailable for game {}: no Steam library path",
            game_id_str
        );
        None
    }
}
