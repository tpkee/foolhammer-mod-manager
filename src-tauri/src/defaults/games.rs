use std::path::PathBuf;

use crate::defaults::system::STEAMDIR_INSTANCE;

#[derive(Debug)]
pub struct DefaultGameInfo {
    pub game_id: &'static str,
    pub executable_name: &'static str,
    pub mods_path: &'static str, // relative path. It will be appended to the game_path to get the full path to the mods folder. Probably won't be changed ever
    pub saves_path: &'static str, // the default root should be the roaming folder (on Linux it is relative to proton's prefix, es: /home/<username>/.local/share/Steam/steamapps/compatdata/<gameid>/pfx/drive_c/users/steamuser/AppData/Roaming)
}

impl DefaultGameInfo {
    pub fn get_game_path(&self) -> Option<PathBuf> {
        match &*STEAMDIR_INSTANCE {
            Some(steam_dir) => match steam_dir.find_app(self.game_id.parse().unwrap()) {
                Ok(res) => {
                    let (app, library) = res.unwrap();
                    Some(library.resolve_app_dir(&app))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

pub const DEFAULT_GAME_ID: &str = "1142710";

pub const SUPPORTED_GAMES: [DefaultGameInfo; 1] = [DefaultGameInfo {
    game_id: "1142710",
    // steam_workshop_path: Some("/Steam/steamapps/workshop/content/"), // the game ID will be appended to this path to get the full workshop path.
    executable_name: "Warhammer3.exe",
    mods_path: "data/",
    saves_path: "/The Creative Assembly/save_games/",
}];
