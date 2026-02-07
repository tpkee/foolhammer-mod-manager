use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type UserSettings = HashMap<SettingKey, serde_json::Value>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SettingKey {
    GameId,
    GamePath,
    SteamWorkshopPath,
    SavesPath,
    ModsPath,
}

impl SettingKey {
    pub fn get(&self) -> String {
        match self {
            Self::GameId => "game_id".to_string(),
            Self::GamePath => "game_path".to_string(),
            Self::SteamWorkshopPath => "steam_workshop_path".to_string(),
            Self::SavesPath => "saves_path".to_string(),
            Self::ModsPath => "mods_path".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "game_path" => Ok(Self::GamePath),
            "steam_workshop_path" => Ok(Self::SteamWorkshopPath),
            "game_id" => Ok(Self::GameId),
            "saves_path" => Ok(Self::SavesPath),
            "mods_path" => Ok(Self::ModsPath),
            _ => Err("Invalid SettingKey"),
        }
    }

    pub fn is_path_setting(&self) -> bool {
        matches!(
            self,
            Self::GamePath | Self::ModsPath | Self::SavesPath | Self::SteamWorkshopPath
        )
    }
}
