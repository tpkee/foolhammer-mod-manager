use crate::dto::mods::ModResponseDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponseDto {
    pub id: uuid::Uuid,
    pub name: String,
    pub mods: Vec<ModResponseDto>,
}
