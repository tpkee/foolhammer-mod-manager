use crate::dto::mods::ModDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDto {
    pub name: String, // name must be unique across profiles, but not necessarily across games
    pub mods: Vec<ModDto>,
}
