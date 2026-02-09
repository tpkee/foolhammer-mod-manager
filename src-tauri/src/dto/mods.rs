use crate::mods::helpers::Pack;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModResponseDto {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub order: u32,
    pub can_enable: bool,
    pub last_updated: String,
    pub is_steam_workshop: bool,
    pub image: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModRequestDto {
    pub name: String,
    pub enabled: bool,
    pub order: u32,
}
