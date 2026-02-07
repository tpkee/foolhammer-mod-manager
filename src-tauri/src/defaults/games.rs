#[derive(Debug)]
pub struct DefaultGameInfo {
    pub game_id: &'static str,
    pub game_path: &'static str,                   // relative path
    pub steam_workshop_path: Option<&'static str>, // relative path. It shouldn't contain the gameID since we will append it when building the full, default path. None if a game doesn't have a workshop.
    pub executable_name: &'static str,
    pub mods_path: &'static str, // relative path. It will be appended to the game_path to get the full path to the mods folder. Probably won't be changed ever
    pub saves_path: &'static str, // the default root should be the roaming folder (on Linux it is relative to proton's prefix, es: /home/<username>/.local/share/Steam/steamapps/compatdata/<gameid>/pfx/drive_c/users/steamuser/AppData/Roaming)
}

pub const DEFAULT_GAME_ID: &str = "1142710";

pub const DEFAULT_GAMES_DATA: [DefaultGameInfo; 1] = [DefaultGameInfo {
    game_id: "1142710",
    game_path: "/Steam/steamapps/common/Total War WARHAMMER III/",
    steam_workshop_path: Some("/Steam/steamapps/workshop/content/"), // the game ID will be appended to this path to get the full workshop path.
    executable_name: "Warhammer3.exe",
    mods_path: "data/",
    saves_path: "/The Creative Assembly/save_games/",
}];
