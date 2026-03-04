use crate::state::user_settings::{SettingKey, UserSettings};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettingsResponseDto {
    pub default_game: Option<String>,
}

impl From<&UserSettings> for UserSettingsResponseDto {
    fn from(settings: &UserSettings) -> Self {
        let get_str = |key: &SettingKey| -> Option<String> {
            settings
                .get(key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        };

        Self {
            default_game: get_str(&SettingKey::GameId),
        }
    }
}
