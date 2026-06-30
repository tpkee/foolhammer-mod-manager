use std::path::PathBuf;

use steamlocate::SteamDir;

use crate::{
    defaults::system::STEAMDIR_INSTANCE, mods::pack::ModPack, resolve_existing_path,
    stores::games::GameStore, stores::settings::SettingsStore, supported_games::SupportedGames,
    utils::ErrorCode,
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
            log::trace!("Using configured steam_library_path: {}", path.display());
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

    /// Ensure the Steam client is running, starting it if necessary. This is a
    /// prerequisite for any steamworks call. A no-op when Steam is already up.
    pub fn run_steam(&self) -> Result<(), ErrorCode> {
        let sys = sysinfo::System::new_all();
        if sys.processes_by_name("steam".as_ref()).count() > 0 {
            log::trace!("Steam already running");
            return Ok(());
        }

        log::info!("Steam not running, launching Steam");
        self.spawn_steam()
    }

    #[cfg(target_os = "windows")]
    fn spawn_steam(&self) -> Result<(), ErrorCode> {
        use std::os::windows::process::CommandExt;

        const DETACHED_PROCESS: u32 = 0x00000008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

        let steam_path = self.get_steam_path().ok_or_else(|| {
            log::error!("Cannot launch Steam: Steam isn't installed");
            ErrorCode::NotFound
        })?;

        let steam_exe = resolve_existing_path!(&steam_path, "steam.exe").ok_or_else(|| {
            log::error!("Cannot launch Steam: steam.exe not found under {}", steam_path.display());
            ErrorCode::NotFound
        })?;

        log::info!("Steam executable: {}", steam_exe.display());

        std::process::Command::new(&steam_exe)
            .creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP)
            .spawn()
            .map_err(|e| {
                log::error!("Failed to launch Steam: {:?}", e);
                ErrorCode::InternalError
            })?;

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn spawn_steam(&self) -> Result<(), ErrorCode> {
        std::process::Command::new("steam").spawn().map_err(|e| {
            log::error!("Failed to launch Steam: {:?}", e);
            ErrorCode::InternalError
        })?;

        // Steam is eepy on a cold start; give it a moment before callers query it.
        std::thread::sleep(std::time::Duration::from_secs(10));

        Ok(())
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

/// Fill in missing custom names for Steam Workshop mods by querying their real
/// titles from Steam. Only mods that don't already have a custom name are
/// touched, so manual renames are never overwritten.
///
/// This is best-effort and cosmetic: any failure (no workshop path, Steam can't
/// be started, query fails) is logged and ignored — mods still load with their
/// pack-name fallback.
pub async fn enrich_missing_workshop_names(app_handle: &tauri::AppHandle, game_id: SupportedGames) {
    let steam_config = match SteamConfig::from_app_handle(app_handle) {
        Ok(config) => config,
        Err(e) => {
            log::warn!("Skipping workshop name enrichment: {:?}", e);
            return;
        }
    };

    let Some(workshop_path) = steam_config.retrieve_steam_workshop_path(game_id) else {
        log::debug!("No workshop path; skipping workshop name enrichment");
        return;
    };

    // Existing custom names for this game (the gate for "only when missing").
    let existing = match GameStore::get_store(app_handle, game_id)
        .and_then(|store| GameStore::from_entries(store.entries()))
    {
        Ok(game) => game.mod_custom_names,
        Err(e) => {
            log::warn!("Could not read custom names, skipping enrichment: {:?}", e);
            return;
        }
    };

    // Cheap scan: pack_name -> workshop_id for mods lacking a name.
    let pending: Vec<(String, u64)> = ModPack::scan_workshop_ids(&workshop_path)
        .into_iter()
        .filter(|(name, _)| !existing.contains_key(name))
        .collect();

    if pending.is_empty() {
        return;
    }

    log::info!(
        "{} workshop mod(s) missing names; fetching from Steam",
        pending.len()
    );

    if let Err(e) = steam_config.run_steam() {
        log::warn!("Could not start Steam, skipping name fetch: {:?}", e);
        return;
    }

    let ids: Vec<u64> = pending.iter().map(|(_, id)| *id).collect();
    let titles = tauri::async_runtime::spawn_blocking(move || {
        crate::utils::steam_client::fetch_workshop_titles(game_id, ids)
    })
    .await
    .unwrap_or_default();

    // Map fetched titles back to pack names.
    let resolved: Vec<(String, String)> = pending
        .into_iter()
        .filter_map(|(name, id)| titles.get(&id).map(|title| (name, title.clone())))
        .collect();

    if resolved.is_empty() {
        return;
    }

    let persisted = GameStore::get(app_handle, game_id, |game| {
        for (name, title) in resolved {
            // Re-check at write time in case a name was set concurrently.
            game.mod_custom_names.entry(name).or_insert(title);
        }
        Ok(())
    })
    .await;

    if let Err(e) = persisted {
        log::warn!("Failed to persist fetched workshop names: {:?}", e);
    }
}
