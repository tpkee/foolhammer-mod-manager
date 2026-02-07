#[derive(Debug)]
pub struct DefaultGameInfo {
    pub game_id: &'static str,
    pub game_path: &'static str,                   // relative path
    pub steam_workshop_path: Option<&'static str>, // relative path. It shouldn't contain the gameID since we will append it when building the full, default path. None if a game doesn't have a workshop.
    pub executable_name: &'static str,
}

pub const DEFAULT_GAMES_DATA: [DefaultGameInfo; 1] = [DefaultGameInfo {
    game_id: "1142710",
    game_path: "/Steam/steamapps/common/Total War WARHAMMER III/",
    steam_workshop_path: Some("/Steam/steamapps/workshop/content/"), // the game ID will be appended to this path to get the full workshop path.
    executable_name: "Warhammer3.exe",
}];

pub const DEFAULT_GAME_ID: &str = "1142710";
