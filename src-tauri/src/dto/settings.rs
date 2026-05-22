use std::path::PathBuf;

use crate::{stores::settings::SettingsStore, supported_games::SupportedGames};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettingsResponseDto {
    pub default_game: Option<SupportedGames>,
    pub steam_path: Option<PathBuf>,
    pub steam_library_path: Option<PathBuf>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserSettingsDto {
    pub steam_path: Option<PathBuf>,
    pub steam_library_path: Option<PathBuf>,
}

impl From<&SettingsStore> for UserSettingsResponseDto {
    fn from(settings: &SettingsStore) -> Self {
        Self {
            default_game: Some(settings.default_game),
            steam_path: settings.steam_path.clone(),
            steam_library_path: settings.steam_library_path.clone(),
        }
    }
}
