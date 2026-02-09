use crate::mods::helpers::Pack;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModDto {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub can_enable: bool,
    pub last_updated: String,
    pub is_steam_workshop: bool,
    pub image: Option<String>,
}
