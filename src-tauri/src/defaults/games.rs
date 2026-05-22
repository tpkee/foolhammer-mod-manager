use crate::supported_games::SupportedGames;
use crate::utils::steam::SteamConfig;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DefaultGameInfo {
    pub game_id: SupportedGames,
    pub executable_name: &'static str,
    pub mods_path: &'static str, // relative path. It will be appended to the game_path to get the full path to the mods folder. Probably won't be changed ever
    pub saves_path: &'static str, // the default root should be the roaming folder (on Linux it is relative to proton's prefix, es: /home/<username>/.local/share/Steam/steamapps/compatdata/<gameid>/pfx/drive_c/users/steamuser/AppData/Roaming)
}
impl DefaultGameInfo {
    pub fn get_game_path(&self, steam_config: &SteamConfig) -> Option<PathBuf> {
        let game_id_str: String = self.game_id.into();
        let steam_dir = steam_config.get_steam_dir()?;
        let (app, library) = match steam_dir.find_app(self.game_id.into()) {
            Ok(Some(found)) => found,
            Ok(None) => {
                log::warn!("Steam app not found for game {}", game_id_str);
                return None;
            }
            Err(e) => {
                log::warn!("Failed to find Steam app for game {}: {:?}", game_id_str, e);
                return None;
            }
        };
        let path = library.resolve_app_dir(&app);
        log::info!("Resolved game path for {}: {}", game_id_str, path.display());
        Some(path)
    }

    pub fn find_by_id(game_id: SupportedGames) -> Option<&'static DefaultGameInfo> {
        SUPPORTED_GAMES.iter().find(|game| game.game_id == game_id)
    }
}

pub const DEFAULT_GAME_ID: SupportedGames = SupportedGames::Warhammer3;

// TODO: source this from a json file or something instead of hardcoding it. This will make it easier to add support for more games in the future without having to change the code
pub const SUPPORTED_GAMES: [DefaultGameInfo; 1] = [DefaultGameInfo {
    game_id: SupportedGames::Warhammer3,
    executable_name: "Warhammer3.exe",
    mods_path: "data/",
    saves_path: "/The Creative Assembly/Warhammer3/save_games/",
}];
