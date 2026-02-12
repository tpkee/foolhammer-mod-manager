use crate::dto::{mods::ModResponseDto, profiles::ProfileResponseDto};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponseDto {
    pub mods: Vec<ModResponseDto>, // populated at runtime
    pub profiles: Vec<ProfileResponseDto>,
    pub game_id: String,
    pub game_path: String,
    pub saves_path: String,
    pub mods_path: String,
}
