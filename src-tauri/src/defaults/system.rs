use lazy_static::lazy_static;
use std::path::PathBuf;
use steamlocate::SteamDir;
lazy_static! {
    pub static ref PROGRAM_FILES_PATH: PathBuf = match std::env::consts::OS { // this is pointless now, keeping it just in case
        "windows" => std::env::var("FOLDERID_ProgramFiles")
            .expect("Failed to get ProgramFiles environment variable")
            .into(),
        _ => dirs::data_dir().expect("Failed to get data directory"),
    };

    pub static ref STEAMWORKS_CLIENT_INSTANCE: Option<steamworks::Client> = match steamworks::Client::init() {
        Ok(client) => Some(client),
        Err(e) => {
            eprintln!("Failed to initialize Steam client: {:?}", e);
            None
        }
    };

    pub static ref STEAMDIR_INSTANCE: Option<SteamDir> = match SteamDir::locate() { // this is useful to lookup apps and paths even if the steam client isn't running
        Ok(steam_dir) => Some(steam_dir),
        _ => {
            eprintln!("Failed to locate Steam directory");
            None
        }
    };


}
