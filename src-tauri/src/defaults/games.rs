use crate::defaults::system::STEAMDIR_INSTANCE;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SupportedGames {
    Warhammer3,
}

impl fmt::Display for SupportedGames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id: u32 = (*self).into();
        write!(f, "{}", id)
    }
}

impl From<SupportedGames> for String {
    fn from(value: SupportedGames) -> Self {
        value.to_string()
    }
}

impl From<SupportedGames> for u32 {
    fn from(value: SupportedGames) -> Self {
        match value {
            SupportedGames::Warhammer3 => 1142710,
        }
    }
}

impl TryFrom<u64> for SupportedGames {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1142710 => Ok(SupportedGames::Warhammer3),
            _ => Err(format!("Unsupported game id: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct DefaultGameInfo {
    pub game_id: SupportedGames,
    pub executable_name: &'static str,
    pub mods_path: &'static str, // relative path. It will be appended to the game_path to get the full path to the mods folder. Probably won't be changed ever
    pub saves_path: &'static str, // the default root should be the roaming folder (on Linux it is relative to proton's prefix, es: /home/<username>/.local/share/Steam/steamapps/compatdata/<gameid>/pfx/drive_c/users/steamuser/AppData/Roaming)
}
impl DefaultGameInfo {
    pub fn get_game_path(&self) -> Option<PathBuf> {
        match &*STEAMDIR_INSTANCE {
            Some(steam_dir) => match steam_dir.find_app(self.game_id.into()) {
                Ok(res) => {
                    let (app, library) = res.unwrap();
                    Some(library.resolve_app_dir(&app))
                }
                _ => None,
            },
            _ => None,
        }
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
