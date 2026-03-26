use crate::defaults::system::STEAMDIR_INSTANCE;
use crate::supported_games::SupportedGames;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DefaultGameInfo {
    pub game_id: SupportedGames,
    pub executable_name: &'static str,
    pub mods_path: &'static str, // relative path. It will be appended to the game_path to get the full path to the mods folder. Probably won't be changed ever
    pub saves_path: &'static str, // the default root should be the roaming folder (on Linux it is relative to proton's prefix, es: /home/<username>/.local/share/Steam/steamapps/compatdata/<gameid>/pfx/drive_c/users/steamuser/AppData/Roaming)
}
impl DefaultGameInfo {
    pub fn get_game_path(&self) -> Option<PathBuf> {
        let steam_dir = STEAMDIR_INSTANCE.as_ref()?;
        let (app, library) = steam_dir.find_app(self.game_id.into()).ok()??;
        Some(library.resolve_app_dir(&app))
    }

    pub fn find_by_id(game_id: SupportedGames) -> Option<&'static DefaultGameInfo> {
        SUPPORTED_GAMES.iter().find(|game| game.game_id == game_id)
    }
}

pub const DEFAULT_GAME_ID: SupportedGames = SupportedGames::Warhammer3;

pub const SUPPORTED_GAMES: [DefaultGameInfo; 1] = [DefaultGameInfo {
    game_id: SupportedGames::Warhammer3,
    executable_name: "Warhammer3.exe",
    mods_path: "data/",
    saves_path: "/The Creative Assembly/Warhammer3/save_games/",
}];
