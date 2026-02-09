use crate::dto::{mods::ModDto, profiles::ProfileDto};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDto {
    pub mods: Vec<ModDto>,
    pub profiles: Vec<ProfileDto>,
    pub game_id: String,
    pub game_path: String,
    pub saves_path: String,
    pub mods_path: String,
}
