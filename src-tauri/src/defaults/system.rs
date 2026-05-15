use lazy_static::lazy_static;
use steamlocate::SteamDir;
lazy_static! {
    pub static ref STEAMDIR_INSTANCE: Option<SteamDir> = match SteamDir::locate() { // this is useful to lookup apps and paths even if the steam client isn't running
        Ok(steam_dir) => Some(steam_dir),
        _ => {
            eprintln!("Failed to locate Steam directory");
            None
        }
    };
}
