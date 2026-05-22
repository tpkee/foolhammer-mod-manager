use lazy_static::lazy_static;
use steamlocate::SteamDir;

lazy_static! {
    pub static ref STEAMDIR_INSTANCE: Option<SteamDir> = match SteamDir::locate() {
        Ok(steam_dir) => {
            log::info!(
                "Auto-detected Steam directory: {}",
                steam_dir.path().display()
            );
            Some(steam_dir)
        }
        Err(e) => {
            log::error!("Failed to locate Steam directory: {:?}", e);
            None
        }
    };
}
