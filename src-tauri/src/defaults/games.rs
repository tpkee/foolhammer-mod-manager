pub struct DefaultGameInfo {
    pub game_id: &'static str,
    pub game_path: &'static str,
    pub steam_workshop_path: Option<&'static str>,
    pub executable_name: &'static str,
}

pub const DEFAULT_GAMES_DATA: [DefaultGameInfo; 1] = [DefaultGameInfo {
    game_id: "1142710",
    game_path: "/Steam/steamapps/common/Total War WARHAMMER III",
    steam_workshop_path: Some("/Steam/steamapps/workshop/content/1142710"),
    executable_name: "Warhammer3.exe",
}];

pub const DEFAULT_GAME_ID: &str = "1142710";
