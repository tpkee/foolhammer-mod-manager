use crate::{defaults::games::SupportedGames, stores::settings::SettingsStore};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettingsResponseDto {
    pub default_game: Option<SupportedGames>,
}

impl From<&SettingsStore> for UserSettingsResponseDto {
    fn from(settings: &SettingsStore) -> Self {
        Self {
            default_game: Some(settings.default_game),
        }
    }
}
