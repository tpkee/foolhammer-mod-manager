use crate::supported_games::SupportedGames;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponseDto {
    pub id: uuid::Uuid,
    pub name: String,
    pub mods: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRequestDto {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub mods: Vec<String>,
    pub game_id: SupportedGames,
}
