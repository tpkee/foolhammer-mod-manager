use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackResponseDto {
    pub name: String,
    pub path: PathBuf,
    pub image: Option<PathBuf>,
    pub last_updated: Option<String>,
    pub from_steam_workshop: bool,
}
