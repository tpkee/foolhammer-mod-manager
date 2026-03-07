use crate::{
    dto::mods::{ModRequestDto, ModResponseDto},
    supported_games::SupportedGames,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponseDto {
    pub id: uuid::Uuid,
    pub name: String,
    pub mods: Vec<ModResponseDto>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRequestDto {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub mods: Vec<ModRequestDto>,
    pub game_id: SupportedGames,
}
