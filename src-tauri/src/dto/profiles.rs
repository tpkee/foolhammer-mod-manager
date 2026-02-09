use crate::dto::mods::ModResponseDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponseDto {
    pub name: String, // name must be unique across profiles, but not necessarily across games
    pub mods: Vec<ModResponseDto>,
    pub default: bool,
    pub manual_mode: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileRequestDto {
    pub game_id: String,
    pub name: String,
    pub default: Option<bool>,
    pub manual_mode: Option<bool>,
    pub mods: Vec<ModResponseDto>,
}
